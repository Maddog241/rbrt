use core::panic;
use std::{f64::INFINITY, sync::Arc};

use cgmath::InnerSpace;

use crate::{spectrum::Spectrum, geometry::{ray::Ray, interaction::SurfaceInteraction}, scene::Scene, sampler::Sampler, bxdf::bsdf::Bsdf};
use super::{Integrator, visibility_test};

pub struct PathIntegrator {
    pub max_depth: usize,
    pub b_mis: bool,
}

impl PathIntegrator {
    pub fn new(max_depth: usize, b_mis: bool) -> Self {
        PathIntegrator { max_depth, b_mis }
    }
}

fn multiple_importance_sampling(radiance: &mut Spectrum, scene: &Scene, sampler: &Arc<dyn Sampler>, bsdf: &Bsdf, isect: &SurfaceInteraction, ray: &Ray, throughput: Spectrum) {
    let (light, light_pdf) = scene.lightlist.importance_sample_light(sampler.get_2d());

    // sample light
    let (li, sample_p, pdf) = light.sample_li(&isect, sampler.get_2d());
    let l_pdf = pdf * light_pdf;
    let l_radiance = if l_pdf > 0.0 && !li.is_black() && visibility_test(&isect, sample_p, scene) {
        let wi = (sample_p - isect.geo.p).normalize();
        let rho = bsdf.f(-ray.d.normalize(), wi);
        let cosine = wi.dot(isect.geo.n).abs();

        li * throughput * rho * cosine / l_pdf
    } else {
        Spectrum::new(0.0, 0.0, 0.0)
    };

    // sample bsdf 
    let (rho, wi, b_pdf) = bsdf.sample_f(-ray.d.normalize(), sampler.get_2d());
    let mut new_ray = Ray::new(isect.geo.p, wi, ray.time, INFINITY);
    let li = if let Some(inter) = scene.intersect(&mut new_ray) {
        if inter.hit_light {
            inter.radiance.unwrap()
        } else {
            Spectrum::new(0.0, 0.0, 0.0)
        }
    } else {
        Spectrum::new(0.0, 0.0, 0.0)
    };

    let b_radiance = if b_pdf > 0.0 && !li.is_black() {
        let cosine = wi.dot(isect.geo.n).abs();
        li * throughput * rho * cosine / b_pdf
    } else {
        Spectrum::new(0.0, 0.0, 0.0)
    };

    *radiance += (l_pdf) / (l_pdf + b_pdf) * l_radiance + (b_pdf) / (l_pdf + b_pdf) * b_radiance;
}

fn sample_one_light(radiance: &mut Spectrum, scene: &Scene, sampler: &Arc<dyn Sampler>, bsdf: &Bsdf, isect: &SurfaceInteraction, ray: &Ray, throughput: Spectrum) {
    let (light, light_pdf) = scene.lightlist.importance_sample_light(sampler.get_2d());
    let (li, sample_p, pdf) = light.sample_li(&isect, sampler.get_2d());
    let l_pdf = pdf * light_pdf;
    if l_pdf > 0.0 && !li.is_black() && visibility_test(&isect, sample_p, scene) {
        let wi = (sample_p - isect.geo.p).normalize();
        let rho = bsdf.f(-ray.d.normalize(), wi);
        let cosine = wi.dot(isect.geo.n).abs();

        *radiance += li * throughput * rho * cosine / l_pdf
    };
}

impl Integrator for PathIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: &Arc<dyn Sampler>) -> Spectrum {
        let mut throughput = Spectrum::new(1.0, 1.0, 1.0);
        let mut radiance = Spectrum::new(0.0, 0.0, 0.0);
        let mut specular = false;

        for depth in 0..self.max_depth {
            if let Some(isect) = scene.intersect(ray) {
                // hit the light after shot from the camera or leaving a specular vertex
                if isect.hit_light {
                    if depth == 0 || specular {
                        radiance += throughput * isect.radiance.unwrap();
                    }
                    break;
                }

                if let Some(mat) = &isect.material {
                    // check if it's specular vertex
                    specular = mat.is_specular();
                    let bsdf = mat.compute_scattering(&isect);
                    // sample lights to estimate the radiance value
                    if !specular && self.b_mis {
                        multiple_importance_sampling(&mut radiance, scene, sampler, &bsdf, &isect, ray, throughput); 
                    } else {
                        sample_one_light(&mut radiance, scene, sampler, &bsdf, &isect, ray, throughput)
                    }

                    // sample the bsdf to get the scattered ray
                    let sample = sampler.get_2d();
                    let (rho, wi, pdf) = bsdf.sample_f(-ray.d.normalize(), sample);

                    // update the throughput for next iteration, spawn the new ray
                    let cosine = wi.dot(isect.geo.n).abs();
                    throughput *= rho * cosine / pdf;
                    *ray = Ray::new(isect.geo.p, wi, ray.time, INFINITY);
                } else {
                    // hit the medium, currently not implemented
                    panic!();
                }
                
            } else {
                // does not hit the scene
                // radiance += Spectrum::skyblue(ray.d.y) * throughput;
                break;
            }
        }

        radiance
    }

}