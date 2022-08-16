use cgmath::Vector3;

#[derive(Clone)]
pub struct Pixel {
    pub rgb: Vector3<f64>,
}

impl Pixel {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pixel {
            rgb: Vector3::new(x, y, z),
        }
    }

    pub fn to_u8(&self) -> [u8; 3] {
        let x = (self.rgb.x * 255.0) as u8;
        let y = (self.rgb.y * 255.0) as u8;
        let z = (self.rgb.z * 255.0) as u8;
        [x, y, z]
    }
}
