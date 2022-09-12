use super::{Bxdf, BxdfType};
use super::Spectrum;
use cgmath::{Vector3, Point2};
use std::f64::consts::PI;

const INV_PI: f64 = 1.0 / PI;

pub struct LambertianReflection {
    reflectance: Spectrum,    
    types: i32,
}

impl LambertianReflection {
    pub fn new(reflectance: Spectrum) -> Self {
        LambertianReflection  {
            reflectance,
            types: BxdfType::Reflection | BxdfType::Diffuse,
        }
    }
}

impl Bxdf for LambertianReflection {
    fn f(&self, _wo: &Vector3<f64>, _wi: &Vector3<f64>) -> Spectrum {
        self.reflectance * INV_PI
    }

    fn sample_f(&self, _wo: &Vector3<f64>, wi: &mut Vector3<f64>, sample: Point2<f64>, pdf: &mut f64) -> Spectrum {
        *pdf = INV_PI / 2.0;
        let theta = sample[0].acos();
        let phi = sample[1] * 2.0 * PI;
        *wi = Vector3::new(phi.cos(), phi.sin(), theta.sin());

        self.reflectance *  INV_PI
    }

    fn types(&self) -> i32 {
        self.types
    }
}