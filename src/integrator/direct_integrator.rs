
use cgmath::Point2;
use rand::random;

use crate::spectrum::Spectrum;

use super::*;

pub struct DirectIntegrator {
}

impl DirectIntegrator {
    pub fn new() -> Self {
        DirectIntegrator {
        }
    }
}

impl Integrator for DirectIntegrator {
    fn li(&self, ray: &mut crate::geometry::ray::Ray, scene: &Scene) -> crate::spectrum::Spectrum {
        match scene.intersect(ray) {
            Some(isect) => {
                if isect.hit_light {
                    isect.radiance.unwrap()
                } else {
                    match &isect.material {
                        Some(mat) => {
                            let u = Point2::new(random::<f64>(), random::<f64>());
                            let (light, light_pdf) = scene.lightlist.importance_sample_light(u);
                            let p_light = light.uniform_sample_point(u);
                            let light_pdf = light_pdf * p_light.pdf;

                            let wi = (p_light.position - isect.geo.p).normalize();
                            let wo = -ray.d.normalize();
                            let cos_theta = isect.geo.n.dot(wi);
                            let cos_alpha = wi.dot(p_light.normal).abs();
                            let r2 = (p_light.position - isect.geo.p).magnitude2();
                            let bsdf = mat.compute_scattering(&isect);

                            let mut lo = Spectrum::new(0.0, 0.0, 0.0);
                            if light_pdf > 0.0 && !p_light.le.is_black() && visibility_test(&isect, p_light.position, scene) {
                                lo = bsdf.f(wo, wi) * p_light.le * cos_theta * cos_alpha / (light_pdf * r2);
                            }

                            lo
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
                Spectrum::new(0.0, 0.0, 0.0)
            }
        }
    }
}