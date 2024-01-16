use crate::{spectrum::Spectrum, utils::reflect};

use super::{Bxdf, BxdfType};

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
        Spectrum::new(0.0, 0.0, 0.0)
    }

    fn sample_f(&self, wo: cgmath::Vector3<f64>, _sample: cgmath::Point2<f64>) -> (Spectrum, cgmath::Vector3<f64>, f64) {
        let wi = reflect(wo);

        (self.reflectance / wo.z.abs(), wi, 1.0)
    }

    fn types(&self) -> i32 {
        BxdfType::Reflection | BxdfType::Specular
    }
}