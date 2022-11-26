pub mod lambertian;
pub mod bsdf;
pub mod fresnel;
pub mod microfacet;

use std::ops::{BitOr, BitAnd};

use crate::spectrum::Spectrum;
use cgmath::{Point2, Vector3};

pub trait Bxdf {
    fn f(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> Spectrum;
    fn sample_f(&self, wo: Vector3<f64>, sample: Point2<f64>) -> (Spectrum, Vector3<f64>, f64);
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

impl BitOr<BxdfType> for BxdfType {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as i32) | (rhs as i32)
    }
}

impl BitOr<i32> for BxdfType {
    type Output = i32;

    fn bitor(self, rhs: i32) -> Self::Output {
        (self as i32) | rhs
    }
}

impl BitOr<BxdfType> for i32 {
    type Output = i32;

    fn bitor(self, rhs: BxdfType) -> Self::Output {
        self | rhs as i32
    }
}

impl BitAnd<BxdfType> for BxdfType {
    type Output = i32;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self as i32) & (rhs as i32)
    }
}

impl BitAnd<i32> for BxdfType {
    type Output = i32;

    fn bitand(self, rhs: i32) -> Self::Output {
        (self as i32) & rhs
    }
}

impl BitAnd<BxdfType> for i32 {
    type Output = i32;

    fn bitand(self, rhs: BxdfType) -> Self::Output {
        self & (rhs as i32)
    }
}

