use super::Primitive;
use crate::geometry::{ray::Ray, interaction::SurfaceInteraction, bound3::Bound3, shape::Shape};
use std::rc::Rc;

pub struct GeometricPrimitive {
    shape: Box<dyn Shape>,
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        if let Some(isect) = self.shape.intersect(r) {
            // warning: here's no an infinitesimal value to avoid round-off errors. ok?
            r.t_max = isect.t;
            // let isect point to the primitive
            Some(isect)
        } else  {
            return None            
        }
    }

    fn intersect_p(&self, r: &Ray) -> Option<f64> {
        self.shape.intersect_p(r)
    }

    fn world_bound(&self) -> Bound3 {
        self.shape.world_bound()
    }
}