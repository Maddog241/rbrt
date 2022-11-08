use std::f64::consts::PI;

use super::bound3::Bound3;
use super::interaction::*;
use super::ray::*;
use super::shape::Shape;
use cgmath::Point2;
use cgmath::Vector3;
use cgmath::{EuclideanSpace, InnerSpace, Point3};


pub fn object_bound(sphere: &Shape) -> Bound3 {
    if let Shape::Sphere { radius, object_to_world, world_to_object } = sphere {
        Bound3 {
            p_min: Point3::new(-radius, -radius, -radius),
            p_max: Point3::new(*radius, *radius, *radius),
        }
    } else {
        panic!()
    }
}

pub fn world_bound(sphere: &Shape) -> Bound3 {
    if let Shape::Sphere { radius, object_to_world, world_to_object } = sphere {
        object_to_world.transform_bound3(&object_bound(sphere))
    } else {
        panic!()
    }
}

pub fn intersect(sphere: &Shape, r: &Ray) -> Option<SurfaceInteraction> {
    if let Shape::Sphere { radius, object_to_world, world_to_object } = sphere {
        // the incoming ray is in world space
        // transform the ray to object space
        let r = world_to_object.transform_ray(r);
        // compute the quadric coefficients
        let a = r.d.dot(r.d);
        let b = 2.0 * r.o.dot(r.d);
        let c = r.o.dot(r.o.to_vec()) - radius * radius;
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

        let inter = object_to_world.transform_surface_interaction(&inter);
        Some(inter)

    } else {
        panic!()
    }
}

pub fn intersect_p(sphere: &Shape, r: &Ray) -> Option<f64> {
    if let Shape::Sphere { radius, object_to_world, world_to_object } = sphere {
        let r = world_to_object.transform_ray(r);

        let a = r.d.dot(r.d);
        let b = 2.0 * r.o.dot(r.d);
        let c = r.o.dot(r.o.to_vec()) - radius * radius;
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
    } else {
        panic!()
    }
}

pub fn area(sphere: &Shape) -> f64 {
    if let Shape::Sphere { radius, object_to_world, world_to_object } = sphere {
        4.0 * PI * radius * radius
    } else {
        panic!()
    }
}

pub fn sample(sphere: &Shape, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64) {
    if let Shape::Sphere { radius, object_to_world, world_to_object } = sphere {
        let theta = (u[0] * 2.0 - 1.0).acos();
        let phi = u[1] * 2.0 * PI;
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();

        let p = object_to_world.transform_point3(Point3::new(x, y, z));
        let n = object_to_world.transform_vector3(Vector3::new(x, y, z) / *radius);
        (p, n, 1.0 / area(sphere))
    } else {
        panic!()
    }
}
