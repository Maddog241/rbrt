use super::bound3::Bound3;
use super::interaction::SurfaceInteraction;
use super::ray::Ray;

pub trait Shape {
    fn object_bound(&self) -> Bound3;
    fn world_bound(&self) -> Bound3;
    fn intersect(&self, r: &Ray) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, r: &Ray) -> Option<f64>;
    fn area(&self) -> f64;
}