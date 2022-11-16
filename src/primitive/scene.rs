use std::f64::INFINITY;
use std::sync::Arc;

use crate::accelerator::bvh::BVH;
use crate::geometry::cylinder::Cylinder;
use crate::geometry::disk::Disk;
use crate::geometry::sphere::Sphere;
use crate::light::area::AreaLight;
use crate::light::point::PointLight;
use crate::material::glass::Glass;
use crate::material::matte::Matte;
use crate::texture::constant::ConstantTexture;
use crate::texture::imagemap::{ImageTexture, Texels};
use crate::texture::mapping::spherical::SphericalMapping;
use crate::{light::Light, geometry::ray::Ray, spectrum::Spectrum};
use crate::Transform;
use super::Primitive;
use super::geometric_primitive::GeometricPrimitive;
use cgmath::{Vector3, Point3};
use rand::random;

pub struct Scene {
    pub lights: Vec<Box<dyn Light>>,
    pub aggregate: Box<dyn Primitive>,
}

#[allow(dead_code)]
impl Scene {
    pub fn new(lights: Vec<Box<dyn Light>>, aggregate: Box<dyn Primitive>) -> Self {
        Scene {
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
            t = t.min(new_t);
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

    pub fn world_one() -> Scene {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();
        // create ball
        // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 2.0);
        //// create lambertian material
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(1.0, 0.6, 0.2))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Sphere::new(object_to_world2, world_to_object2, 100.0);
        //// create lambertian material
        let matte_material2 = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.4, 0.4, 0.5))));
        let ball2 = GeometricPrimitive::new(Box::new(sphere),Arc::new(matte_material2));
        primitives.push(Box::new(ball2));

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Cylinder::new(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        //// create lambertian material
        let matte_material3 = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.0, 0.0))));
        let cylinder = GeometricPrimitive::new(Box::new(cylinder), Arc::new(matte_material3));
        primitives.push(Box::new(cylinder));

        // create light
        let p_light = PointLight::new(Point3::new(2.0, 4.0, 4.0), Spectrum::new(10.0, 10.0, 10.0));
        lights.push(Box::new(p_light));

        let p_light2 = PointLight::new(Point3::new(-2.0, 4.0, 4.0), Spectrum::new(10.0, 5.0, 10.0));
        lights.push(Box::new(p_light2));

        let p_light3 = PointLight::new(Point3::new(0.0, 8.0, 3.0), Spectrum::new(10.0, 5.0, 10.0));
        lights.push(Box::new(p_light3));

        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn world_two() -> Scene {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new( object_to_world, world_to_object, 2.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(1.0, 0.6, 0.2))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Sphere::new( object_to_world2, world_to_object2, 100.0);
        let matte_material2 = Glass::new(1.0, 1.5, Spectrum::new(0.6, 0.6, 0.6), Spectrum::new(0.6, 0.6, 0.6));
        let ball2 = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material2));
        primitives.push(Box::new(ball2));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, -20.0, 10.0));
        let world_to_object4= object_to_world4.inverse();
        let sphere = Sphere::new( object_to_world4, world_to_object4, 2.5);
        let matte_material4 = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
        let ball4 = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material4));
        primitives.push(Box::new(ball4));

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Cylinder::new(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        let matte_material3 = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.0, 0.0))));
        let cylinder = GeometricPrimitive::new(Box::new(cylinder), Arc::new(matte_material3));
        primitives.push(Box::new(cylinder));

        // create light
        let p_light = PointLight::new(Point3::new(2.0, 4.0, 4.0), Spectrum::new(10.0, 10.0, 10.0));
        lights.push(Box::new(p_light));

        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn world_three() -> Scene {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new( object_to_world, world_to_object, 2.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(1.0, 0.6, 0.2))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Sphere::new(object_to_world2, world_to_object2, 100.0);
        let matte_material2 = Glass::new(1.0, 1.5, Spectrum::new(0.6, 0.6, 0.6), Spectrum::new(0.6, 0.6, 0.6));
        let ball2 = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material2));
        primitives.push(Box::new(ball2));

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Cylinder::new(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        let matte_material3 = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.0, 0.0))));
        let cylinder = GeometricPrimitive::new(Box::new(cylinder), Arc::new(matte_material3));
        primitives.push(Box::new(cylinder));

        // create light
        let object_to_world4 = Transform::translate(Vector3::new(1.0, 1.0, 3.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Sphere::new(object_to_world4, world_to_object4, 1.0);
        let sphere_light = AreaLight::new(Box::new(sphere2), Spectrum::new(1.0, 1.0, 1.0));
        lights.push(Box::new(sphere_light));
    
        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn cornell_box() -> Scene {
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

        let object_to_world = Transform::translate(Vector3::new(0.0, -6.0, 20.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 3.0);
        let matte_material = Glass::new(1.0, 1.5, Spectrum::new(0.8, 0.8, 0.8), Spectrum::new(1.0, 1.0, 1.0));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, -6.0, 20.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 1.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.5, 0.1, 0.1))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.99, 20.0)) * Transform::rotate_x(90.0);
        let world_to_object4 = object_to_world4.inverse();
        let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
        let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
        lights.push(Box::new(disk_light));


        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn nested_glass() -> Scene {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        let object_to_world = Transform::translate(Vector3::new(1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(-1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new( object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 1000.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 900.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, -1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));


        let object_to_world = Transform::translate(Vector3::new(0.0, 1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new( object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, -3.0, 25.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 10.0);
        let matte_material = Glass::new(1.0, 1.5, Spectrum::new(0.8, 0.8, 0.8), Spectrum::new(0.8, 0.8, 0.8));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, -3.0, 25.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 7.0);
        let matte_material = Glass::new(1.5, 1.0, Spectrum::new(0.8, 0.8, 0.8), Spectrum::new(0.8, 0.8, 0.8));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 20.0, 25.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Sphere::new(object_to_world4, world_to_object4, 2.0);
        let sphere_light = AreaLight::new(Box::new(sphere2), Spectrum::new(500.0, 500.0, 500.0));
        lights.push(Box::new(sphere_light));


        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn cornell_disk() -> Scene {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        let object_to_world = Transform::translate(Vector3::new(1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new( object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(-1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 1000.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 900.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, -1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));


        let object_to_world = Transform::translate(Vector3::new(0.0, 1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world, world_to_object, 950.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 25.0)) * Transform::rotate_x(90.0);
        let world_to_object = object_to_world.inverse();
        let disk = Disk::new(object_to_world, world_to_object, 5.0);
        let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.4, 0.4, 0.4))));
        let disk = GeometricPrimitive::new(Box::new(disk), Arc::new(matte_material));
        primitives.push(Box::new(disk));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 20.0, 25.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Sphere::new(object_to_world4, world_to_object4, 2.0);
        let sphere_light = AreaLight::new(Box::new(sphere2), Spectrum::new(500.0, 500.0, 500.0));
        lights.push(Box::new(sphere_light));


        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn test_texture() -> Scene {
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

        let object_to_world = Transform::translate(Vector3::new(0.0, -4.0, 20.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world.clone(), world_to_object, 4.0);
        let world_to_texture = (object_to_world * Transform::rotate_x(90.0)).inverse();
        let mapping = SphericalMapping::new(world_to_texture);
        let matte_material = Matte::new(Box::new(ImageTexture::new(Box::new(mapping), Texels::new("./earthmap.jpg"))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.99, 20.0)) * Transform::rotate_x(90.0);
        let world_to_object4 = object_to_world4.inverse();
        let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
        let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
        lights.push(Box::new(disk_light));


        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }

    pub fn cornell_box2() -> Scene {
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

        let object_to_world = Transform::translate(Vector3::new(0.0, -4.0, 20.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(object_to_world.clone(), world_to_object, 4.0);
        let world_to_texture = (object_to_world * Transform::rotate_x(90.0)).inverse();
        let mapping = SphericalMapping::new(world_to_texture);
        let matte_material = Matte::new(Box::new(ImageTexture::new(Box::new(mapping), Texels::new("./earthmap.jpg"))));
        let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
        primitives.push(Box::new(ball));

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


        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }
    
    
    pub fn sphere_100() -> Scene {
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


        let bvh = BVH::new(primitives);
        let scene = Scene::new(lights, Box::new(bvh));

        scene
    }   
}