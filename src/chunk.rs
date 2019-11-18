use num_traits::{AsPrimitive, Num};
use crate::world::World;
use crate::utils;
use crate::collidables::Color;
use rand::prelude::ThreadRng;

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

  fn idx<T: AsPrimitive<usize>>(&self, x: T, y: T) -> usize {
    x.as_() + y.as_() * self.width
  }

  pub fn render(&mut self, world: &World, rng: &mut ThreadRng) {
        //dx / dy : relative positions of ray in chunk
        //x / y : absolute pixel positions
        //println!("Rendering chunk at {}, {}", self.x, self.y);
        for (dx, x) in (self.x..self.x+self.width).enumerate() {
            for (dy, y) in (self.y..self.y+self.height).enumerate() {

                  let mut ray = world.camera.create_ray(x, y, rng);

                  let trace = world.trace_ray(&mut ray);
                  let color : Color = trace.col;
                  //let color : Color= (trace.distance as f32 / 10.0, trace.distance as f32 / 10.0, trace.distance as f32 / 10.0).into();
                  //let color = trace.normal.map(|e| e.mul_add(0.5, 0.5));
                  let idx = self.idx(dx, dy);
                  self.data[idx] = utils::rgb_to_u32(color.x * 255.0, color.y * 255.0, color.z * 255.0);
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
