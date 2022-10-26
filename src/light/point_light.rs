use super::Light;
use crate::geometry::interaction::SurfaceInteraction;
use crate::spectrum::Spectrum;
use cgmath::{Point2, Point3, InnerSpace};

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
    fn sample_li(&self, isect: &SurfaceInteraction, _u: Point2<f64>) -> (Spectrum, Point3<f64>, f64) {
        let pdf = 1.0;
        let distance2 = (self.p - isect.p).magnitude2();

        if distance2 > 0.0 {
            (self.intensity / distance2, self.p, pdf)
        } else {
            (Spectrum::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0), pdf)
        }
    }

}