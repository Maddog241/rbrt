pub mod lambertian;
pub mod bsdf;
pub mod fresnel;
pub mod microfacet;
pub mod perfect_specular;
// pub mod fresnel_blend;

use std::{ops::{BitOr, BitAnd}, f64::consts::PI};

use crate::{spectrum::Spectrum, utils::cos_theta};
use cgmath::{EuclideanSpace, InnerSpace, Point2, Point3, Vector3};

const INV_PI: f64 = 1.0 / PI;

pub struct BxdfSample {
    pub rho: Spectrum,
    pub wi: Vector3<f64>,
    pub pdf: f64,
    pub is_delta: bool
}

pub trait Bxdf {
    fn f(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> Spectrum;
    fn sample_f(&self, wo: Vector3<f64>, sample: Point2<f64>) -> BxdfSample {
        // // here we will use uniform sampling
        // let (u, v) = (sample[0], sample[1]);
        // let phi = 2.0 * PI * u;
        // let theta = v.acos();
        // // let theta = v.asin() / 2.0;

        // let x = theta.sin() * phi.cos();
        // let y = theta.sin() * phi.sin();
        // let z = theta.cos();

        // cosine sample the hemisphere
        let p = cosine_sample_hemisphere(sample);

        let mut wi = p.to_vec();

        // if the bxdf is transmissive
        if (self.types() & BxdfType::Reflection) == 0 {
            wi = -wi;
        }

        if (self.types() & BxdfType::Reflection) != 0 && (self.types() & BxdfType::Transmission) != 0 {
            eprintln!("Warning: Bxdf that is both reflecsive and transmissive should not use the default sample method");
        }

        let rho = self.f(wo, wi);

        BxdfSample {
            rho,
            wi,
            pdf: self.pdf(wo, wi),
            is_delta: self.is_delta()
        }
    }
    fn pdf(&self, _wo: Vector3<f64>, wi: Vector3<f64>) -> f64 {
        INV_PI * cos_theta(wi)
    }
    fn types(&self) -> i32;
    fn is_delta(&self) -> bool;
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


// given (u0, u1), return (x, y)
pub fn uniform_sample_disk(u: Point2<f64>) -> Point2<f64> {
    let r = u[0].sqrt();
    let theta = 2.0 * PI * u[1];

    let x = r * theta.cos();
    let y = r * theta.sin();

    Point2::new(x, y)
}

pub fn cosine_sample_hemisphere(u: Point2<f64>) -> Point3<f64> {
    let p= uniform_sample_disk(u);
    let z = (1.0 - p.x * p.x - p.y * p.y).sqrt();

    Point3::new(p.x, p.y, z)
}