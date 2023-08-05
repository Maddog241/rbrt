use core::panic;
use std::{f64::INFINITY, sync::Arc};

use cgmath::{Point2, InnerSpace};
use rand::random;

use crate::{spectrum::Spectrum, geometry::ray::Ray, scene::Scene, sampler::Sampler};
use super::{Integrator, visibility_test};

pub struct PathIntegrator {
    pub max_depth: usize,
}

impl PathIntegrator {
    pub fn new(max_depth: usize) -> Self {
        PathIntegrator { max_depth }
    }
}

impl Integrator for PathIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: &Arc<dyn Sampler>) -> Spectrum {
        let mut throughput = Spectrum::new(1.0, 1.0, 1.0);
        let mut radiance = Spectrum::new(0.0, 0.0, 0.0);
        let mut specular = false;
        let mut count = 0;

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
                    if !specular {
                        let light = scene.lightlist.importance_sample_light(sampler.get_2d()).0;
                        let (li, sample_p, pdf) = light.sample_li(&isect, Point2::new(random(), random()));
                        // visibility testing for wi
                        if pdf > 0.0 && !li.is_black() && visibility_test(&isect, sample_p, scene) {
                            let wi = (sample_p - isect.geo.p).normalize();
                            let rho = bsdf.f(-ray.d.normalize(), wi);
                            let cosine = wi.dot(isect.geo.n).abs();
                            
                            radiance += li * throughput * rho * cosine / pdf; 
                            count += 1;
                        } 
                    }

                    // sample the bsdf to get the scattered ray
                    let sample = sampler.get_2d();
                    let (rho, wi, pdf) = bsdf.sample_f(-ray.d.normalize(), sample);

                    // update the throughput for next iteration, spawn the new ray
                    let cosine = wi.dot(isect.geo.n).abs();
                    throughput *= rho * cosine / pdf;
                    *ray = Ray::new(isect.geo.p, wi, ray.time, INFINITY);
                } else {
                    // hit the medium
                    panic!();
                }
                
            } else {
                // does not hit the scene
                radiance += Spectrum::skyblue(ray.d.y) * throughput;
                count += 1;
                break;
            }
        }

        radiance / count as f64
    }

}