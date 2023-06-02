pub mod point;
pub mod area;

use std::sync::Arc;

use crate::{spectrum::Spectrum, geometry::{interaction::SurfaceInteraction, ray::Ray}, sampler::wrs::Reservoir};
use cgmath::{Point2, Point3};

pub trait Light: Sync + Send {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2<f64>) -> (Spectrum, Point3<f64>, f64);
    fn uniform_sample_point(&self, u: Point2<f64>) -> Point3<f64>;
    fn le(&self) -> Spectrum;
    fn intersect_p(&self, r: &Ray) -> Option<f64>;
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction>;
}

pub struct LightList {
    pub lights: Vec<Arc<dyn Light>>,
}


impl LightList {
    pub fn new(lights: Vec<Arc<dyn Light>>) -> Self {
        LightList {
            lights,
        }
    }

    pub fn push(&mut self, light: Arc<dyn Light>) {
        self.push(light);
    }

    /// given a random number, importance sample a light, and then uniformly sample the light
    pub fn importance_sample_point(&self, u: Point2<f64>) -> Point3<f64> {
        // sample the light 
        assert!(!self.lights.is_empty());
        let mut r = Reservoir::new(self.lights[0].clone(), self.lights[0].le().sum());

        for light in &self.lights[1..] {
            let weight = light.le().sum();
            r.update(light.clone(), weight);
        }

        // uniform sample the light
        r.y.uniform_sample_point(u)
    }
}