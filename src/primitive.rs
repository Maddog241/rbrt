pub mod geometric_primitive;
pub mod scene;
pub mod bound_scene;


use crate::geometry::{ray::Ray, interaction::SurfaceInteraction, bound3::Bound3};

pub trait Primitive: Sync + Send {
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, r: &Ray) -> Option<f64>;
    fn world_bound(&self) -> Bound3;
}
