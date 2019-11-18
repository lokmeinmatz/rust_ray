use cgmath::prelude::*;
use cgmath::Vector3;
use super::*;
use crate::ray::Ray;

pub struct Circle {
    center: Vector3<f64>,
    radius: f64,
    mat: Material
}

impl Circle {
    pub fn new(center: Vector3<f64>, radius: f64, mat: Material) -> Circle {
        Circle {
            center,
            radius,
            mat
        }
    }
}

impl Collidable for Circle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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

        Some(Intersection {
            distance: t0,
            mat: self.mat,
            normal: (poi - self.center).normalize(), // TODO calc normal
            poi
        })
    }
}


pub struct Plane {
    position: Vector3<f64>,
    normal: Vector3<f64>,
    mat: Material
}


impl Plane {
    pub fn new(position: Vector3<f64>, normal: Vector3<f64>, mat: Material) -> Plane {
        Plane {
            position,
            normal: normal.normalize(),
            mat
        }
    }
}

impl Collidable for Plane {
    fn intersect(&self, ray : &Ray) -> Option<Intersection> {
        let divisor = ray.dir.dot(self.normal);

        if divisor.abs() < 0.001 { return None; }

        let dividend = (ray.origin - self.position).dot(self.normal);

        let t = dividend / divisor;
        
        if t <= 0.0 { return None; }

        Some(Intersection {
            distance: t,
            mat: self.mat,
            normal: self.normal,
            poi: ray.origin + ray.dir * t
        })
    }
}