use super::Primitive;
use crate::geometry::{ray::Ray, interaction::SurfaceInteraction, bound3::Bound3, shape::Shape};
use crate::material::Material;
use std::rc::Rc;

pub struct GeometricPrimitive {
    shape: Box<dyn Shape>,
    material: Rc<dyn Material>,
}

impl GeometricPrimitive {
    pub fn new(shape: Box<dyn Shape>, material: Rc<dyn Material>) -> Self {
        GeometricPrimitive { shape, material }
    }
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        if let Some(mut isect) = self.shape.intersect(r) {
            // warning: here's no an infinitesimal value to avoid round-off errors. ok?
            r.t_max = isect.t;
            isect.material = Some(Rc::clone(&self.material));
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