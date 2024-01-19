use crate::spectrum::Spectrum;

use super::*;

pub struct DirectIntegrator {
    max_depth: usize
}

impl DirectIntegrator {
    pub fn new(depth: usize) -> Self {
        DirectIntegrator {
            max_depth: depth
        }
    }
}

impl Integrator for DirectIntegrator {
    fn li(&self, ray: &mut crate::geometry::ray::Ray, scene: &Scene, sampler: &Arc<dyn Sampler>) -> crate::spectrum::Spectrum {
        let mut lo = Spectrum::new(0.0, 0.0, 0.0);
        let mut throughput = Spectrum::new(1.0, 1.0, 1.0);
        let mut specular = false;

        for depth in 0..self.max_depth {
            match scene.intersect(ray) {
                Some(isect) => {
                    if isect.hit_light {
                        if depth == 0 || specular {
                            lo += isect.radiance.unwrap()
                        } 
                        break;
                    } else {
                        match &isect.material {
                            Some(mat) => {
                                if !mat.is_specular() {
                                    let u = sampler.get_2d();
                                    let (light, light_pdf) = scene.lightlist.importance_sample_light(u);
                                    let p_light = light.sample_li(&isect, u);
                                    let light_pdf = light_pdf * p_light.pdf;

                                    let wi = (p_light.position - isect.geo.p).normalize();
                                    let wo = -ray.d.normalize();
                                    let cos_theta = isect.geo.n.dot(wi);
                                    let cos_alpha = wi.dot(-p_light.normal).max(0.0);
                                    let r2 = (p_light.position - isect.geo.p).magnitude2();
                                    let bsdf = mat.compute_scattering(&isect);

                                    if light_pdf > 0.0 && !p_light.le.is_black() && visibility_test(&isect, p_light.position, scene) {
                                        lo = throughput * bsdf.f(wo, wi) * p_light.le * cos_theta * cos_alpha / (light_pdf * r2);
                                    }

                                    break;
                                } else {
                                    specular = true;

                                    // sample the specular bsdf
                                    let bsdf = mat.compute_scattering(&isect);
                                    let wo = -ray.d.normalize();
                                    let bsdf_sample = bsdf.sample_f(wo, sampler.get_2d());
                                    let (rho, wi, pdf) = (bsdf_sample.rho, bsdf_sample.wi, bsdf_sample.pdf);

                                    let cosine = wi.dot(isect.geo.n).abs();
                                    throughput *= rho * cosine / pdf;

                                    // spawn the new ray
                                    *ray = Ray::new(isect.geo.p, wi, ray.time, f64::INFINITY);
                                }
                                
                            }
                            None => {
                                panic!();
                            }
                        }
                    }
                },
                None => {
                    // returns the background color
                    // Spectrum::skyblue(ray.d.y)
                }
            }
        }

        lo
    }
}