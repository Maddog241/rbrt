use super::Primitive;
use std::sync::Arc;
use crate::geometry::{ray::Ray, interaction::SurfaceInteraction, bound3::Bound3, shape::Shape};


pub fn intersect(primitive: &Primitive, r: &mut Ray) -> Option<SurfaceInteraction> {
    if let Primitive::GeometricPrimitive { shape, material } = primitive {
        if let Some(mut isect) = shape.intersect(r) {
            // warning: here's no an infinitesimal value to avoid round-off errors. ok?
            r.t_max = isect.t;
            isect.material = Some(Arc::clone(material));
            // let isect point to the primitive
            Some(isect)
        } else  {
            return None            
        }
    } else {
        panic!()
    }
}

pub fn intersect_p(primitive: &Primitive, r: &Ray) -> Option<f64> {
    if let Primitive::GeometricPrimitive { shape, material } = primitive {
        shape.intersect_p(r)
    } else {
        panic!()
    }
}

pub fn world_bound(primitive: &Primitive) -> Bound3 {
    if let Primitive::GeometricPrimitive { shape, material } = primitive {
        shape.world_bound()
    } else {
        panic!()
    }
}