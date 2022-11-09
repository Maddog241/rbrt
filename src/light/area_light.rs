use cgmath::{Point3, Point2, InnerSpace, Vector3};

use crate::{geometry::{shape::Shape, interaction::SurfaceInteraction, ray::Ray}, spectrum::Spectrum};

use super::Light;


pub fn sample_li(light: &Light, isect: &SurfaceInteraction, u: Point2::<f64>) -> (Spectrum, Point3::<f64>, f64) {
    if let Light::AreaLight { shape, emit } = light {
        let (p, n, area_pdf) = shape.sample(u);

        let distance2 = (p - isect.p).magnitude2();
        let we = (isect.p - p).normalize();
        let cosine = we.dot(n).abs();

        let pdf = area_pdf * distance2 / cosine;

        (le(light), p, pdf)
    } else {
        panic!()
    }
}

pub fn le(light: &Light) -> Spectrum {
    if let Light::AreaLight { shape, emit } = light {
        *emit
    } else {
        panic!()
    }
}

pub fn intersect_p(light: &Light, r: &crate::geometry::ray::Ray) -> Option<f64> {
    if let Light::AreaLight { shape, emit } = light {
        shape.intersect_p(r)
    } else {
        panic!()
    }
}

pub fn intersect(light: &Light, r: &mut Ray) -> Option<SurfaceInteraction> {
    if let Light::AreaLight { shape, emit } = light {
        if let Some(mut isect) = shape.intersect(r) {
            isect.hit_light = true;
            isect.radiance = Some(le(light));
            Some(isect)
        } else {
            None
        }
    } else {
        panic!()
    }
}