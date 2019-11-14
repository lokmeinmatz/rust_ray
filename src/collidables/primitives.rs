use cgmath::prelude::*;
use cgmath::Vector3;
use super::{Collidable, Collision};
use crate::ray::Ray;

pub struct Circle {
    center: Vector3<f64>,
    radius: f64,
    color: Vector3<f64>
}

impl Circle {
    pub fn new(center: Vector3<f64>, radius: f64, color: Vector3<f64>) -> Circle {
        Circle {
            center,
            radius,
            color
        }
    }
}

impl Collidable for Circle {
    fn intersect(&self, ray: &Ray) -> Option<Collision> {
        // based on https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection

        let l = self.center - ray.origin; // points towars origin
        
        let t_center = l.dot(ray.dir);

        if t_center < 0.0 { return None }

        let d = (l.dot(l) - t_center * t_center).sqrt(); // perpendicular length from origin + t_center * dir to center

        if d < 0.0 || d > self.radius { return None }

        let t_hc = (self.radius * self.radius - d * d).sqrt();

        let mut t0 = t_center - t_hc;
        let t1 = t_center + t_hc;

        if t0 < 0.0 || t1 < 0.0 { return None }

        //if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }
        if t0 > t1 { t0 = t1; }

        let poi = ray.origin + ray.dir * t0;

        Some(Collision {
            distance: t0,
            color: self.color,
            normal: poi - self.center, // TODO calc normal
            poi
        })
    }
}