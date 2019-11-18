use cgmath::Vector3;
use rand::prelude::ThreadRng;


pub struct Ray<'a> {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub bounces_remaining: usize,
    pub rng: &'a mut ThreadRng
}


impl <'a> Ray<'a> {
    pub fn new(origin: Vector3<f64>, dir: Vector3<f64>, bounces_remaining: usize, rng: &'a mut ThreadRng) -> Ray<'a> {
        Ray {
            origin,
            dir,
            bounces_remaining,
            rng
        }
    }
}