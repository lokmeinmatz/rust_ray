pub mod colors {
    use crate::collidables::Color;
    use super::rgb_to_col;
    pub const RED: Color = rgb_to_col(255, 0, 0);
    pub const GREEN: Color = rgb_to_col(0, 255, 0);
    pub const BLUE: Color = rgb_to_col(0, 0, 255);
    pub const WHITE: Color = rgb_to_col(255, 255, 255);
    pub const BLACK: Color = rgb_to_col(0, 0, 0);
    pub const GRAY: Color = rgb_to_col(128, 128, 128);
}



use num_traits::{Num, AsPrimitive, FromPrimitive};
use crate::collidables::Color;
use cgmath::{Vector3, InnerSpace};

#[inline]
pub const fn rgb_to_col(r: u32, g: u32, b: u32) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub fn reflect(ray: Vector3<f64>, normal:Vector3<f64>) -> Vector3<f64> {
    ray - 2.0 * (ray.dot(normal)) * normal
}


#[inline]
pub fn rgb_to_u32<T: AsPrimitive<u32>>(r: T, g: T, b: T) -> u32 {
    let r = clamp(r.as_(), 0, 255);
    let g = clamp(g.as_(), 0, 255);
    let b = clamp(b.as_(), 0, 255);
    (r & 0xFF) << 16 | (g & 0xFF) << 8 | (b& 0xFF)
}


/// Converts u32 Color to generic (r, g, b) tuple, each between 0 and 255
#[inline]
pub fn u32_to_rgb<T: FromPrimitive>(col: u32) -> (T, T, T) {
    let r = col >> 16 & 0xFF;
    let g = col >> 8  & 0xFF;
    let b = col       & 0xFF;
    (T::from_u32(r).unwrap(), T::from_u32(g).unwrap(), T::from_u32(b).unwrap())
}

#[inline]
pub fn clamp<T : Num + std::cmp::PartialOrd>(value : T, min : T, max : T) -> T {

    if value < min {min}
    else if value > max {max}
    else{value}
}
use rand::prelude::*;

pub fn ray_in_hemisphere(rng: &mut ThreadRng, normal: &Vector3<f64>) -> Vector3<f64> {
    let theta: f64 = rng.gen();
}


#[test]
fn test_colors() {
    assert_eq!(rgb_to_u32(255,  0, 128), 0x00ff_0080);
    assert_eq!(rgb_to_u32(  0, 32,   0), 0x0000_2000);
    assert_eq!(u32_to_rgb(0x00f0_f0aa), (240f64, 240f64, 170.0f64));
    assert_eq!(u32_to_rgb(0x00ee_20aa), (238f64, 32f64, 170.0f64));
}