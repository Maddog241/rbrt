pub mod path_integrator;
// pub mod wrs_direct_integrator;
pub mod direct_integrator;

use std::sync::Arc;

use cgmath::{InnerSpace, Point3};

use crate::{geometry::{ray::Ray, interaction::SurfaceInteraction}, spectrum::Spectrum, scene::Scene, sampler::Sampler};

pub trait Integrator: Sync + Send {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: &Arc<dyn Sampler>) -> Spectrum;
}

fn visibility_test(isect: &SurfaceInteraction, sample_p: Point3<f64>, scene: &Scene) -> bool {
        let shadow_ray = Ray::new(isect.geo.p, sample_p-isect.geo.p, isect.time, 1.0-0.0001);
        // back facing surfaces do not get lit
        if shadow_ray.d.dot(isect.geo.n) < 0.0 { return false; }
        // test intersection 
        match scene.intersect_p(&shadow_ray) {
            Some(_t) => false,
            None => true,
        }
    }