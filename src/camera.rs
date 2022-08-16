pub mod film;
pub mod perspective;

use cgmath::Point3;
use super::geometry::ray::Ray;

pub trait Camera {
    fn generate_ray(&self, sample: CameraSample) -> Ray;
}

pub struct CameraSample {
    p_film: Point3<f64>, // point on the film, in raster coordinate system, [0, resolution.x] x [0, resolution.y]
    time: f64,
}
