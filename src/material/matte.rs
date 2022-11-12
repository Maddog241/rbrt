use crate::spectrum::Spectrum;
use crate::utils::perpendicular;
use crate::bxdf::{bsdf::Bsdf, Bxdf::LambertianReflection};

use super::Material;

pub struct Matte {
    kd: Spectrum,
}

impl Matte {
    pub fn new(kd: Spectrum) -> Matte {
        Matte { kd }
    }
}

impl Material for Matte {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
        let (ss, ts) =  perpendicular(isect.n); 

        let ret = Bsdf {
            ns: isect.n,
            ng: isect.n,
            ss,
            ts,
            bxdfs: vec![LambertianReflection{reflectance: self.kd} ],
            n_bxdfs: 1,
        };

        ret
    }

    fn is_specular(&self) -> bool {
        false
    }
}
