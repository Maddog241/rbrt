use crate::utils::perpendicular;
use crate::bxdf::{bsdf::Bsdf, Bxdf::LambertianReflection};
use cgmath::InnerSpace;

use super::Material;

pub fn compute_scattering(material: &Material, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
    if let Material::Matte { kd } = material {
        let (ss, ts) = if isect.n.dot(isect.wo) > 0.0 { perpendicular(isect.n) } else {perpendicular(-isect.n) }; 
        let ret = Bsdf {
            ns: isect.n,
            ng: isect.n,
            ss,
            ts,
            bxdfs: vec![LambertianReflection{reflectance: *kd} ],
            n_bxdfs: 1,
        };

        ret
    } else {
        panic!()
    }
}

pub fn is_specular(material: &Material) -> bool {
    if let Material::Matte { kd } = material {
        false
    } else {
        panic!()
    }
}
