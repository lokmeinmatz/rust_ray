use num_traits::{ToPrimitive, Num};
use std;
use crate::cam::Camera;
use log::trace;
use cgmath::prelude::*;

fn clamp<T : Num + std::cmp::PartialOrd>(value : T, min : T, max : T) -> T {

    if value < min {min}
    else if value > max {max}
    else{value}
}

pub struct Chunk {
    x      : usize,
    y      : usize,
    width  : usize,
    height : usize,
    pub data : Vec<u32>,
    
}

impl Chunk {

  fn new(x : usize, y: usize, width: usize, height: usize) -> Chunk {
    Chunk {x, y, data: vec![0; width * height], width, height}
  }

  pub fn new_clamped(x: usize, y: usize, chunk_size: usize, screen_width: usize, screen_height: usize) -> Result<Chunk, ()> {
    if x >= screen_width || y >= screen_height {
        return Err(());
    }
    Ok(Self::new(x, y, chunk_size.min(screen_width - x), chunk_size.min(screen_height - y)))
  }

  fn idx<T: ToPrimitive>(&self, x: T, y: T) -> usize {
    x.to_usize().unwrap() + y.to_usize().unwrap() * self.width
  }

  pub fn render(&mut self, cam: &Camera) {
        //dx / dy : relative positions of ray in chunk
        //x / y : absolute pixel positions
        //println!("Rendering chunk at {}, {}", self.x, self.y);

        for (dx, x) in (self.x..self.x+self.width).enumerate() {
            for (dy, y) in (self.y..self.y+self.height).enumerate() {

                  let ray = cam.create_ray(x, y);

                  if dx + dy == 0 {
                    //trace!("Ray dir: {:?}", ray.dir);
                  }

                  let r = 200;
                  let g = 100;
                  let b = ray.dir.dot(cam.direction.normalize()) * 255.0;
                  let idx = self.idx(dx, dy);
                  self.data[idx] = (r as u32 & 0xFF) << 16 | (g as u32 & 0xFF) << 8 | (b as u32 & 0xFF);
                
            }
        }
    }

    pub fn copy_to_base(&self, line_width: usize, base: &mut Vec<u32>) {
        for local_y in 0..self.height {
            let start_base_idx = (local_y + self.y) * line_width + self.x;
            base[start_base_idx..start_base_idx + self.width].copy_from_slice(&self.data[local_y * self.width..local_y * self.width + self.width]);
        }
    }
}
