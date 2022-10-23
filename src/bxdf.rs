pub mod lambertian;
pub mod bsdf;


use std::ops::{BitOr, BitAnd};

use crate::spectrum::Spectrum;
use cgmath::{Point2, Vector3};

pub trait Bxdf {
    fn f(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> Spectrum;
    fn sample_f(&self, wo: Vector3<f64>, sample: Point2<f64>) -> (Spectrum, Vector3<f64>, f64); // f-value, wi, pdf
    fn types(&self) -> i32;
}

pub enum BxdfType {
    Reflection = 1,
    Transmission = 2,
    Specular = 4,
    Diffuse = 8,
    Glossy = 16,
    All = 31,
}

impl BitOr for BxdfType {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as i32) | (rhs as i32)
    }
}

impl BitAnd for BxdfType {
    type Output = i32;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self as i32) & (rhs as i32)
    }
}

pub fn match_flags(bxdf_obj: &Box<dyn Bxdf>, flags: i32) -> bool {
    // test if the bxdf_obj matches all the requirements represented by flags
    (bxdf_obj.types() & flags) == bxdf_obj.types()
}