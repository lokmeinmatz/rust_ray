use crate::cam::Camera;
use crate::collidables::{Collidable, primitives::*};
use crate::ray::Ray;
use cgmath::{Vector3, prelude::*};

type Color = Vector3<f64>;

pub fn populate_test(world: &mut World) {
  world.objects.push(Box::new(Circle::new((0.0, 0.0, 1.0).into(), 1.0, (1.0, 0.0, 0.0).into())));
}

pub struct World {
  pub camera : Camera,
  pub sky_color: Color,
  pub objects: Vec<Box<dyn Collidable + Send + Sync>>
}


impl World {
  pub fn trace_ray(&self, ray: &Ray) -> Color {
    let light : Vector3<f64> = Vector3::new(0.1, 0.3, -0.2).normalize();
    let mut color = self.sky_color;
    let mut i_dist = std::f64::INFINITY;

    for obj in &self.objects {
      if let Some(col) = obj.intersect(ray) {
        if col.distance < i_dist {
          color = col.color * (col.normal.dot(light) / 2.0 + 0.5);
          i_dist = col.distance;
        }
      }
    }

    color
  }
}