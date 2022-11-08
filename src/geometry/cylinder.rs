use std::f64::consts::PI;

use cgmath::{Vector3, Point3, InnerSpace, Point2};

use super::{shape::Shape, interaction::SurfaceInteraction, ray::{Ray, Beam}, bound3::Bound3};


pub fn object_bound(cylinder: &Shape) -> super::bound3::Bound3 {
    if let Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min } = cylinder {
        Bound3::new(
            Point3::new(-radius, -radius, *z_min),
            Point3::new(*radius, *radius, *z_max),
        )
    } else {
        panic!()
    }
}

pub fn world_bound(cylinder: &Shape) -> super::bound3::Bound3 {
    if let Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min } = cylinder {
        object_to_world.transform_bound3(&object_bound(cylinder))
    } else {
        panic!()
    }
}

pub fn intersect(cylinder: &Shape, r: &Ray) -> Option<SurfaceInteraction> {
    if let Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min } = cylinder {
        // transform the ray from world to object
        let r = world_to_object.transform_ray(r);

        // compute the quadric coefficients
        let a = r.d.x * r.d.x + r.d.y * r.d.y;
        let b = 2.0 * (r.o.x * r.d.x + r.o.y * r.d.y);
        let c = r.o.x * r.o.x + r.o.y * r.o.y - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        // check solutions
        if discriminant <= 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);
        let p1 = r.at(t);
        if t < 0.0001 || t > r.t_max || p1.z < *z_min || p1.z > *z_max {
            t = (-b + sqrt_discriminant) / (2.0 * a);
            let p2 = r.at(t);
            if t < 0.0001 || t > r.t_max || p2.z < *z_min || p2.z > *z_max {
                return None;
            }
        }
        // get the solution t
        let p = r.at(t);
        let n = Vector3::new(p.x, p.y, 0.0);
        
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

        // transform the interation back to the world coordinate
        let inter = object_to_world.transform_surface_interaction(&inter);
        Some(inter)
    } else {
        panic!()
    }
}

pub fn intersect_p(cylinder: &Shape, r: &Ray) -> Option<f64> {
    if let Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min } = cylinder {
        let r = world_to_object.transform_ray(r);

        // compute the quadric coefficients
        let a = r.d.x * r.d.x + r.d.y * r.d.y;
        let b = 2.0 * (r.o.x * r.d.x + r.o.y * r.d.y);
        let c = r.o.x * r.o.x + r.o.y * r.o.y - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        // check solutions
        if discriminant <= 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);
        let p1 = r.at(t);
        if t < 0.0001 || t > r.t_max || p1.z < *z_min || p1.z > *z_max {
            t = (-b + sqrt_discriminant) / (2.0 * a);
            let p2 = r.at(t);
            if t < 0.0001 || t > r.t_max || p2.z < *z_min || p2.z > *z_max {
                return None;
            }
        }
        // get the solution t
    
        Some(t)
    } else {
        panic!()
    }
}

pub fn area(cylinder: &Shape) -> f64 {
    if let Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min } = cylinder {
        2.0 * PI * radius * (z_max - z_min) // only considering the outfacing side
    } else {
        panic!()
    }
}

pub fn sample(cylinder: &Shape, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64) {
    if let Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min } = cylinder {
        let theta = u[0] * 2.0 * PI;
        let z = u[1] * (z_max - z_min) + z_min;
        let x = radius * theta.cos();
        let y = radius * theta.sin();

        (Point3::new(x, y, z), Vector3::new(x, y, 0.0) / *radius, 1.0 / area(cylinder))
    } else {
        panic!()
    }
}