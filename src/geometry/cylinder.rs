use std::f64::consts::PI;

use cgmath::{Vector3, Point3};

use super::{shape::Shape, interaction::SurfaceInteraction, ray::{Ray, Beam}, bound3::Bound3, transform::Transform};

pub struct Cylinder {
    object_to_world: Transform,
    world_to_object: Transform,
    radius: f64,
    z_max: f64,
    z_min: f64,
}

impl Cylinder {
    pub fn new(object_to_world: Transform, world_to_object: Transform, radius: f64, z_max: f64, z_min: f64) -> Self {
        if z_max < z_min {
            eprintln!("[cyliner initialization]: z_max smaller than z_min, switched automatically");
            Cylinder {object_to_world, world_to_object, radius, z_min: z_max, z_max: z_min}
        } else {
            Cylinder {object_to_world, world_to_object, radius, z_max, z_min}
        }
        
    }
}

impl Shape for Cylinder {
    fn object_bound(&self) -> super::bound3::Bound3 {
        Bound3::new(
            Point3::new(-self.radius, -self.radius, self.z_min),
            Point3::new(self.radius, self.radius, self.z_max),
        )
    }

    fn world_bound(&self) -> super::bound3::Bound3 {
        self.object_to_world.transform_bound3(&self.object_bound())
    }

    fn intersect(&self, r: &Ray) -> Option<SurfaceInteraction> {
        // transform the ray from world to object
        let r = self.world_to_object.transform_ray(r);

        // compute the quadric coefficients
        let a = r.d.x * r.d.x + r.d.y * r.d.y;
        let b = 2.0 * (r.o.x * r.d.x + r.o.y + r.d.y);
        let c = r.o.x * r.o.x + r.o.y * r.o.y - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        // check solutions
        if discriminant <= 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);
        let p1 = r.at(t);
        if t < 0.0001 || t > r.t_max || p1.z < self.z_min || p1.z > self.z_max {
            t = (-b + sqrt_discriminant) / (2.0 * a);
            let p2 = r.at(t);
            if t < 0.0001 || t > r.t_max || p2.z < self.z_min || p2.z > self.z_max {
                return None;
            }
        }
        // get the solution t
        let p = r.at(t);
        let n = Vector3::new(p.x, p.y, 0.0);
        let wo = -r.d;
        
        let inter = SurfaceInteraction {
            p,
            n,
            t,
            time: r.time,
            wo,
        };

        // transform the interation back to the world coordinate
        let inter = self.object_to_world.transform_surface_interaction(&inter);
        Some(inter)
    }

    fn intersect_p(&self, r: &Ray) -> Option<f64> {
        let r = self.world_to_object.transform_ray(r);

        // compute the quadric coefficients
        let a = r.d.x * r.d.x + r.d.y * r.d.y;
        let b = 2.0 * (r.o.x * r.d.x + r.o.y + r.d.y);
        let c = r.o.x * r.o.x + r.o.y * r.o.y - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        // check solutions
        if discriminant <= 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);
        let p1 = r.at(t);
        if t < 0.0001 || t > r.t_max || p1.z < self.z_min || p1.z > self.z_max {
            t = (-b + sqrt_discriminant) / (2.0 * a);
            let p2 = r.at(t);
            if t < 0.0001 || t > r.t_max || p2.z < self.z_min || p2.z > self.z_max {
                return None;
            }
        }
        // get the solution t
    
        Some(t)
    }

    fn area(&self) -> f64 {
        2.0 * PI * self.radius * (self.z_max - self.z_min)
    }
}