use std::f64::consts::PI;

use cgmath::{Point2, Vector3, Point3, InnerSpace, EuclideanSpace};

use crate::geometry::{transform::Transform, bound3::Bound3, ray::{Ray, Beam}, interaction::GeometryInfo};

use super::Shape;


pub struct Disk {
    object_to_world: Transform,
    world_to_object: Transform,
    radius: f64,
}

impl Disk {
    pub fn new(object_to_world: Transform, world_to_object: Transform, radius: f64) -> Disk {
        Disk {
            object_to_world,
            world_to_object,
            radius
        }
    }
}

impl Shape for Disk {
    fn object_bound(&self) -> Bound3 {
        // DO NOT let the bounding box be thickless
        let p = Point3::new(-self.radius, -self.radius, 0.1);
        let q = Point3::new(self.radius, self.radius, -0.1);

        Bound3::new(p, q)
    }

    fn world_bound(&self) -> Bound3 {
        self.object_to_world.transform_bound3(&self.object_bound())
    }

    fn intersect(&self, r: &Ray) -> Option<GeometryInfo> {
        let r = self.world_to_object.transform_ray(r);

        if r.d.z == 0.0 { return None; } // parallel to the disk

        let t = -r.o.z / r.d.z;
        let p = r.at(t);
        // hit the plane

        // check if it is inside radius
        if t <= 0.0001 || t >= r.t_max || p.to_vec().magnitude2() > self.radius * self.radius {
            return None;
        }

        let n = Vector3::new(0.0, 0.0, 1.0);

        let geo = GeometryInfo{ p, n, t, wo: -r.d.normalize() };

        let geo = self.object_to_world.transform_geometry_info(&geo);

        Some(geo)
    }

    fn intersect_p(&self, r: &Ray) -> Option<f64> {
        let r = self.world_to_object.transform_ray(r);

        if r.d.z == 0.0 { return None; } // parallel to the disk

        let t = -r.o.z / r.d.z;
        let p = r.at(t);
        // hit the plane

        // check if it is inside radius
        if t <= 0.0001 || t >= r.t_max || p.to_vec().magnitude2() > self.radius * self.radius {
            return None;
        }

        Some(t)
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn uniform_sample_point(&self, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64) {
        let r = self.radius * u[0].sqrt();
        let theta = u[1] * PI * 2.0;

        let p = Point3::new(r * theta.cos(), r * theta.sin(), 0.0);
        let n = Vector3::new(0.0, 0.0, 1.0);
        let area_pdf = 1.0 / self.area();

        let p = self.object_to_world.transform_point3(p);
        let n = self.object_to_world.transform_vector3(n);

        (p, n, area_pdf)
    }
}
