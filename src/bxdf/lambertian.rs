use super::{Bxdf, BxdfType, BxdfSample};
use super::Spectrum;
use cgmath::{Vector3, Point2};
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
            let pdf = INV_PI / 2.0;

            let theta = sample[0].acos();
            let phi = sample[1] * 2.0 * PI;
            let wi = Vector3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos());

            if wi[0].is_nan() || wi[1].is_nan() || wi[2].is_nan() {
                println!("Not a Number in lambertian");
            }

            let rho = self.reflectance * INV_PI;

            BxdfSample {
                rho,
                wi,
                pdf,
                is_delta: self.is_delta()
            }
    }

    fn pdf(&self, _wo: Vector3<f64>, _wi: Vector3<f64>) -> f64 {
        INV_PI / 2.0
    }

    fn types(&self) -> i32 {
        BxdfType::Diffuse | BxdfType::Reflection
    }

    fn is_delta(&self) -> bool {
        false
    }
}
