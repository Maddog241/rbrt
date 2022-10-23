use super::Light;
use crate::geometry::interaction::SurfaceInteraction;
use crate::spectrum::Spectrum;
use cgmath::{Point2, Point3, Vector3, InnerSpace};

pub struct PointLight {
    p: Point3<f64>,
    intensity: Spectrum, // radiance = intensity / (distance)^2 
}

impl PointLight {
    pub fn new(p: Point3<f64>, intensity: Spectrum) -> Self {
        PointLight {
            p, 
            intensity
        }
    }
}

impl Light for PointLight {
    fn sample_li(&self, isect: &SurfaceInteraction, _sample: Point2<f64>) -> (Spectrum, Vector3<f64>, f64) {
        let pdf = 1.0;
        let wi = (self.p - isect.p).normalize();
        let distance2 = (self.p - isect.p).magnitude2();

        (self.intensity / distance2, wi, pdf)
    }
}