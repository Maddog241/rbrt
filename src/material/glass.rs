use crate::{bxdf::{bsdf::Bsdf, Bxdf::FresnelSpecular}, utils::perpendicular};
use super::Material;

pub fn compute_scattering(material: &Material, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
    if let Material::Glass { eta_a, eta_b, r, t } = material {
        let (ss, ts) = perpendicular(isect.n);
        Bsdf {
            ns: isect.n,
            ng: isect.n,
            ss,
            ts,
            bxdfs: vec![FresnelSpecular{eta_a: *eta_a, eta_b: *eta_b, r: *r, t: *t}],
            n_bxdfs: 1,
        }
    } else {
        panic!()
    }
}

pub fn is_specular(material: &Material) -> bool {
    if let Material::Glass { eta_a:_, eta_b:_, r:_, t:_ } = material {
        true
    } else {
        panic!()
    }
}