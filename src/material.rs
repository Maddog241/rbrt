use crate::{bxdf::bsdf::Bsdf, geometry::interaction::SurfaceInteraction};

pub mod matte;

pub trait Material {
    fn compute_scattering(&self, isect: &SurfaceInteraction) -> Bsdf;
}