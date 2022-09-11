use crate::spectrum::Spectrum;
use cgmath::{Point2, Vector3};

pub trait Bxdf {
    fn f(wi: &Vector3<f64>, wo: &Vector3<f64>) -> Spectrum;
    fn sample_f(wi: &Vector3<f64>, wo: &Vector3<f64>, sample: Point2<f64>, pdf: &mut f64) -> Spectrum;
}