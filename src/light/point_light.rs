use super::Light;
use crate::geometry::{interaction::SurfaceInteraction, ray::Ray};
use crate::spectrum::Spectrum;
use cgmath::{Point2, Point3, InnerSpace};


pub struct PointLight {
    p: Point3<f64>,
    intensity: Spectrum,
}

impl PointLight {
    pub fn new(p: Point3<f64>, intensity: Spectrum) -> PointLight {
        PointLight { p, intensity}
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

    fn le(&self) -> Spectrum {
        panic!("This method should not be called");
    }

    fn intersect_p(&self, _r: &crate::geometry::ray::Ray) -> Option<f64> {
        None
    }

    fn intersect(&self, _r: &mut Ray) -> Option<SurfaceInteraction> {
        None
    }
}
