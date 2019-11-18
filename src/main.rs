#![feature(const_fn)]

mod collidables;
mod obj;
mod render;
mod chunk;
mod cam;
mod ray;
mod world;
mod utils;

use render::{Renderer, RenderSettings};
use std::time::{Duration, Instant};
use minifb::{Window, WindowOptions, Key};
use std::sync::{Arc, RwLock};
use simplelog::*;
use log::{info, debug, trace, warn};
use cgmath::prelude::*;
use cgmath::Vector3;

const CHUNK_SIZE : usize = 128;
const WIDTH : usize = 1080;
const HEIGHT : usize = 1080;


fn main() {

    let rset = RenderSettings {
        chunk_size: CHUNK_SIZE,
        width: WIDTH,
        height: HEIGHT
    };

    SimpleLogger::init(LevelFilter::Info, ConfigBuilder::new().set_time_format_str("%H:%M:%S%.6f").build()).unwrap();
    info!("Starting RustRay");
    //start measure
    
    
    
    let mut window = Window::new("RustRay", rset.width, rset.height, WindowOptions::default()).expect("Failed to create window");
    debug!("Created Window");
    //Create objects



    let mut buffer = vec![0u32; rset.width * rset.height];
    debug!("Created 0ed buffer");

    let cam = cam::Camera::new((8.0, 0.0, 1.0).into(), (-0.15, 0.0, 0.0).into(), (0.1 * WIDTH as f64 / HEIGHT as f64, 0.1), (rset.width, rset.height));

    let mut wrld = world::World {
        camera: cam,
        objects: Vec::new(),
        lights: Vec::new(),
        sky: world::Sky::new(
            Vector3::from((-0.2, 0.3, -0.4)).normalize(),   // sun dir
            (3.0, 1.0, 2.0).into(),           // sun color
            (0.4, 0.4, 0.8).into(),             // ambient color
            (0.35, 0.25, 0.25).into()              // ground color
        )
    };

    world::populate_test(&mut wrld);
    let wrld = Arc::new(RwLock::new(wrld));

    let mut renderer = Renderer::new(rset.width, rset.height, rset.chunk_size);

    let time_render = Instant::now();
    let mut last_draw = Instant::now();
    let mut chunks_since_draw = 0;


    renderer.start_render(4, rset, wrld);

    while let Ok(chnk) = renderer.get_next_finished() {
        trace!("Received chunk from renderer, writing to buffer and updating screen");
        chnk.copy_to_base(WIDTH, &mut buffer);
        chunks_since_draw += 1;
        if last_draw.elapsed().as_millis() > 30 {
            window.update_with_buffer(&buffer).expect("update_with_buffer failed");
            window.update();
            debug!("Updating screen with {} new chunks drawn", chunks_since_draw);
            chunks_since_draw = 0;
            last_draw = Instant::now();

            if !window.is_open() || window.is_key_down(Key::Escape) {
                warn!("Program terminated while rendering. Killing workers");
                renderer.kill();
                info!("All workers terminated");
                return;
            }
        }
    }

    // needed for reset
    renderer.reset();
    if chunks_since_draw > 0 {
        debug!("Finisheing screen with {} new chunks drawn", chunks_since_draw);
        window.update_with_buffer(&buffer).expect("update_with_buffer failed");
    }
    let elpsd = time_render.elapsed();
    info!("Time for render and displaying: {}s", elpsd.as_secs_f64());

        
    

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
        std::thread::sleep(Duration::from_millis(30));
    }
}



