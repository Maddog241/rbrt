use cgmath::{Point3, Point2, InnerSpace, Vector3};
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
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2::<f64>) -> LightSample {
        let (p, n, pdf_area) = self.shape.uniform_sample_point(u);

        LightSample {
            position: p,
            normal: n,
            le: self.le(),
            dir: (isect.geo.p - p).normalize(),
            pdf: pdf_area, 
            is_delta: false,
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

            let radiance = if r.d.dot(geo.n) < 0.0 {
                self.le()
            } else {
                Spectrum::black()
            };

            let isect = SurfaceInteraction {
                geo,
                time: r.time,
                material: None,
                hit_light: true,
                radiance: Some(radiance),
                light: None,
            };

            Some(isect)
        } else {
            None
        }
    }

    fn pdf(&self, isect_p: Point3<f64>, isect_n: Vector3<f64>, p: Point3<f64>) -> f64 {
        let distance2 = (isect_p - p).magnitude2();
        let we = (isect_p - p).normalize();
        let cos_alpha = we.dot(isect_n).max(0.0);

        let pdf_area = 1.0 / self.shape.area();
        pdf_area * distance2 / cos_alpha
    }
}
