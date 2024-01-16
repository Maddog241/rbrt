use crate::{spectrum::Spectrum, utils::perpendicular, bxdf::{perfect_specular::PerfectSpecular, bsdf::Bsdf}};

use super::Material;

pub struct Mirror {
    reflectance: Spectrum
}

impl Mirror {
    pub fn new(reflectance: Spectrum) -> Self {
        Self {
            reflectance
        }
    }
}

impl Material for Mirror {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
        let (ss, ts) = perpendicular(isect.geo.n);

        let bsdf = Bsdf {
            ns: isect.geo.n,
            ng: isect.geo.n,
            ss,
            ts,
            bxdfs: vec![Box::new(PerfectSpecular::new(self.reflectance))],
            n_bxdfs: 1
        };

        bsdf
    }

    fn is_specular(&self) -> bool {
        true
    }
}