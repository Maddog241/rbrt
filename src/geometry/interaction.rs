use std::sync::Arc;

use cgmath::{Point3, Vector3};
use crate::{material::Material, spectrum::Spectrum, light::Light};


pub struct GeometryInfo {
    pub p: Point3<f64>, // hit point, in world space
    pub n: Vector3<f64>, // surface normal
    pub t: f64,  // the parametric distance along the ray
    pub wo: Vector3<f64>, // normalized reverse direction of incoming ray
}


pub struct SurfaceInteraction {
    pub geo: GeometryInfo,
    pub time: f64,
    pub material: Option<Arc<dyn Material>>,
    pub hit_light: bool,
    pub radiance: Option<Spectrum>,
    pub light: Option<Arc<dyn Light>>,
}