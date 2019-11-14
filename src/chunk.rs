use num_traits::{AsPrimitive, Num};
use std;
use crate::world::World;
use crate::utils;

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

  fn idx<T: AsPrimitive<usize>>(&self, x: T, y: T) -> usize {
    x.as_() + y.as_() * self.width
  }

  pub fn render(&mut self, world: &World) {
        //dx / dy : relative positions of ray in chunk
        //x / y : absolute pixel positions
        //println!("Rendering chunk at {}, {}", self.x, self.y);

        for (dx, x) in (self.x..self.x+self.width).enumerate() {
            for (dy, y) in (self.y..self.y+self.height).enumerate() {

                  let ray = world.camera.create_ray(x, y);

                  let color = world.trace_ray(&ray);

                  let idx = self.idx(dx, dy);
                  self.data[idx] = utils::rgb_to_col(color.x * 255.0, color.y * 255.0, color.z * 255.0);
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
