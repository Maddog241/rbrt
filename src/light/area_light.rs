use cgmath::{Point3, Point2, InnerSpace};
use crate::{geometry::{interaction::SurfaceInteraction, ray::Ray, shape::Shape}, spectrum::Spectrum};

use super::Light;

pub struct AreaLight {
    shape: Box<dyn Shape>,
    emit: Spectrum,
}

impl AreaLight {
    pub fn new(shape: Box<dyn Shape>, emit: Spectrum) -> AreaLight {
        AreaLight { shape, emit}
    }
}

impl Light for AreaLight {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2::<f64>) -> (Spectrum, Point3::<f64>, f64) {
        let (p, n, area_pdf) = self.shape.sample(u);

        let distance2 = (p - isect.p).magnitude2();
        let we = (isect.p - p).normalize();
        let cosine = we.dot(n).abs();

        let pdf = area_pdf * distance2 / cosine;

        (self.le(), p, pdf)
    }

    fn le(&self) -> Spectrum {
        self.emit
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        self.shape.intersect_p(r)
    }

    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        if let Some(mut isect) = self.shape.intersect(r) {
            isect.hit_light = true;
            isect.radiance = Some(self.le());
            Some(isect)
        } else {
            None
        }
    }
}
