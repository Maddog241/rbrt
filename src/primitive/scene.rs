use std::{f64::INFINITY, rc::Rc};

use crate::{light::{Light, point_light::PointLight, area_light::AreaLight}, geometry::{ray::Ray, sphere::Sphere, cylinder::Cylinder}, material::{matte::Matte, glass::Glass}, spectrum::Spectrum};
use crate::Transform;
use super::{Primitive, geometric_primitive::GeometricPrimitive};
use cgmath::{Vector3, Point3};

pub struct Scene {
    pub lights: Vec<Box<dyn Light>>,
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            primitives: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_primitive(&mut self, primitive: Box<dyn Primitive>) {
        self.primitives.push(primitive);
    }

    pub fn intersect(&self, r: &mut crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        let mut ret = None;        
        for prim in self.primitives.iter() {
            if let Some(isect) = prim.intersect(r) {
                ret = Some(isect);
            }
        }

        for light in self.lights.iter() {
            if let Some(isect) = light.intersect(r) {
                ret = Some(isect);
            }
        }

        ret
    }

    pub fn intersect_p(&self, r: &Ray) -> Option<f64> {
        let mut t = INFINITY;
        for prim in self.primitives.iter() {
            if let Some(new_t) = prim.intersect_p(r) {
                t = t.min(new_t);
            }
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
        let mut scene = Scene::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(2.0, object_to_world, world_to_object);
        //// create lambertian material
        let matte_material = Matte::new(Spectrum::new(1.0, 0.6, 0.2));
        let ball = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material));
        scene.add_primitive(Box::new(ball));

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Sphere::new(100.0, object_to_world2, world_to_object2);
        //// create lambertian material
        let matte_material2 = Matte::new(Spectrum::new(0.4, 0.4, 0.5));
        let ball2 = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material2));
        scene.add_primitive(Box::new(ball2));

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Cylinder::new(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        //// create lambertian material
        let matte_material3 = Matte::new(Spectrum::new(0.8, 0.0, 0.0));
        let cylinder = GeometricPrimitive::new(Box::new(cylinder), Rc::new(matte_material3));
        scene.add_primitive(Box::new(cylinder));

        // create light
        let p_light = PointLight::new(Point3::new(2.0, 4.0, 4.0), Spectrum::new(10.0, 10.0, 10.0));
        scene.add_light(Box::new(p_light));

        let p_light2 = PointLight::new(Point3::new(-2.0, 4.0, 4.0), Spectrum::new(10.0, 5.0, 10.0));
        scene.add_light(Box::new(p_light2));

        let p_light3 = PointLight::new(Point3::new(0.0, 8.0, 3.0), Spectrum::new(10.0, 5.0, 10.0));
        scene.add_light(Box::new(p_light3));

        scene
    }

    pub fn world_two() -> Scene {
        let mut scene = Scene::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(2.0, object_to_world, world_to_object);
        let matte_material = Matte::new(Spectrum::new(1.0, 0.6, 0.2));
        let ball = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material));
        scene.add_primitive(Box::new(ball));

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Sphere::new(100.0, object_to_world2, world_to_object2);
        let matte_material2 = Glass::new(1.0, 1.5, Spectrum::new(0.6, 0.6, 0.6), Spectrum::new(0.6, 0.6, 0.6));
        let ball2 = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material2));
        scene.add_primitive(Box::new(ball2));

        let object_to_world4 = Transform::translate(Vector3::new(0.0, -20.0, 10.0));
        let world_to_object4= object_to_world4.inverse();
        let sphere = Sphere::new(2.5, object_to_world4, world_to_object4);
        let matte_material4 = Matte::new(Spectrum::new(0.8, 0.8, 0.8));
        let ball4 = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material4));
        scene.add_primitive(Box::new(ball4));

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Cylinder::new(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        let matte_material3 = Matte::new(Spectrum::new(0.8, 0.0, 0.0));
        let cylinder = GeometricPrimitive::new(Box::new(cylinder), Rc::new(matte_material3));
        scene.add_primitive(Box::new(cylinder));

        // create light
        let p_light = PointLight::new(Point3::new(2.0, 4.0, 4.0), Spectrum::new(10.0, 10.0, 10.0));
        scene.add_light(Box::new(p_light));

        scene
    }

    pub fn world_three() -> Scene {
        let mut scene = Scene::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Sphere::new(2.0, object_to_world, world_to_object);
        let matte_material = Matte::new(Spectrum::new(1.0, 0.6, 0.2));
        let ball = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material));
        scene.add_primitive(Box::new(ball));

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Sphere::new(100.0, object_to_world2, world_to_object2);
        let matte_material2 = Glass::new(1.0, 1.5, Spectrum::new(0.6, 0.6, 0.6), Spectrum::new(0.6, 0.6, 0.6));
        let ball2 = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material2));
        scene.add_primitive(Box::new(ball2));

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Cylinder::new(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        let matte_material3 = Matte::new(Spectrum::new(0.8, 0.0, 0.0));
        let cylinder = GeometricPrimitive::new(Box::new(cylinder), Rc::new(matte_material3));
        scene.add_primitive(Box::new(cylinder));

        // create light
        let object_to_world4 = Transform::translate(Vector3::new(1.0, 1.0, 3.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Sphere::new(1.0, object_to_world4, world_to_object4);
        let sphere_light = AreaLight::new(Box::new(sphere2), Spectrum::new(1.0, 1.0, 1.0));
        scene.add_light(Box::new(sphere_light));
    
        scene
    }
}