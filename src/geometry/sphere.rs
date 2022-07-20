use super::shape::Shape;
use super::bound3::Bound3;
use cgmath::*;

pub struct Sphere {
    radius: f64
}

impl Shape for Sphere {
    fn object_bound(&self) -> Bound3 {
        Bound3 {
            p_min: Point3::new(-self.radius, -self.radius, -self.radius),
            p_max: Point3::new(self.radius, self.radius, self.radius)
        }
    }

    fn world_bound(&self) -> Bound3 {
        
    }
}