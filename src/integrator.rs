pub mod path_integrator;
pub mod direct_integrator;

use cgmath::{InnerSpace, Point3};

use crate::{geometry::{ray::Ray, interaction::SurfaceInteraction}, spectrum::Spectrum, primitive::scene::Scene};

pub trait Integrator {
    fn li(&self, ray: &mut Ray, scene: &Scene) -> Spectrum;
}

fn visibility_test(isect: &SurfaceInteraction, sample_p: Point3<f64>, scene: &Scene) -> bool {
        let shadow_ray = Ray::new(isect.p, sample_p-isect.p, isect.time, 1.0-0.0001);
        // back facing surfaces do not get lit
        if shadow_ray.d.dot(isect.n) < 0.0 { return false; }
        // test intersection 
        match scene.intersect_p(&shadow_ray) {
            Some(_t) => false,
            None => true,
        }
    }