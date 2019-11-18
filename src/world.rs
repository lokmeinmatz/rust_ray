use crate::cam::Camera;
use crate::collidables::{*, primitives::*};
use crate::ray::Ray;
use cgmath::{Vector3, prelude::*};
use crate::collidables::Color;
use crate::utils::*;
use rand::prelude::*;

pub fn populate_test(world: &mut World) {
  world.objects.push(Box::new(Circle::new(
    (0.0, 0.0, 1.0).into(), 
    1.5, 
    Material::diffuse(colors::RED)
  )));
  world.objects.push(Box::new(Circle::new(
    (0.0, 3.0, 0.5).into(), 
    1.0, 
    Material::mirror(colors::WHITE)
  )));
  world.objects.push(Box::new(Circle::new(
    (0.0, -3.0, 1.5).into(), 
    1.0, 
    Material::specular(colors::BLACK)
  )));
  world.objects.push(Box::new(Plane::new(
    (0.0, 0.0, 0.0).into(), 
    (0.0, 0.0, 1.0).into(), 
    Material::diffuse(colors::GREEN)
  )));


  world.lights.push(Light::DirectionalLight{ dir: Vector3::new(-0.1, 0.3, -0.4).normalize(), color: (100.0, 200.0, 210.0).into() });
}

pub struct World {
  pub camera : Camera,
  pub objects: Vec<Box<dyn Collidable + Send + Sync>>,
  pub lights: Vec<Light>,
  pub sky: Sky
}

pub struct Sky {
  pub sun_dir: Vector3<f64>,
  pub sun_color: Color,
  pub ambient_color: Color,
  pub ground_color: Color
}

impl Sky {
  pub fn new(sun_dir: Vector3<f64>, sun_color: Color, ambient_color: Color, ground_color: Color) -> Sky {
    Sky {
      sun_dir,
      sun_color,
      ambient_color,
      ground_color
    }
  }

  pub fn sample_sky(&self, dir: &Vector3<f64>) -> Color {
      let to_sun = dir.dot(self.sun_dir).mul_add(0.5, 0.5).powi(16);

      // a squished sigmoid function to lerp between ground and sky
      let ambi_color = self.ambient_color.lerp(self.ground_color, 1.0 / (1.0 + (-40.0 * dir.z as f32).exp()));
      ambi_color + self.sun_color * to_sun as f32
  }
}


const MAX_DIST : f64 = 200_000.0;

impl World {
  pub fn trace_ray(&self, ray: &mut Ray) -> Collision {
    
    // Default to sky data
    let mut color = self.sky.sample_sky(&ray.dir);
    let mut i_dist = MAX_DIST;
    let mut normal = -ray.dir;

    if ray.bounces_remaining > 1 {
      for obj in &self.objects {
        if let Some(col) = obj.intersect(ray) {
          if col.distance < i_dist {

            // TODO randomize diffuse dir based on specular value
            let diffuse = self.trace_ray(&mut Ray::new(col.poi, col.normal, ray.bounces_remaining - 1, ray.rng));

            // TODO randomize reflection dir based on specular value
            let reflection = self.trace_ray(&mut Ray::new(col.poi, reflect(ray.dir, col.normal), ray.bounces_remaining - 1, ray.rng));

            color = col.mat.base_color.mul_element_wise(diffuse.col.lerp(reflection.col, col.mat.metallic));
            //color = col.color * 1.0;
            i_dist = col.distance;
            normal = col.normal;
          }
        }
      }
    }

    Collision {
      col: color,
      distance: i_dist,
      normal
    }

  }
}