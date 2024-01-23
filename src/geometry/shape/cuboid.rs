use cgmath::{InnerSpace, Point3, Vector3};

use crate::geometry::{bound3::Bound3, interaction::GeometryInfo, ray::Beam, transform::Transform};

use super::Shape;

pub struct Cuboid {
    object_to_world: Transform,
    world_to_object: Transform,
    half_x: f64,
    half_y: f64,
    half_z: f64,
}

impl Cuboid {
    pub fn new(half_x: f64, half_y: f64, half_z: f64, object_to_world: Transform) -> Self {
        let world_to_object = object_to_world.inverse();
        Self {
            object_to_world,
            world_to_object,
            half_x,
            half_y,
            half_z
        }
    }

    // return the area of the face facing toward x+ direction
    fn area_x(&self) -> f64 {
        let size_y = 2.0 * self.half_y;
        let size_z = 2.0 * self.half_z;

        size_y * size_z
    }

    fn area_y(&self) -> f64 {
        let size_x = 2.0 * self.half_x;
        let size_z = 2.0 * self.half_z;

        size_x * size_z
    }

    fn area_z(&self) -> f64 {
        let size_x = 2.0 * self.half_x;
        let size_y = 2.0 * self.half_y;

        size_x * size_y
    }
}

impl Shape for Cuboid {
    fn object_bound(&self) -> crate::geometry::bound3::Bound3 {
        let p = Point3::new(-self.half_x, -self.half_y, -self.half_z);
        let q = Point3::new(self.half_x, self.half_y, self.half_z);
        Bound3::new(p, q)
    }

    fn world_bound(&self) -> crate::geometry::bound3::Bound3 {
        self.object_to_world.transform_bound3(&self.object_bound())
    }

    fn intersect(&self, r: &crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::GeometryInfo> {
        match self.intersect_p(r) {
            None => None, 
            Some(t) => {
                let local_r = self.world_to_object.transform_ray(r);
                let local_p = local_r.at(t);
                let local_n;

                if 1e-3 > (local_p.x.abs() - self.half_x).abs() {
                    // p is on the x plane
                    if local_p.x > 0.0 { local_n = Vector3::new(1.0, 0.0, 0.0); } 
                    else { local_n = Vector3::new(-1.0, 0.0, 0.0); }
                } else if 1e-3 > (local_p.y.abs() - self.half_y).abs() {
                    // p is on the y plane
                    if local_p.y > 0.0 { local_n = Vector3::new(0.0, 1.0, 0.0); }
                    else { local_n = Vector3::new(0.0, -1.0, 0.0); }
                } else {
                    // p is on the z plane
                    if local_p.z > 0.0 { local_n = Vector3::new(0.0, 0.0, 1.0); }
                    else { local_n = Vector3::new(0.0, 0.0, -1.0); }
                }

                Some(GeometryInfo {
                    p: self.object_to_world.transform_point3(local_p),
                    n: self.object_to_world.transform_normal(local_n),
                    t,
                    wo: -r.d.normalize()
                })
            }
        }
    }

    // make use of the bounding box
    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        let local_r = self.world_to_object.transform_ray(r);
        let bound = self.object_bound();
        match bound.intersect_p(&local_r) {
            Some(t) => Some(t.0),
            None => None,
        }
    }

    fn area(&self) -> f64 {
        let mut area = 0.0;
        area += 2.0 * self.area_x();
        area += 2.0 * self.area_y();
        area += 2.0 * self.area_z();

        area
    }

    fn uniform_sample_point(&self, u: cgmath::Point2<f64>) -> (cgmath::Point3<f64>, cgmath::Vector3<f64>, f64) {
        // choose direction 
        let (ax, ay, az) = (self.area_x(), self.area_y(), self.area_z());
        let tot = ax + ay + az;
        let (mut u, mut v) = (u[0], u[1]);

        let axis: usize = if u < ax / tot {
            u *= tot / ax;
            0
        } else if u < (ax + ay) / tot {
            u *= tot / ay;
            1
        } else {
            u *= tot / az;
            2
        };

        // use v to choose face (positive or negative)
        let positive = if v < 0.5 { 
            v *= 2.0;
            true 
        } else { 
            v = (v-0.5) * 2.0;
            false 
        };

        // sample on this plane
        match axis {
            0 => {
                let x = if positive { self.half_x } else { -self.half_x };
                let y = -self.half_y + 2.0 * self.half_y * u;
                let z = -self.half_z + 2.0 * self.half_z * v;
                let local_p = Point3::new(x, y, z);

                let mut local_n = Vector3::new(1.0, 0.0, 0.0);
                if !positive { local_n *= -1.0; }

                let world_p = self.object_to_world.transform_point3(local_p);
                let world_n = self.object_to_world.transform_normal(local_n);
                let pdf = (ax / tot) / (2.0 * ax);

                (world_p, world_n, pdf)
            },
            1 => {
                let y = if positive { self.half_y } else { -self.half_y };
                let x = -self.half_x + 2.0 * self.half_x * u;
                let z = -self.half_z + 2.0 * self.half_z * v;
                let local_p = Point3::new(x, y, z);

                let mut local_n = Vector3::new(0.0, 1.0, 0.0);
                if !positive { local_n *= -1.0; }

                let world_p = self.object_to_world.transform_point3(local_p);
                let world_n = self.object_to_world.transform_normal(local_n);
                let pdf = (ay / tot) / (2.0 * ay);

                (world_p, world_n, pdf)
            },

            2 => {
                let z = if positive { self.half_z } else { -self.half_z };
                let x = -self.half_x + 2.0 * self.half_x * u;
                let y = -self.half_y + 2.0 * self.half_y * v;
                let local_p = Point3::new(x, y, z);

                let mut local_n = Vector3::new(0.0, 0.0, 1.0);
                if !positive { local_n *= -1.0; }

                let world_p = self.object_to_world.transform_point3(local_p);
                let world_n = self.object_to_world.transform_normal(local_n);
                let pdf = (az / tot) / (2.0 * az);

                (world_p, world_n, pdf)
            },

            _ => panic!(),
        }
    }
}