use cgmath::{Point3, Vector3};

pub trait Interaction {}

pub struct SurfaceInteraction {
    pub p: Point3<f64>,  // hit point, in world space
    pub n: Vector3<f64>, // surface normal
    pub t: f64,          // denotes the parametric point along the ray
    pub time: f64,
    pub wo: Vector3<f64>, // the reverse direction of the incoming ray
                          // medium interface, to be implemented later
}

impl Interaction for SurfaceInteraction {}
