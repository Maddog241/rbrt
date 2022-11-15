use std::f64::INFINITY;
use std::sync::Arc;

use crate::accelerator::bvh::BVH;
use crate::geometry::cylinder::Cylinder;
use crate::geometry::disk::Disk;
use crate::geometry::sphere::Sphere;
use crate::light::area::AreaLight;
use crate::material::matte::Matte;
use crate::texture::constant::ConstantTexture;
use crate::{light::Light, geometry::ray::Ray, spectrum::Spectrum};
use crate::Transform;
use super::Primitive;
use super::geometric_primitive::GeometricPrimitive;
use cgmath::Vector3;
use rand::random;

pub struct BoundScene {
    pub lights: Vec<Box<dyn Light>>,
    pub aggregate: BVH,
}


impl BoundScene {
    pub fn new(lights: Vec<Box<dyn Light>>, aggregate: BVH) -> Self {
        BoundScene {
            lights,
            aggregate,
        }
    }

    pub fn intersect(&self, r: &mut crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        let mut ret = self.aggregate.intersect(r);

        for light in self.lights.iter() {
            if let Some(isect) = light.intersect(r) {
                ret = Some(isect);
            }
        }

        ret
    }

    pub fn intersect_p(&self, r: &Ray) -> Option<f64> {
        let mut t = INFINITY;
        
        if let Some(new_t) = self.aggregate.intersect_p(r) {
            t = new_t;
        }

        for light in self.lights.iter() {
            if let Some(new_t) = light.intersect_p(r) {
                t = t.min(new_t);
            }
        }

        if t == INFINITY {
            None
        } else {
            Some(t)
        }
    }

    pub fn sphere_100() -> (Vec<Box<dyn Primitive>>, Vec<Box<dyn Light>>) {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
        let world_to_object = object_to_world.inverse();
        let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
        let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
        primitives.push(Box::new(right_wall));

        let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
        let world_to_object = object_to_world.inverse();
        let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
        let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
        primitives.push(Box::new(left_wall));

        let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
        let world_to_object = object_to_world.inverse();
        let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
        let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
        primitives.push(Box::new(back_wall));

        let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
        let world_to_object = object_to_world.inverse();
        let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
        let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
        primitives.push(Box::new(upper_wall));


        let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(-90.0);
        let world_to_object = object_to_world.inverse();
        let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
        let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
        primitives.push(Box::new(bot_wall));

        // create 100 spheres inside the box
        let z_min = 18.0;
        let z_max = 28.0;
        let y_min = -9.0;
        let y_max = 9.0;
        let x_min = -7.0;
        let x_max = 7.0;

        for _i in 0..100 {
            let x = x_min + (x_max - x_min) * random::<f64>();
            let y = y_min + (y_max - y_min) * random::<f64>();
            let z = z_min + (z_max - z_min) * random::<f64>();
            let object_to_world = Transform::translate(Vector3::new(x, y, z));
            let world_to_object = object_to_world.inverse();
            let sphere = Sphere::new(object_to_world.clone(), world_to_object, 0.2);
            let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(random(), random(), random()))));
            let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
            primitives.push(Box::new(ball));
        }

        // lights

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.9, 20.0)) * Transform::rotate_x(90.0);
        let world_to_object4 = object_to_world4.inverse();
        let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
        let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
        lights.push(Box::new(disk_light));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.9, 20.0)) * Transform::rotate_x(90.0);
        let world_to_object4 = object_to_world4.inverse();
        let disk_light = Cylinder::new(object_to_world4, world_to_object4, 3.0, 0.0, 0.3);
        let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
        lights.push(Box::new(disk_light));


        (primitives, lights)
    }
}