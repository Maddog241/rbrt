use cgmath::*;

pub trait Interaction {

}

pub struct SurfaceInteraction {
    p: Point3<f64>,
    n: Vector3<f64>,
    t: f64,
    time: f64,
}

impl Interaction for SurfaceInteraction{

}
