use super::{Light, LightSample};
use crate::geometry::{interaction::SurfaceInteraction, ray::Ray};
use crate::spectrum::Spectrum;
use cgmath::{Point2, Point3, InnerSpace, Vector3};


pub struct PointLight {
    p: Point3<f64>,
    le: Spectrum,
}

#[allow(dead_code)]
impl PointLight {
    pub fn new(p: Point3<f64>, le: Spectrum) -> PointLight {
        PointLight { p, le }
    }
}

impl Light for PointLight {
    fn sample_li(&self, isect: &SurfaceInteraction, _u: Point2<f64>) -> (Spectrum, Point3<f64>, f64) {
        let pdf = 1.0;
        let distance2 = (self.p - isect.geo.p).magnitude2();

        if distance2 > 0.0 {
            (self.le, self.p, pdf)
        } else {
            (Spectrum::black(), Point3::new(0.0, 0.0, 0.0), pdf)
        }
    }

    fn uniform_sample_point(&self, _u: Point2<f64>) -> LightSample {
        LightSample {
            position: self.p,
            normal: Vector3::new(0.0, 0.0, 0.0), // problem?
            le: self.le,
            pdf: 1.0,
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
