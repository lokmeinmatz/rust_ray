use std::sync::{
    Arc, 
    Mutex, 
    RwLock,
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, sync_channel}};

use std::thread::{self, JoinHandle};
use log::{error, info, trace, debug};
use crate::world::World;
use crate::chunk::Chunk;

#[derive(Clone, Copy)]
pub struct RenderSettings {
    pub chunk_size: usize,
    pub width: usize,
    pub height: usize
}

pub struct Renderer {
    chunk_coords: Arc<Mutex<Vec<(u32, u32)>>>,
    running: Arc<AtomicBool>,
    workers: Vec<JoinHandle<()>>,
    finished_queue: Option<Receiver<Chunk>>,
    size: (usize, usize),
    chunk_size: usize
}


impl Renderer {
    pub fn new(w: usize, h: usize, chunk_size: usize) -> Renderer {

        let mut c_coords = Vec::new();
        //split into smaller chunks for mulithreading
        let mut x = 0;
        while x < w {

            let mut y = 0;
            while y < h {

                c_coords.push((x as u32, y as u32));
                

                //println!("Created new Chunk at {0} {1}", x, y);

                y += chunk_size;
            }



            x += chunk_size;
        }

        debug!("Chunks to process: {}", c_coords.len());

        Renderer {
            chunk_coords: Arc::new(Mutex::new(c_coords)),
            running: Arc::from(AtomicBool::from(false)),
            workers: Vec::new(),
            finished_queue: None,
            size: (w, h),
            chunk_size
        }
    }

    pub fn reset(&mut self) {

        for worker in self.workers.drain(..) {
            if worker.join().is_err() {
                error!("Failed to join a worker");
            }
        }

        self.running.store(false, Ordering::Relaxed);

        
        let mut c_coords = self.chunk_coords.lock().unwrap();
        let mut x = 0;
        let (w, h) = self.size;
        while x < w {
            let mut y = 0;
            while y < h {
                c_coords.push((x as u32, y as u32));
                //println!("Created new Chunk at {0} {1}", x, y);
                y += self.chunk_size;
            }
            x += self.chunk_size;
        }
    }

    pub fn get_next_finished(&mut self) -> Result<Chunk, ()> {
        match &self.finished_queue {
            Some(queue) => {
                queue.recv().map_err(|_| ())
            },
            None => Err(())
        }
    }

    pub fn start_render(&mut self, worker_count: usize, settings: RenderSettings, world: Arc<RwLock<World>>) {

        assert!(self.workers.is_empty());

        self.running.store(true, Ordering::Release);

        let (tx, rx) = sync_channel(30);
        self.finished_queue = Some(rx);
        
        for i in 0..worker_count {

            let running = self.running.clone();
            let chunk_coords = self.chunk_coords.clone();
            let world = world.clone();
            let tx = tx.clone();

            match thread::Builder::new().name(format!("Worker #{}", i)).spawn(move || {
                // Worker Thread start
                debug!("Worker #{} active", i);
                
                let mut rng = rand::prelude::thread_rng();

                while running.load(Ordering::Relaxed) {
                    let (x, y) = match chunk_coords.try_lock() {
                        Ok(mut coords) => {
                            match coords.pop() {
                                Some(c) => c,
                                None => {
                                    trace!("No coords in queue");
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            //error!("Error while locking chunk_coords");
                            continue
                        }
                    };
                    
                    let mut chunk = Chunk::new_clamped(x as usize, y as usize, settings.chunk_size, settings.width, settings.height).expect("Failed to create Chunk");
                    chunk.render(&world.read().unwrap(), &mut rng);
                    if tx.send(chunk).is_err() {
                        error!("Failed to send chunk back");
                    }
                }

                debug!("Worker #{} shut down", i);
                // Worker Thread end
            }) {
                Ok(t) => self.workers.push(t),
                Err(_) => error!("Failed to start worker #{}", i)
            }
        }
    }

    pub fn kill(&mut self) {
        info!("Killing renderer...");
        self.running.store(false, Ordering::Release);

        // drain not processed chunks
        if let Some(queue) = &self.finished_queue {
            while let Ok(_) = queue.recv() {}
        }
        for worker in self.workers.drain(..) {
            if worker.join().is_err() {
                error!("Failed to join a worker");
            }
        }
    }
}