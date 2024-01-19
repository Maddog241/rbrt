pub mod point;
pub mod area;

use std::sync::Arc;

use crate::{spectrum::Spectrum, geometry::{interaction::SurfaceInteraction, ray::Ray}, sampler::wrs::Reservoir};
use cgmath::{Point2, Point3, Vector3, InnerSpace};

pub trait Light: Sync + Send {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2<f64>) -> LightSample;

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

    /// sampling the lights with respect to their power
    /// 
    /// returns the ptr to the light and its sampling probability
    pub fn importance_sample_light(&self, u: Point2<f64>) -> (Arc<dyn Light>, f64) {
        // sample the light 
        assert!(!self.lights.is_empty());
        let mut r = Reservoir::new();

        for light in &self.lights {
            let weight = light.le().sum();
            r.update(light.clone(), weight);
        }

        let y = r.output_sample();
        (y.clone(), y.le().sum() / r.weight_sum)
    }
}

pub struct LightSample {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
    pub le: Spectrum,
    pub dir: Vector3<f64>, // direction
    pub pdf: f64, // pdf in area
    pub is_delta: bool
}

impl LightSample {
    pub fn pdf_area_to_solid(&self, isect: &SurfaceInteraction) -> f64 {
        let distance2 = (isect.geo.p - self.position).magnitude2();
        let we = (isect.geo.p - self.position).normalize();
        let cos_alpha = we.dot(self.normal);

        if cos_alpha > 0.0 {
            self.pdf * distance2 / cos_alpha
        } else {
            0.0 // garbage value?
        }
    }
}