use cgmath::Vector3;
use cgmath::prelude::*;
use crate::ray::Ray;


#[derive(Clone, Copy)]
pub struct Camera {
    /// Position of the camera sensor
    pub position : Vector3<f64>,

    /// Distance and direction of the aperture, maybe make direction and distance independent?
    pub direction : Vector3<f64>,

    /// (width / 2, height / 2) of the sensor
    sensor_size: (f64, f64),

    /// Sensor resolution in pixels
    sensor_resolution: (usize, usize),

    /// [Unused] for future variable aperture
    aperture: f64
}

impl Camera {

    pub fn new(position : Vector3<f64>, direction : Vector3<f64>, sensor_size: (f64, f64), resolution: (usize, usize)) -> Camera {
       
      Camera {
        position,
        direction,
        sensor_size,
        sensor_resolution: resolution,
        aperture: 0.0
      }
    }

    /// Creates an ray, casted from the pixel's position on the sensor through the aperture hole 
    /// perc x and y must be between -1 and 1, indicating the position on the sensor / image
    pub fn create_ray(&self, pixel_x: usize, pixel_y: usize) -> Ray {
      

      let perc_x = (pixel_x as f64 / self.sensor_resolution.0 as f64) * 2.0 - 1.0;
      let perc_y = (pixel_y as f64 / self.sensor_resolution.1 as f64) * 2.0 - 1.0;

      assert!(perc_x >= -1.0 && perc_x <= 1.0 && perc_y >= -1.0 && perc_y <= 1.0);

      // get Pixels location on sensor

      // unit to left dir
      let left = Vector3::unit_z().cross(self.direction.normalize()).normalize();
      let up = self.direction.normalize().cross(left.clone()).normalize();

      let pixel_pos = self.position + (left * -perc_x * self.sensor_size.0) + (up * -perc_y * self.sensor_size.1);

      Ray::new(pixel_pos, ((self.position + self.direction) - pixel_pos).normalize())
    }
}


#[test]
fn test_cam_ray() {
    let cam = Camera::new((-1.0, 0.0, 0.0).into(), (0.1, 0.0, 0.0).into(), (0.1, 0.1), (10, 10));

    let mid = cam.create_ray(5, 5);

    assert_eq!(mid.origin, cam.position);
    assert_eq!(mid.dir, (1.0, 0.0, 0.0).into());
    
    // TODO write better tests
    let top_left = cam.create_ray(10, 10);
    
    assert_eq!(top_left.origin, cam.position + Vector3::from((0f64, -0.1f64, -0.1f64)));

    let target_dir = Vector3::from((1f64, 1f64, 1f64)).normalize();

    assert!((top_left.dir.x - target_dir.x).abs() < 0.001, "dir.x not as expected");
    assert!((top_left.dir.y - target_dir.y).abs() < 0.001, "dir.y not as expected");
    assert!((top_left.dir.z - target_dir.z).abs() < 0.001, "dir.z not as expected");
    
}