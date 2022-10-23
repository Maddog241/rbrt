use std::rc::Rc;

use cgmath::{Point3, Vector3};
use crate::primitive::geometric_primitive::GeometricPrimitive;

pub trait Interaction {}

pub struct SurfaceInteraction {
    pub p: Point3<f64>,  // hit point, in world space
    pub n: Vector3<f64>, // surface normal
    pub t: f64,          // denotes the parametric point along the ray
    pub time: f64,
    pub wo: Vector3<f64>, // the reverse direction of the incoming ray
                          // medium interface, to be implemented later
    // pub primitive: Option<Rc<GeometricPrimitive>>,
}

impl Interaction for SurfaceInteraction {}
