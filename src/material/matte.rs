use crate::utils::perpendicular;
use crate::{spectrum::Spectrum, bxdf::{bsdf::Bsdf, lambertian::LambertianReflection}};

use super::Material;

pub struct Matte {
    // now there's no texture. So this returns a Lambertian only
    kd: Spectrum,
}

impl Matte {
    pub fn new(kd: Spectrum) -> Matte {
        Matte {
            kd
        }
    }
}


impl Material for Matte {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
        let (ss, ts) = perpendicular(&isect.n);
        let ret = Bsdf {
            eta_i: 1.0,
            eta_o: 1.0,
            ns: isect.n,
            ng: isect.n,
            ss,
            ts,
            bxdfs: vec![Box::new(LambertianReflection::new(self.kd))],
            n_bxdfs: 1,
        };

        ret
    }
}
