pub mod point_light;
pub mod area_light;

use crate::{spectrum::Spectrum, geometry::{interaction::SurfaceInteraction, ray::Ray}};
use cgmath::{Point2, Point3, Vector3};

pub trait Light {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2<f64>) -> (Spectrum, Point3<f64>, f64);
    fn le(&self, n: Vector3<f64>, d: Vector3<f64>) -> Spectrum;
    fn intersect_p(&self, r: &Ray) -> Option<f64>;
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction>;
}