use std::f64::INFINITY;

use cgmath::{Point2, InnerSpace};
use rand::random;

use crate::{camera::{perspective::PerspectiveCamera, CameraSample, pixel::Pixel, Camera}, spectrum::Spectrum, geometry::ray::Ray, primitive::scene::Scene};
use crate::utils::assert_spectrum;
use super::Integrator;

pub struct PathIntegrator {
    max_depth: usize,
    camera: PerspectiveCamera,
}

impl PathIntegrator {
    pub fn new(max_depth: usize, camera: PerspectiveCamera) -> Self {
        PathIntegrator { max_depth, camera }
    }

    pub fn render(&mut self, scene: &Scene, filename: &str) {
        let res = self.camera.film.resolution;
        let (width, height) = (res.x, res.y);
        for i in 0..height {
            for j in 0..width {
                // first render the upper left pixel, then go rightwards and downwards
                let sample = CameraSample::new(Point2::new(j as f64, i as f64), 0.0);
                let mut r = self.camera.generate_ray(sample);

                let radiance = self.li(&mut r, scene);

                let pixel = radiance.to_pixel();
                self.camera.film.record(i, j, pixel);
            }
        }

        self.camera.film.write_to_image(filename);
    }
}

impl Integrator for PathIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene) -> Spectrum {
        let mut throughput = Spectrum::new(1.0, 1.0, 1.0);
        let mut radiance = Spectrum::new(0.0, 0.0, 0.0);

        for _depth in 0..self.max_depth {
            if let Some(isect) = scene.intersect(ray) {
                // first checks if it has directly hit the light

                // then check if it has hit a medium
                if let Some(mat) = &isect.material {
                    let bsdf = mat.compute_scattering(&isect);
                    // sample lights to estimate the radiance value
                    let sample: Point2<f64> = Point2::new(random(), random());
                    for light in scene.lights.iter() {
                        let (incoming_r, wi, pdf) = light.sample_li(&isect, sample);
                        // visibility testing for wi
                        let f_value = bsdf.f(-ray.d, wi);
                        let cosine = wi.dot(isect.n).max(0.0);
                        radiance += incoming_r * throughput * f_value * cosine / pdf;
                    }

                    // sample the bsdf to get the scattered ray
                    let sample: Point2<f64> = Point2::new(random(), random());
                    let (f_value, wi, pdf) = bsdf.sample_f(-ray.d, sample);

                    // update the throughput for next iteration, spawn the new ray
                    let cosine = wi.dot(isect.n).max(0.0);
                    throughput *= f_value * cosine / pdf;
                    *ray = Ray::new(isect.p, wi, ray.time, INFINITY);
                }
            } else {
                // does not hit the scene
                break;
            }
        }

        radiance
    }
}