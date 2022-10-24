pub mod point_light;

use crate::{spectrum::Spectrum, geometry::interaction::SurfaceInteraction};
use cgmath::{Point2, Point3};

pub trait Light {
    fn sample_li(&self, isect: &SurfaceInteraction, sample: Point2<f64>) -> (Spectrum, Point3<f64>, f64);
}