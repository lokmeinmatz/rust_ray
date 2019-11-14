use crate::ray::Ray;
use cgmath::Vector3;

pub mod primitives;

pub struct Collision {
    pub distance : f64,
    pub poi : Vector3<f64>,
    pub normal : Vector3<f64>,
    pub color: Vector3<f64>
}

pub trait Collidable {
  fn intersect(&self, ray : &Ray) -> Option<Collision>;
}
