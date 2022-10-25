use crate::{bxdf::{bsdf::Bsdf, specular::FresnelSpecular}, spectrum::Spectrum, utils::perpendicular};
use cgmath::InnerSpace;
use super::Material;

pub struct Glass {
    eta_a: f64, // refractive index outside
    eta_b: f64, // refractive index inside
    r: Spectrum,
    t: Spectrum,
}

impl Glass {
    pub fn new(eta_a: f64, eta_b: f64, r: Spectrum, t: Spectrum) -> Self {
        Glass {
            eta_a,
            eta_b,
            r,
            t
        }
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
            bxdfs: vec![Box::new(FresnelSpecular::new(self.eta_a, self.eta_b, self.r, self.t))],
            n_bxdfs: 1,
        }
    }
}