use super::Light;
use crate::geometry::{interaction::SurfaceInteraction, ray::Ray};
use crate::spectrum::Spectrum;
use cgmath::{Point2, Point3, InnerSpace, Vector3};


pub fn sample_li(light: &Light, isect: &SurfaceInteraction, _u: Point2<f64>) -> (Spectrum, Point3<f64>, f64) {
    if let Light::PointLight { p, intensity } = light {
        let pdf = 1.0;
        let distance2 = (p - isect.p).magnitude2();

        if distance2 > 0.0 {
            (*intensity / distance2, *p, pdf)
        } else {
            (Spectrum::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0), pdf)
        }
    } else {
        panic!()
    }
}

pub fn le(light: &Light) -> Spectrum {
    if let Light::PointLight { p, intensity } = light {
        panic!("This method should not be called");
    } else {
        panic!()
    }
}

pub fn intersect_p(light: &Light, _r: &crate::geometry::ray::Ray) -> Option<f64> {
    if let Light::PointLight { p, intensity } = light {
        None
    } else {
        panic!()
    }
}

pub fn intersect(light: &Light, _r: &mut Ray) -> Option<SurfaceInteraction> {
    if let Light::PointLight { p, intensity } = light {
        None
    } else {
        panic!()
    }
}