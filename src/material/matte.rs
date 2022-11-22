use crate::spectrum::Spectrum;
use crate::texture::Texture;
use crate::utils::perpendicular;
use crate::bxdf::{bsdf::Bsdf, Bxdf::LambertianReflection};

use super::Material;

pub struct Matte {
    kd: Box<dyn Texture<Spectrum>>,
}

impl Matte {
    pub fn new(kd: Box<dyn Texture<Spectrum>>) -> Matte {
        Matte { kd }
    }
}

impl Material for Matte {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
        let (ss, ts) =  perpendicular(isect.geo.n); 

        let ret = Bsdf {
            ns: isect.geo.n,
            ng: isect.geo.n,
            ss,
            ts,
            bxdfs: vec![LambertianReflection{reflectance: self.kd.evaluate(isect)} ],
            n_bxdfs: 1,
        };

        ret
    }

    fn is_specular(&self) -> bool {
        false
    }
}
