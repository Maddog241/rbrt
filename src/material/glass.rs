use crate::{bxdf::{bsdf::Bsdf, Bxdf::FresnelSpecular}, utils::perpendicular, spectrum::Spectrum};
use super::Material;


pub struct Glass {
    eta_a: f64,
    eta_b: f64,
    r: Spectrum,
    t: Spectrum,
}

impl Glass {
    pub fn new(eta_a: f64, eta_b: f64, r: Spectrum, t: Spectrum) -> Glass {
        Glass { eta_a, eta_b, r, t}
    }
}

impl Material for Glass {
    fn compute_scattering(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> crate::bxdf::bsdf::Bsdf {
        let (ss, ts) = perpendicular(isect.n);
        Bsdf {
            ns: isect.n,
            ng: isect.n,
            ss,
            ts,
            bxdfs: vec![FresnelSpecular{eta_a: self.eta_a, eta_b: self.eta_b, r: self.r, t: self.t}],
            n_bxdfs: 1,
        }
    }

    fn is_specular(&self) -> bool {
        true
    }
}
