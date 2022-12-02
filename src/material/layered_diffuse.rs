use crate::{utils::perpendicular, spectrum::Spectrum, bxdf::{microfacet::MicrofacetDistribution, fresnel_blend::FresnelBlend, bsdf::Bsdf}};

use super::Material;

pub struct LayeredDiffuse {
    roughness: f64,
    kd: Spectrum, // this could be texture
    ks: Spectrum,
}

impl LayeredDiffuse {
    pub fn new(roughness: f64, ks: Spectrum, kd: Spectrum) -> Self {
        Self {
            roughness,
            ks,
            kd,
        }
    }
}

impl Material for LayeredDiffuse {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> Bsdf {
        let (ss, ts) = perpendicular(isect.geo.n);

        let alpha = MicrofacetDistribution::roughness_to_alpha(self.roughness);
        let distribution = MicrofacetDistribution::TrowbridgeReitz { alpha_x: alpha, alpha_y: alpha };
        let fresnel_blend = FresnelBlend::new(distribution, self.ks, self.kd);

        Bsdf {
            ng: isect.geo.n,
            ns: isect.geo.n,
            ss,
            ts,
            bxdfs: vec![Box::new(fresnel_blend)],
            n_bxdfs: 1,
        }
    }

    fn is_specular(&self) -> bool {
        false
    }
}