use crate::{bxdf::{bsdf::Bsdf, microfacet::{MicrofacetDistribution, MicrofacetReflection}, fresnel::{FresnelSpecular, FresnelNoOp}, lambertian::LambertianReflection}, utils::perpendicular, spectrum::Spectrum};

use super::Material;

pub struct Plastic {
    roughness: f64,
    ks: Spectrum, // reflectance for specular reflection
    kd: Spectrum, // reflectance for diffuse reflection
}

impl Plastic {
    pub fn new(roughness: f64, ks: Spectrum, kd: Spectrum) -> Self {
        Self {
            roughness,
            ks,
            kd,
        }
    }
}

impl Material for Plastic {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> Bsdf {
        let (ss, ts) = perpendicular(isect.geo.n);

        let rough = MicrofacetDistribution::roughness_to_alpha(self.roughness);
        let distribution = MicrofacetDistribution::TrowbridgeReitz { alpha_x: rough, alpha_y: rough};
        let fresnel = Box::new(FresnelSpecular::new(1.0, 1.5, Spectrum::new(1.0, 1.0, 1.0), Spectrum::new(1.0, 1.0, 1.0)));
        // let fresnel = Box::new(FresnelNoOp::new());
        
        let micro = MicrofacetReflection::new(
            distribution,
            fresnel,
            self.ks
        );

        let lambert = LambertianReflection::new(
            self.kd,
        );

        Bsdf {
            ng: isect.geo.n,
            ns: isect.geo.n,
            ss,
            ts,
            bxdfs: vec![Box::new(micro), Box::new(lambert)],
            n_bxdfs: 2,
        }
    }

    fn is_specular(&self) -> bool {
        false
    }
}