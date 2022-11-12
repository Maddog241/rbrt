use cgmath::{Point2, Point3, Vector3};

use super::bound3::Bound3;
use super::interaction::SurfaceInteraction;
use super::ray::Ray;



pub trait Shape: Sync + Send {
    fn object_bound(&self) -> Bound3;
    fn world_bound(&self) -> Bound3;
    fn intersect(&self, r: &Ray) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, r: &Ray) -> Option<f64>;
    fn area(&self) -> f64 ;
    fn sample(&self, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64);
}
