pub mod path_integrator;

use crate::{geometry::ray::Ray, spectrum::Spectrum, primitive::scene::Scene};

pub trait Integrator {
    fn li(&self, ray: &mut Ray, scene: &Scene) -> Spectrum;
}