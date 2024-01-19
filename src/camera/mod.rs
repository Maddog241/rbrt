pub mod film;
pub mod perspective;
pub mod pixel;

use super::geometry::ray::Ray;
use cgmath::Point2;

pub trait Camera {
    fn generate_ray(&self, sample: CameraSample) -> Ray;
}

pub struct CameraSample {
    p_film: Point2<f64>, // point on the film, in raster coordinate system, [0, resolution.x] x [0, resolution.y]
    time: f64,
}

impl CameraSample {
    pub fn new(p_film: Point2<f64>, time: f64) -> Self {
        CameraSample { p_film, time }
    }
}
