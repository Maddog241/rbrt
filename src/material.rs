use crate::{bxdf::bsdf::Bsdf, geometry::interaction::SurfaceInteraction};

pub mod matte;
pub mod glass;

pub trait Material {
    fn compute_scattering(&self, isect: &SurfaceInteraction) -> Bsdf;
    fn is_specular(&self) -> bool;
}