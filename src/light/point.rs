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
    fn sample_li(&self, isect: &SurfaceInteraction, _u: Point2<f64>) -> LightSample {
        let distance2 = (self.p - isect.geo.p).magnitude2();
        let dir = (isect.geo.p - self.p).normalize();

        if distance2 > 0.0 {
            LightSample {
                position: self.p,
                normal: Vector3::new(1.0, 0.0, 0.0),
                dir,
                le: self.le,
                pdf: 1.0,
                is_delta: true,
            }
        } else {
            LightSample {
                position: self.p,
                normal: Vector3::new(1.0, 0.0, 0.0),
                dir,
                le: Spectrum::black(),
                pdf: 1.0,
                is_delta: true,
            }
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
