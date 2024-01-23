use crate::utils::cos_theta;

use super::{cosine_sample_hemisphere, Bxdf, BxdfSample, BxdfType};
use super::Spectrum;
use cgmath::{EuclideanSpace, Point2, Vector3};
use std::f64::consts::PI;

const INV_PI: f64 = 1.0 / PI;

pub struct LambertianReflection {
    reflectance: Spectrum,    
}

impl LambertianReflection {
    pub fn new(reflectance: Spectrum) -> Self {
        Self {
            reflectance,
        }
    }
}

impl Bxdf for LambertianReflection {
    fn f(&self, _wo: Vector3<f64>, _wi: Vector3<f64>) -> Spectrum {
        self.reflectance * INV_PI
    }

    fn sample_f(&self, _wo: Vector3<f64>, sample: Point2<f64>) -> BxdfSample {
            let p = cosine_sample_hemisphere(sample);
            let wi = p.to_vec();

            if wi[0].is_nan() || wi[1].is_nan() || wi[2].is_nan() {
                println!("Not a Number in lambertian");
            }

            let rho = self.reflectance * INV_PI;
            let pdf = INV_PI * cos_theta(wi);

            BxdfSample {
                rho,
                wi,
                pdf,
                is_delta: self.is_delta()
            }
    }

    fn pdf(&self, _wo: Vector3<f64>, wi: Vector3<f64>) -> f64 {
        INV_PI * cos_theta(wi)
    }

    fn types(&self) -> i32 {
        BxdfType::Diffuse | BxdfType::Reflection
    }

    fn is_delta(&self) -> bool {
        false
    }
}
