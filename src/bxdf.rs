pub mod lambertian;
pub mod bsdf;
pub mod fresnel;
pub mod microfacet;
// pub mod fresnel_blend;

use std::{ops::{BitOr, BitAnd}, f64::consts::PI};

use crate::spectrum::Spectrum;
use cgmath::{Point2, Vector3};

const INV_PI: f64 = 1.0 / PI;

pub trait Bxdf {
    fn f(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> Spectrum;
    fn sample_f(&self, wo: Vector3<f64>, sample: Point2<f64>) -> (Spectrum, Vector3<f64>, f64) {
        // here we will use uniform sampling
        let (u, v) = (sample[0], sample[1]);
        let phi = 2.0 * PI * u;
        let theta = v.acos();

        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();

        let mut wi = Vector3::new(x, y, z);

        // if the bxdf is transmissive
        if (self.types() & BxdfType::Reflection) == 0 {
            wi = -wi;
        }

        if (self.types() & BxdfType::Reflection) != 0 && (self.types() & BxdfType::Transmission) != 0 {
            eprintln!("Warning: Bxdf that is both reflecsive and transmissive should not use the default sample method");
        }

        (self.f(wo, wi), wi, INV_PI / 2.0)
    }
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

