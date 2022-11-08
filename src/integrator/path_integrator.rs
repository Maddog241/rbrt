use std::f64::INFINITY;
use std::thread;
use std::sync::{Arc, Mutex};

use cgmath::{Point2, InnerSpace};
use rand::random;

use crate::{camera::{perspective::PerspectiveCamera, CameraSample, Camera}, spectrum::Spectrum, geometry::ray::Ray, primitive::scene::Scene, utils::random_2d};
use super::{Integrator, visibility_test};

pub struct PathIntegrator {
    pub max_depth: usize,
    pub camera: PerspectiveCamera,
    pub n_sample: usize,
    pub n_thread: usize,
}

impl PathIntegrator {
    pub fn new(camera: PerspectiveCamera, max_depth: usize) -> Self {
        PathIntegrator { max_depth, camera, n_sample: 20, n_thread: 10}
    }
}

impl Integrator for PathIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene) -> Spectrum {
        let mut throughput = Spectrum::new(1.0, 1.0, 1.0);
        let mut radiance = Spectrum::new(0.0, 0.0, 0.0);
        let mut specular = false;

        for depth in 0..self.max_depth {
            if let Some(isect) = scene.intersect(ray) {
                // first checks if it has directly hit the light
                if depth == 0 && isect.hit_light {
                    radiance = isect.radiance.unwrap();
                    break;
                }

                // hit the light
                if specular && isect.hit_light {
                    radiance += throughput * isect.radiance.unwrap();
                    break;
                }

                if isect.hit_light {
                    break;
                }

                // then check if it has hit a medium
                if let Some(mat) = &isect.material {
                    specular = mat.is_specular();
                    let bsdf = mat.compute_scattering(&isect);
                    // sample lights to estimate the radiance value
                    for light in scene.lights.iter() {
                        // sample once for each light in the scene
                        let sample: Point2<f64> = Point2::new(random(), random());
                        let (incoming_r, sample_p, pdf) = light.sample_li(&isect, sample);
                        // visibility testing for wi
                        if pdf > 0.0 && !incoming_r.is_black() && visibility_test(&isect, sample_p, scene) {
                            let wi = (sample_p - isect.p).normalize();
                            let f_value = bsdf.f(-ray.d.normalize(), wi);
                            let cosine = wi.dot(isect.n).abs();

                            radiance += incoming_r * throughput * f_value * cosine / pdf; 
                        }
                    }

                    // sample the bsdf to get the scattered ray
                    let sample: Point2<f64> = Point2::new(random(), random());
                    let (f_value, wi, pdf) = bsdf.sample_f(-ray.d.normalize(), sample);

                    // update the throughput for next iteration, spawn the new ray
                    let cosine = wi.dot(isect.n).abs();
                    throughput *= f_value * cosine / pdf;
                    *ray = Ray::new(isect.p, wi, ray.time, INFINITY);
                } else {
                    // hit the medium
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