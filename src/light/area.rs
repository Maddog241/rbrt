use cgmath::{Point3, Point2, InnerSpace};
use crate::{geometry::{interaction::SurfaceInteraction, ray::Ray, shape::Shape}, spectrum::Spectrum};

use super::{Light, LightSample};

pub struct AreaLight {
    shape: Box<dyn Shape>,
    emit: Spectrum,
}

impl AreaLight {
    pub fn new(shape: Box<dyn Shape>, emit: Spectrum) -> AreaLight {
        AreaLight { shape, emit }
    }
}

impl Light for AreaLight {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2::<f64>) -> (Spectrum, Point3::<f64>, f64) {
        let (p, n, area_pdf) = self.shape.uniform_sample_point(u);

        let distance2 = (p - isect.geo.p).magnitude2();
        let we = (isect.geo.p - p).normalize();
        let cosine = we.dot(n).abs();

        // converts the pdf w.r.t area to pdf w.r.t. solid angle
        let pdf = area_pdf * distance2 / cosine;

        (self.le(), p, pdf)
    }

    fn uniform_sample_point(&self, u: Point2<f64>) -> LightSample {
        let (p, normal, pdf) = self.shape.uniform_sample_point(u);

        LightSample {
            position: p,
            normal,
            le: self.emit ,
            pdf,
        }
    }

    fn le(&self) -> Spectrum {
        self.emit
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        self.shape.intersect_p(r)
    }

    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        if let Some(geo) = self.shape.intersect(r) {
            // update r.t_max 
            r.t_max = geo.t;

            let isect = SurfaceInteraction {
                geo,
                time: r.time,
                material: None,
                hit_light: true,
                radiance: Some(self.le()),
            };

            Some(isect)
        } else {
            None
        }
    }
}
