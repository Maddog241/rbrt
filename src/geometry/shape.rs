use cgmath::{Point2, Point3, Vector3};

use super::bound3::Bound3;
use super::disk;
use super::interaction::SurfaceInteraction;
use super::ray::Ray;
use super::transform::Transform;

use super::sphere;
use super::cylinder;


pub enum Shape {
    Sphere {
        radius: f64,
        object_to_world: Transform,
        world_to_object: Transform,
    },
    Cylinder {
        object_to_world: Transform,
        world_to_object: Transform,
        radius: f64,
        z_max: f64,
        z_min: f64,
    },
    Disk {
        object_to_world: Transform,
        world_to_object: Transform,
        radius: f64,
    }
}

#[allow(dead_code)]
impl Shape {
    pub fn object_bound(&self) -> Bound3 {
        match self {
            Self::Sphere { radius:_, object_to_world:_, world_to_object:_ } => {
                sphere::object_bound(self)
            },

            Self::Cylinder { object_to_world:_, world_to_object:_, radius:_, z_max:_, z_min:_ } => {
                cylinder::object_bound(self)
            },

            Self::Disk { object_to_world:_, world_to_object:_, radius:_ } => {
                disk::object_bound(self)
            }
        }
    }

    pub fn world_bound(&self) -> Bound3 {
        match self {
            Self::Sphere { radius:_, object_to_world:_, world_to_object:_ } => {
                sphere::world_bound(self)
            } ,

            Self::Cylinder { object_to_world:_, world_to_object:_, radius:_, z_max:_, z_min:_ } => {
                cylinder::world_bound(self)
            },

            Self::Disk { object_to_world:_, world_to_object:_, radius:_ } => {
                disk::world_bound(self)
            }
        }
    }

    pub fn intersect(&self, r: &Ray) -> Option<SurfaceInteraction> {
        match self {
            Self::Sphere { radius:_, object_to_world:_, world_to_object:_ } => {
                sphere::intersect(self, r)
            },

            Self::Cylinder { object_to_world:_, world_to_object:_, radius:_, z_max:_, z_min:_ } => {
                cylinder::intersect(self, r)
            },

            Self::Disk { object_to_world:_, world_to_object:_, radius:_ } => {
                disk::intersect(self, r)
            }
        }
    }

    pub fn intersect_p(&self, r: &Ray) -> Option<f64> {
        match self {
            Self::Sphere { radius:_, object_to_world:_, world_to_object:_ } => {
                sphere::intersect_p(self, r)
            },

            Self::Cylinder { object_to_world:_, world_to_object:_, radius:_, z_max:_, z_min:_ } => {
                cylinder::intersect_p(self, r)
            },

            Self::Disk { object_to_world:_, world_to_object:_, radius:_ } => {
                disk::intersect_p(self, r)
            }
        }
    }

    pub fn area(&self) -> f64 {
        match self {
            Self::Sphere { radius:_, object_to_world:_, world_to_object:_ } => {
                sphere::area(self)
            },

            Self::Cylinder { object_to_world:_, world_to_object:_, radius:_, z_max:_, z_min:_ } => {
                cylinder::area(self)
            },

            Self::Disk { object_to_world:_, world_to_object:_, radius:_ } => {
                disk::area(self)
            }
        }
    }

    pub fn sample(&self, u: Point2<f64>) -> (Point3<f64>, Vector3<f64>, f64) {
        match self {
            Self::Sphere { radius:_, object_to_world:_, world_to_object:_ } => {
                sphere::sample(self, u)
            },

            Self::Cylinder { object_to_world:_, world_to_object:_, radius:_, z_max:_, z_min:_ } => {
                cylinder::sample(self, u)
            },

            Self::Disk { object_to_world:_, world_to_object:_, radius:_ } => {
                disk::sample(self, u)
            }
        }
    }
}

impl Shape {
    pub fn create_sphere( object_to_world: Transform, world_to_object: Transform, radius: f64) -> Shape {
        Shape::Sphere { radius, object_to_world, world_to_object }
    }

    pub fn create_cylinder(object_to_world: Transform, world_to_object: Transform, radius: f64, z_max: f64, z_min: f64) -> Shape {
        Shape::Cylinder { object_to_world, world_to_object, radius, z_max, z_min }
    }

    pub fn create_disk(object_to_world: Transform, world_to_object: Transform, radius: f64) -> Shape {
        Shape::Disk { object_to_world, world_to_object, radius }
    }
}