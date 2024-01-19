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
                radiance: Some(radiance)
            };

            Some(isect)
        } else {
            None
        }
    }
}
