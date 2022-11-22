use std::f64::consts::PI;

use cgmath::{Point2, Point3, EuclideanSpace, InnerSpace};

use crate::geometry::transform::Transform;

use super::TextureMapping2D;

pub struct SphericalMapping {
    world_to_texture: Transform,
}

impl SphericalMapping {
    pub fn new(world_to_texture: Transform) -> Self {
        SphericalMapping {  
            world_to_texture,
        }
    }

    fn sphere(&self, p: Point3<f64>) -> Point2<f64> {
        let vec = self.world_to_texture.transform_point3(p).to_vec().normalize();
        let phi = vec.y.atan2(vec.x);
        let theta = vec.z.acos();
        Point2::new(phi / (2.0 * PI), theta / PI)
    }
}

impl TextureMapping2D for SphericalMapping {
    fn map(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> cgmath::Point2<f64> {
        let st = self.sphere(isect.geo.p);
        st
    }
}