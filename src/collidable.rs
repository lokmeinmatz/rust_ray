use crate::ray::Ray;
use cgmath::Vector3;

pub struct Collision {
    pub t : f32,
    pub poi : Vector3<f64>,
    pub normal : Vector3<f64>
}

pub trait Collidable {
  fn intersect(&self, ray : &Ray) -> Collision;
}
