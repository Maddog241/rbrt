use std::f64::INFINITY;

use cgmath::Point2;
use rand::random;

use crate::{camera::{perspective::PerspectiveCamera, CameraSample, Camera}, spectrum::Spectrum, utils::random_2d, light::Light};

use super::*;

pub struct DirectIntegrator {
    camera: PerspectiveCamera,
    max_depth: usize,
    n_sample: usize,
}

impl DirectIntegrator {
    pub fn new(camera: PerspectiveCamera, max_depth: usize) -> Self {
        DirectIntegrator {
            camera,
            max_depth,
            n_sample: 200,
        }
    }

    pub fn render(&mut self, scene: &Scene, filename: &str) {
        let res = self.camera.film.resolution;
        let (width, height) = (res.x, res.y);
        for i in 0..height {
            for j in 0..width {
                // first render the upper left pixel, then go rightwards and downwards
                let mut radiance = Spectrum::new(0.0, 0.0, 0.0);

                for _ in 0..self.n_sample {
                    let sample = CameraSample::new(Point2::new(j as f64 + random::<f64>(), i as f64 + random::<f64>()), 0.0);
                    let mut r = self.camera.generate_ray(sample);

                    radiance += self.li(&mut r, scene);
                }

                radiance /= self.n_sample as f64;
                let pixel = radiance.to_pixel();
                self.camera.film.record(i, j, pixel);
            }
        }

        self.camera.film.write_to_image(filename);

    }

    fn uniform_sample_one_light(&self, ray: &mut Ray, scene: &Scene, depth: usize) -> Spectrum {
        if depth == 0 { return Spectrum::new(1.0, 0.0, 0.0); }

        let mut radiance = Spectrum::new(0.0, 0.0, 0.0);

        if let Some(isect) = scene.intersect(ray) {
            if let Some(mat) = &isect.material {
                let bsdf = mat.compute_scattering(&isect);
                if mat.is_specular() {
                    let sample = random_2d();
                    let (f_value, wi, pdf) = bsdf.sample_f(-ray.d.normalize(), sample);
                    *ray = Ray::new(isect.p, wi, ray.time, INFINITY);
                    let cosine = wi.dot(isect.n).abs();
                    radiance += f_value * cosine * self.uniform_sample_one_light(ray, scene, depth-1) / pdf;
                } else {
                    let u = random::<f64>();
                    let i = (u * scene.lights.len() as f64) as usize;
                    let light = &scene.lights[i];
                    let (incoming_r, sample_p, pdf) = light.sample_li(&isect, random_2d());

                    if pdf > 0.0 && !incoming_r.is_black() && visibility_test(&isect, sample_p, scene) {
                        let wi = (sample_p - isect.p).normalize();
                        let f_value = bsdf.f(-ray.d.normalize(), wi);
                        let n_lights = scene.lights.len() as f64;
                        let cosine = wi.dot(isect.n).abs();

                        radiance += n_lights * f_value * incoming_r * cosine / pdf;
                    }
                }
            } else {
                // medium or light
                if isect.hit_light {
                    radiance += isect.radiance.unwrap();
                }
            }
        } else {
            // hit nothing in the scene

            // radiance += Spectrum::skyblue(ray.d.y);
        }

        radiance
    }
}

impl Integrator for DirectIntegrator {
    fn li(&self, ray: &mut crate::geometry::ray::Ray, scene: &crate::primitive::scene::Scene) -> crate::spectrum::Spectrum {
        self.uniform_sample_one_light(ray, scene, self.max_depth)
    }
}