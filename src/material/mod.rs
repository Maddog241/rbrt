use crate::{bxdf::bsdf::Bsdf, geometry::interaction::SurfaceInteraction};

pub mod matte;
pub mod glass;
pub mod plastic;
pub mod mirror;
// pub mod layered_diffuse;

pub trait Material: Sync + Send {
    fn compute_scattering(&self, isect: &SurfaceInteraction) -> Bsdf;
    fn is_specular(&self) -> bool;
}
