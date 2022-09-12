pub mod lambertian;


use crate::spectrum::Spectrum;
use cgmath::{Point2, Vector3};

pub trait Bxdf {
    fn f(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Spectrum;
    fn sample_f(&self, wo: &Vector3<f64>, wi: &mut Vector3<f64>, sample: Point2<f64>, pdf: &mut f64) -> Spectrum;
}