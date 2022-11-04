use std::f64::consts::PI;

use super::bound3::Bound3;
use super::interaction::*;
use super::ray::*;
use super::shape::Shape;
use super::transform::Transform;
use cgmath::Point2;
use cgmath::Vector3;
use cgmath::{EuclideanSpace, InnerSpace, Point3};

pub struct Sphere {
    radius: f64,
    object_to_world: Transform,
    world_to_object: Transform,
}

impl Sphere {
    pub fn new(radius: f64, object_to_world: Transform, world_to_object: Transform) -> Self {
        Sphere {
            radius,
            object_to_world,
            world_to_object,
        }
    }
}

impl Shape for Sphere {
    fn object_bound(&self) -> Bound3 {
        Bound3 {
            p_min: Point3::new(-self.radius, -self.radius, -self.radius),
            p_max: Point3::new(self.radius, self.radius, self.radius),
        }
    }

    fn world_bound(&self) -> Bound3 {
        self.object_to_world.transform_bound3(&self.object_bound())
    }

    fn intersect(&self, r: &Ray) -> Option<SurfaceInteraction> {
        // the incoming ray is in world space
        // transform the ray to object space
        let r = self.world_to_object.transform_ray(r);
        // compute the quadric coefficients
        let a = r.d.dot(r.d);
        let b = 2.0 * r.o.dot(r.d);
        let c = r.o.dot(r.o.to_vec()) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        // check the solution for t
        if discriminant <= 0.0 {
            // there's no intersection or only one(considered none)
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);
        if t < 0.0001 || t > r.t_max {
            // the smaller solution is invalid
            t = (-b + sqrt_discriminant) / (2.0 * a);
            if t < 0.0001 || t > r.t_max {
                // both solutions are invalid
                return None;
            }
        }
        // got a valid solution, compute interaction parameters
        let p = r.at(t);
        let n = p.to_vec().normalize();
        let inter = SurfaceInteraction {
            p,
            n,
            t,
            time: r.time,
            wo: -r.d.normalize(),
            material: None,
            hit_light: false,
            radiance: None,
        };
        // convert the interaction in the object space to world space

        let inter = self.object_to_world.transform_surface_interaction(&inter);
        Some(inter)
    }

    fn intersect_p(&self, r: &Ray) -> Option<f64> {
        let r = self.world_to_object.transform_ray(r);

        let a = r.d.dot(r.d);
        let b = 2.0 * r.o.dot(r.d);
        let c = r.o.dot(r.o.to_vec()) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);
        if t < 0.0001 || t > r.t_max {
            // the smaller solution is invalid
            t = (-b + sqrt_discriminant) / (2.0 * a);
            if t < 0.0001 || t > r.t_max {
                // both solutions are invalid
                return None;
            }
        }

        Some(t)
    }

    fn area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    fn sample(&self, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64) {
        let theta = (u[0] * 2.0 - 1.0).acos();
        let phi = u[1] * 2.0 * PI;
        let x = self.radius * theta.sin() * phi.cos();
        let y = self.radius * theta.sin() * phi.sin();
        let z = self.radius * theta.cos();

        let p = self.object_to_world.transform_point3(Point3::new(x, y, z));
        let n = self.object_to_world.transform_vector3(Vector3::new(x, y, z) / self.radius);
        (p, n, 1.0 / self.area())
    }

}
