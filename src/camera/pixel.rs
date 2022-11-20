use cgmath::Vector3;
use image::Rgb;

#[derive(Clone)]
pub struct Pixel {
    pub rgb: Vector3<f64>,
}

impl Pixel {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        // allow x, y, z to be greater than 1.0 for HDR
        assert!(0.0 <= x); 
        assert!(0.0 <= y);
        assert!(0.0 <= z);
        Pixel {
            rgb: Vector3::new(x, y, z),
        }
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        let x = (self.rgb.x * 255.0) as u8;
        let y = (self.rgb.y * 255.0) as u8;
        let z = (self.rgb.z * 255.0) as u8;

        Rgb([x, y, z])
    }
}
