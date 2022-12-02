use std::f64::consts::PI;

use cgmath::InnerSpace;

use crate::spectrum::Spectrum;

use super::{microfacet::MicrofacetDistribution, Bxdf, BxdfType};

pub struct FresnelBlend {
    distribution: MicrofacetDistribution,
    ks: Spectrum,
    kd: Spectrum,
}

impl FresnelBlend {
    pub fn new(distribution: MicrofacetDistribution, ks: Spectrum, kd: Spectrum) -> Self {
        Self {
            distribution,
            ks,
            kd,
        }
    }

    fn schlick(&self, cos_theta: f64) -> Spectrum {
        let pow5 = |v: f64| { (v * v) * (v * v) * v };
        self.ks + (Spectrum::new(1.0, 1.0, 1.0) - self.ks) * (1.0 - pow5(cos_theta))
    }
}

impl Bxdf for FresnelBlend {
    fn f(&self, wo: cgmath::Vector3<f64>, wi: cgmath::Vector3<f64>) -> Spectrum {
        let pow5 = |v: f64| {(v * v) * (v * v) * v };
        let diffuse = (28.0 * self.kd) / (23.0 * PI) * (Spectrum::new(1.0, 1.0, 1.0)-self.ks) * 
                (1.0 - pow5(1.0 - wi.z.abs()/2.0)) * (1.0 - pow5(1.0 - wo.z.abs()/2.0));

        let wh = (wi + wo).normalize();
        assert!(!wh.x.is_nan());
        let specular = self.distribution.d(wh) * self.schlick(wi.dot(wh)) / 
                (4.0 * wh.dot(wi) * wo.z.abs().max(wi.z.abs()));
        
        diffuse + specular
    }

    fn types(&self) -> i32 {
        BxdfType::Glossy | BxdfType::Reflection
    }
}