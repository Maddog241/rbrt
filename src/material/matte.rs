use crate::utils::perpendicular;
use crate::{spectrum::Spectrum, bxdf::{bsdf::Bsdf, lambertian::LambertianReflection}};
use cgmath::InnerSpace;

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
        let (ss, ts) = if isect.n.dot(isect.wo) > 0.0 { perpendicular(isect.n) } else {perpendicular(-isect.n) }; 
        let ret = Bsdf {
            ns: isect.n,
            ng: isect.n,
            ss,
            ts,
            bxdfs: vec![Box::new(LambertianReflection::new(self.kd))],
            n_bxdfs: 1,
        };

        ret
    }

    fn is_specular(&self) -> bool {
        false
    }
}
