use core::panic;
use std::{f64::INFINITY, sync::Arc};

use cgmath::InnerSpace;

use crate::{spectrum::Spectrum, geometry::{ray::Ray, interaction::SurfaceInteraction}, scene::Scene, sampler::Sampler, bxdf::bsdf::Bsdf, light::LightSample};
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

fn power_heuristic(n1: i32, p1: f64, n2: i32, p2: f64) -> f64 {
    let np1 = n1 as f64 * p1;
    let np2 = n2 as f64 * p2;
    (np1 * np1) / (np1 * np1 + np2 * np2)
}

fn multiple_importance_sampling(scene: &Scene, sampler: &Arc<dyn Sampler>, bsdf: &Bsdf, isect: &SurfaceInteraction, ray: &Ray, throughput: Spectrum) -> Spectrum {
    let mut res = Spectrum::black();

    {
        let (light, light_pdf) = scene.lightlist.importance_sample_light(sampler.get_2d());

        // sample light
        let light_sample = light.sample_li(&isect, sampler.get_2d());
        let l_pdf = light_sample.pdf_area_to_solid(&isect) * light_pdf;
        let li = light_sample.le;

        res += if l_pdf > 0.0 && !li.is_black() && visibility_test(&isect, light_sample.position, scene) {
            let wi = -light_sample.dir;
            let wo = -ray.d.normalize();
            let rho = bsdf.f(wo, wi);
            let b_pdf = bsdf.pdf(wo, wi);
            let cosine = wi.dot(isect.geo.n).abs();

            let weight = if light_sample.is_delta { 
                power_heuristic(1, l_pdf, 1, b_pdf)
            } else {
                1.0
            };

            weight * li * throughput * rho * cosine / l_pdf
        } else {
            Spectrum::black()
        };

        if light_sample.is_delta {
            // if the sampled light is delta light, do not sample the bsdf cause there's no need
            return res;
        }
    }

    {
        // sample bsdf 
        let wo = -ray.d.normalize();
        let bsdf_sample = bsdf.sample_f(wo, sampler.get_2d());

        let wi = bsdf_sample.wi;
        let b_pdf = bsdf_sample.pdf;
        let rho = bsdf_sample.rho;

        // shoot a ray
        let mut new_ray = Ray::new(isect.geo.p, wi, ray.time, INFINITY);
        let (li, l_pdf) = if let Some(inter) = scene.intersect(&mut new_ray) {
            if inter.hit_light {
                let l_pdf = inter.light.unwrap().pdf(inter.geo.p, inter.geo.n, isect.geo.p);
                (inter.radiance.unwrap(), l_pdf)
            } else {
                (Spectrum::black(), 0.0)
            }
        } else {
            (Spectrum::black(), 0.0)
        };


        res += if b_pdf > 0.0 && !li.is_black() {
            let cosine = wi.dot(isect.geo.n).abs();
            let weight = power_heuristic(1, b_pdf, 1, l_pdf);
            weight * li * throughput * rho * cosine / b_pdf
        } else {
            Spectrum::black()
        };
    }

    res
}

fn sample_one_light(scene: &Scene, sampler: &Arc<dyn Sampler>, bsdf: &Bsdf, isect: &SurfaceInteraction, ray: &Ray, throughput: Spectrum) -> Spectrum {
    let (light, light_pdf) = scene.lightlist.importance_sample_light(sampler.get_2d());
    let light_sample = light.sample_li(&isect, sampler.get_2d());

    let l_pdf = light_sample.pdf_area_to_solid(&isect) * light_pdf;

    let li = light_sample.le;

    if l_pdf > 0.0 && !li.is_black() && visibility_test(&isect, light_sample.position, scene) {
        let wi = -light_sample.dir;
        let rho = bsdf.f(-ray.d.normalize(), wi);
        let cosine = wi.dot(isect.geo.n).abs();

        li * throughput * rho * cosine / l_pdf
    } else {
        Spectrum::black()
    }
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
                    // if !specular && self.b_mis {
                    if self.b_mis {
                        radiance += multiple_importance_sampling(scene, sampler, &bsdf, &isect, ray, throughput); 
                    } else {
                        radiance += sample_one_light(scene, sampler, &bsdf, &isect, ray, throughput)
                    }

                    // sample the bsdf to get the scattered ray
                    let sample = sampler.get_2d();
                    let bxdf_sample = bsdf.sample_f(-ray.d.normalize(), sample);
                    let (rho, wi, pdf) = (bxdf_sample.rho, bxdf_sample.wi, bxdf_sample.pdf);

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