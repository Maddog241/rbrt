use super::Light;
use crate::geometry::interaction::SurfaceInteraction;
use crate::spectrum::Spectrum;
use cgmath::{Point2, Point3, Vector3, InnerSpace};

pub struct PointLight {
    p: Point3<f64>,
    intensity: Spectrum, // radiance = intensity / (distance)^2 
}

impl Light for PointLight {
    fn sample_li(&self, isect: &SurfaceInteraction, _sample: Point2<f64>, wi: &mut Vector3<f64>, pdf: &mut f64) -> Spectrum {
        *pdf = 1.0;
        *wi = (self.p - isect.p).normalize();
        let distance2 = (self.p - isect.p).magnitude2();
        self.intensity / distance2
    }
}