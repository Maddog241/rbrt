pub mod point_light;

use crate::{spectrum::Spectrum, geometry::interaction::SurfaceInteraction};
use cgmath::{Point2, Vector3};

pub trait Light {
    fn sample_li(&self, isect: &SurfaceInteraction, sample: Point2<f64>, wi: &mut Vector3<f64>, pdf: &mut f64) -> Spectrum;
}