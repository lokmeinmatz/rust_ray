use crate::ray::Ray;
use cgmath::Vector3;

pub mod primitives;

pub type Color = Vector3<f32>;


pub enum Light {
  DirectionalLight { dir: Vector3<f64>, color: Color }
}


#[derive(Copy, Clone)]
pub struct Material {
  pub base_color : Color,
  pub metallic: f32,
  pub roughness: f64
}

impl Material {
  pub fn diffuse(col: Color) -> Material {
    Material {
      base_color: col,
      metallic: 0.0,
      roughness: 1.0
    }
  }

  pub fn mirror(col: Color) -> Material {
    Material {
      base_color: col,
      metallic: 1.0,
      roughness: 0.0
    }
  }

  pub fn specular(col: Color) -> Material {
    Material {
      base_color: col,
      metallic: 0.0,
      roughness: 0.0
    }
  }
}

pub struct Collision {
    pub distance : f64,
    pub normal : Vector3<f64>,
    pub col: Color
}

pub struct Intersection {
    pub distance : f64,
    pub poi : Vector3<f64>,
    pub normal : Vector3<f64>,
    pub mat: Material
}

pub trait Collidable {
  fn intersect(&self, ray : &Ray) -> Option<Intersection>;
}
