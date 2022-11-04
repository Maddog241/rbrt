use cgmath::{Point3, Point2, InnerSpace, Vector3};

use crate::{geometry::{shape::Shape, interaction::SurfaceInteraction, ray::Ray}, spectrum::Spectrum};

use super::Light;

pub struct AreaLight {
    shape: Box<dyn Shape>,
    emit: Spectrum,
}

impl AreaLight {
    pub fn new(shape: Box<dyn Shape>, emit: Spectrum) -> Self {
        AreaLight {
            shape,
            emit,
        }
    }
}

impl Light for AreaLight {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2::<f64>) -> (Spectrum, Point3::<f64>, f64) {
        let (p, n, area_pdf) = self.shape.sample(u);

        let distance2 = (p - isect.p).magnitude2();
        let we = isect.p - p;
        let cosine = we.dot(n).max(0.0);

        let pdf = if cosine != 0.0 { area_pdf * distance2 / cosine } else { 0.0 };

        (self.le(isect.n, isect.wo), p, pdf)
    }

    fn le(&self, n: Vector3<f64>, d: Vector3<f64>) -> Spectrum {
        self.emit * n.dot(d).max(0.0)
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        self.shape.intersect_p(r)
    }

    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        if let Some(mut isect) = self.shape.intersect(r) {
            isect.hit_light = true;
            isect.radiance = Some(self.le(isect.n, isect.wo));
            Some(isect)
        } else {
            None
        }
    }
}