use crate::{bxdf::{bsdf::Bsdf, fresnel::FresnelSpecular}, utils::perpendicular, spectrum::Spectrum};
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
        let (ss, ts) = perpendicular(isect.geo.n);
        Bsdf {
            ns: isect.geo.n,
            ng: isect.geo.n,
            ss,
            ts,
            bxdfs: vec![Box::new(FresnelSpecular::new(self.eta_a, self.eta_b, self.r, self.t)) ],
            n_bxdfs: 1,
        }
    }

    fn is_specular(&self) -> bool {
        true
    }
}
