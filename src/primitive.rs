pub mod geometric_primitive;

use crate::geometry::{ray::Ray, interaction::SurfaceInteraction, bound3::Bound3};

pub trait Primitive {
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, r: &Ray) -> Option<f64>;
    fn world_bound(&self) -> Bound3;
}