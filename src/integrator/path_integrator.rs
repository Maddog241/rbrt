use std::f64::INFINITY;

use cgmath::{Point2, Point3, InnerSpace};
use rand::random;

use crate::{camera::{perspective::PerspectiveCamera, CameraSample, Camera}, spectrum::Spectrum, geometry::{ray::Ray, interaction::SurfaceInteraction}, primitive::scene::Scene};
use super::Integrator;

pub struct PathIntegrator {
    max_depth: usize,
    camera: PerspectiveCamera,
    n_sample: usize,
}

impl PathIntegrator {
    pub fn new(max_depth: usize, camera: PerspectiveCamera) -> Self {
        PathIntegrator { max_depth, camera, n_sample: 200}
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

    fn visibility_test(&self, isect: &SurfaceInteraction, sample_p: Point3<f64>, scene: &Scene) -> bool {
        let shadow_ray = Ray::new(isect.p, sample_p-isect.p, isect.time, 1.0-0.0001);
        // back facing surfaces do not get lit
        if shadow_ray.d.dot(isect.n) < 0.0 { return false; }
        // test intersection 
        match scene.intersect_p(&shadow_ray) {
            Some(_t) => false,
            None => true,
        }
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
                        // sample once for each light in the scene
                        let (incoming_r, sample_p, pdf) = light.sample_li(&isect, sample);
                        // visibility testing for wi
                        if self.visibility_test(&isect, sample_p, scene) && pdf > 0.0 {
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