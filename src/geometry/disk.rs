use std::f64::consts::PI;

use cgmath::{Point2, Vector3, Point3, InnerSpace, EuclideanSpace};
use rand::random;

use super::{bound3::Bound3, shape::Shape, ray::{Ray, Beam}, interaction::SurfaceInteraction};

pub fn object_bound(disk: &Shape) -> Bound3 {
    if let Shape::Disk { object_to_world, world_to_object, radius } = disk {
        let p = Point3::new(-radius, -radius, 0.0);
        let q = Point3::new(*radius, *radius, 0.0);

        Bound3::new(p, q)
    } else {
        panic!()
    }
}

pub fn world_bound(disk: &Shape) -> Bound3 {
    if let Shape::Disk { object_to_world, world_to_object, radius } = disk {
        object_to_world.transform_bound3(&object_bound(disk))
    } else {
        panic!()
    }
}

pub fn intersect(disk: &Shape, r: &Ray) -> Option<SurfaceInteraction> {
    if let Shape::Disk { object_to_world, world_to_object, radius } = disk {
        let r = world_to_object.transform_ray(r);

        if r.d.z == 0.0 { return None; } // parallel to the disk

        let t = -r.o.z / r.d.z;
        let p = r.at(t);
        // hit the plane

        // check if it is inside radius
        if t <= 0.0001 || t >= r.t_max || p.to_vec().magnitude2() > radius * radius {
            return None;
        }

        let n = if r.d.z > 0.0 { Vector3::new(0.0, 0.0, -1.0) } else { Vector3::new(0.0, 0.0, 1.0) };

        let isect = SurfaceInteraction {
            p, 
            n,
            t,
            time: r.time,
            wo: -r.d.normalize(),
            material: None,
            hit_light: false,
            radiance: None,
        };

        let isect = object_to_world.transform_surface_interaction(&isect);

        Some(isect)
    } else {
        panic!()
    }
}

pub fn intersect_p(disk: &Shape, r: &Ray) -> Option<f64> {
    if let Shape::Disk { object_to_world, world_to_object, radius } = disk {
        let r = world_to_object.transform_ray(r);

        if r.d.z == 0.0 { return None; } // parallel to the disk

        let t = -r.o.z / r.d.z;
        let p = r.at(t);
        // hit the plane

        // check if it is inside radius
        if t <= 0.0001 || t >= r.t_max || p.to_vec().magnitude2() > radius * radius {
            return None;
        }

        Some(t)
    } else {
        panic!()
    }
}

pub fn area(disk: &Shape) -> f64 {
    if let Shape::Disk { object_to_world, world_to_object, radius } = disk {
        PI * radius * radius
    } else {
        panic!()
    }
}

pub fn sample(disk: &Shape, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64) {
    if let Shape::Disk { object_to_world, world_to_object, radius } = disk {
        let r = radius * u[0].sqrt();
        let theta = u[1] * PI * 2.0;

        let p = Point3::new(r * theta.cos(), r * theta.sin(), 0.0);
        let n = Vector3::new(0.0, 0.0, 1.0);
        let area_pdf = 1.0 / area(disk);

        let p = object_to_world.transform_point3(p);
        let n = object_to_world.transform_vector3(n);

        (p, n, area_pdf)
    } else {
        panic!()
    }
}