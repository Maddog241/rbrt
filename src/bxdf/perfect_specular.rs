use crate::{spectrum::Spectrum, utils::reflect};

use super::{Bxdf, BxdfType, BxdfSample};

pub struct PerfectSpecular {
    reflectance: Spectrum
}

impl PerfectSpecular {
    pub fn new(reflectance: Spectrum) -> Self {
        Self {
            reflectance
        }
    }
}

impl Bxdf for PerfectSpecular {
    fn f(&self, _wo: cgmath::Vector3<f64>, _wi: cgmath::Vector3<f64>) -> Spectrum {
        Spectrum::black()
    }

    fn sample_f(&self, wo: cgmath::Vector3<f64>, _sample: cgmath::Point2<f64>) -> BxdfSample {
        let wi = reflect(wo);

        let rho = self.reflectance / wo.z.abs();

        BxdfSample {
            rho,
            wi, 
            pdf: 1.0,
            is_delta: self.is_delta()
        }
    }

    fn pdf(&self, _wo: cgmath::Vector3<f64>, _wi: cgmath::Vector3<f64>) -> f64 {
        0.0
    }

    fn types(&self) -> i32 {
        BxdfType::Reflection | BxdfType::Specular
    }

    fn is_delta(&self) -> bool {
        true
    }
}