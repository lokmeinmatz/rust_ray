use num_traits::{AsPrimitive, FromPrimitive};

pub fn rgb_to_col<T: AsPrimitive<u32>>(r: T, g: T, b: T) -> u32 {
    (r.as_() & 0xFF) << 16 | (g.as_() as u32 & 0xFF) << 8 | (b.as_() as u32 & 0xFF)
}


/// Converts u32 Color to generic (r, g, b) tuple, each between 0 and 255
pub fn col_to_rgb<T: FromPrimitive>(col: u32) -> (T, T, T) {
    let r = col >> 16 & 0xFF;
    let g = col >> 8  & 0xFF;
    let b = col       & 0xFF;
    (T::from_u32(r).unwrap(), T::from_u32(g).unwrap(), T::from_u32(b).unwrap())
}


#[test]
fn test_colors() {
    assert_eq!(rgb_to_col(255,  0, 128), 0x00ff_0080);
    assert_eq!(rgb_to_col(  0, 32,   0), 0x0000_2000);
    assert_eq!(col_to_rgb(0x00f0_f0aa), (240f64, 240f64, 170.0f64));
    assert_eq!(col_to_rgb(0x00ee_20aa), (238f64, 32f64, 170.0f64));
}