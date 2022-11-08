use std::{f64::INFINITY, rc::Rc};

use crate::geometry::shape::Shape;
use crate::material::Material;
use crate::{light::Light, geometry::ray::Ray, spectrum::Spectrum};
use crate::Transform;
use super::Primitive;
use cgmath::{Vector3, Point3};

pub struct Scene {
    pub lights: Vec<Light>,
    pub primitives: Vec<Primitive>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            primitives: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
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
        let sphere = Shape::create_sphere(2.0, object_to_world, world_to_object);
        //// create lambertian material
        let matte_material = Material::create_matte(Spectrum::new(1.0, 0.6, 0.2));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Shape::create_sphere(100.0, object_to_world2, world_to_object2);
        //// create lambertian material
        let matte_material2 = Material::create_matte(Spectrum::new(0.4, 0.4, 0.5));
        let ball2 = Primitive::create_geometric_primitive(sphere,matte_material2);
        scene.add_primitive(ball2);

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Shape::create_cylinder(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        //// create lambertian material
        let matte_material3 = Material::create_matte(Spectrum::new(0.8, 0.0, 0.0));
        let cylinder = Primitive::create_geometric_primitive(cylinder,matte_material3);
        scene.add_primitive(cylinder);

        // create light
        let p_light = Light::create_point_light(Point3::new(2.0, 4.0, 4.0), Spectrum::new(10.0, 10.0, 10.0));
        scene.add_light(p_light);

        let p_light2 = Light::create_point_light(Point3::new(-2.0, 4.0, 4.0), Spectrum::new(10.0, 5.0, 10.0));
        scene.add_light(p_light2);

        let p_light3 = Light::create_point_light(Point3::new(0.0, 8.0, 3.0), Spectrum::new(10.0, 5.0, 10.0));
        scene.add_light(p_light3);

        scene
    }

    pub fn world_two() -> Scene {
        let mut scene = Scene::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(2.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(1.0, 0.6, 0.2));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Shape::create_sphere(100.0, object_to_world2, world_to_object2);
        let matte_material2 = Material::create_glass(1.0, 1.5, Spectrum::new(0.6, 0.6, 0.6), Spectrum::new(0.6, 0.6, 0.6));
        let ball2 = Primitive::create_geometric_primitive(sphere, matte_material2);
        scene.add_primitive(ball2);

        let object_to_world4 = Transform::translate(Vector3::new(0.0, -20.0, 10.0));
        let world_to_object4= object_to_world4.inverse();
        let sphere = Shape::create_sphere(2.5, object_to_world4, world_to_object4);
        let matte_material4 = Material::create_matte(Spectrum::new(0.8, 0.8, 0.8));
        let ball4 = Primitive::create_geometric_primitive(sphere, matte_material4);
        scene.add_primitive(ball4);

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Shape::create_cylinder(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        let matte_material3 = Material::create_matte(Spectrum::new(0.8, 0.0, 0.0));
        let cylinder = Primitive::create_geometric_primitive(cylinder, matte_material3);
        scene.add_primitive(cylinder);

        // create light
        let p_light = Light::create_point_light(Point3::new(2.0, 4.0, 4.0), Spectrum::new(10.0, 10.0, 10.0));
        scene.add_light(p_light);

        scene
    }

    pub fn world_three() -> Scene {
        let mut scene = Scene::new();
    // create ball
    // create sphere
        let object_to_world = Transform::translate(Vector3::new(0.0, 4.0, 6.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(2.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(1.0, 0.6, 0.2));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world2 = Transform::translate(Vector3::new(0.0, -100.0, 6.0));
        let world_to_object2= object_to_world2.inverse();
        let sphere = Shape::create_sphere(100.0, object_to_world2, world_to_object2);
        let matte_material2 = Material::create_glass(1.0, 1.5, Spectrum::new(0.6, 0.6, 0.6), Spectrum::new(0.6, 0.6, 0.6));
        let ball2 = Primitive::create_geometric_primitive(sphere, matte_material2);
        scene.add_primitive(ball2);

        let object_to_world3 = Transform::translate(Vector3::new(0.0, 0.0, 6.0)) * Transform::rotate_x(90.0);
        let world_to_object3= object_to_world3.inverse();
        let cylinder = Shape::create_cylinder(object_to_world3, world_to_object3, 2.0, 2.0, 0.0);
        let matte_material3 = Material::create_matte(Spectrum::new(0.8, 0.0, 0.0));
        let cylinder = Primitive::create_geometric_primitive(cylinder, matte_material3);
        scene.add_primitive(cylinder);

        // create light
        let object_to_world4 = Transform::translate(Vector3::new(1.0, 1.0, 3.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Shape::create_sphere(1.0, object_to_world4, world_to_object4);
        let sphere_light = Light::create_area_light(sphere2, Spectrum::new(1.0, 1.0, 1.0));
        scene.add_light(sphere_light);
    
        scene
    }

    pub fn cornell_box() -> Scene {
        let mut scene = Scene::new();

        let object_to_world = Transform::translate(Vector3::new(1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.65, 0.05, 0.05));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(-1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.12, 0.45, 0.15));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 1000.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(900.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.73, 0.73, 0.73));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, -1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.73, 0.73, 0.73));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);


        let object_to_world = Transform::translate(Vector3::new(0.0, 1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.8, 0.8, 0.8));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, -3.0, 25.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(10.0, object_to_world, world_to_object);
        let matte_material = Material::create_glass(1.0, 1.5, Spectrum::new(0.8, 0.8, 0.8), Spectrum::new(0.8, 0.8, 0.8));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 20.0, 25.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Shape::create_sphere(2.0, object_to_world4, world_to_object4);
        let sphere_light = Light::create_area_light(sphere2, Spectrum::new(500.0, 500.0, 500.0));
        scene.add_light(sphere_light);


        scene
    }

    pub fn nested_glass() -> Scene {
        let mut scene = Scene::new();

        let object_to_world = Transform::translate(Vector3::new(1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.65, 0.05, 0.05));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(-1000.0, 0.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.12, 0.45, 0.15));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 1000.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(900.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.73, 0.73, 0.73));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, -1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.73, 0.73, 0.73));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);


        let object_to_world = Transform::translate(Vector3::new(0.0, 1000.0, 3.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(950.0, object_to_world, world_to_object);
        let matte_material = Material::create_matte(Spectrum::new(0.8, 0.8, 0.8));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, -3.0, 25.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(10.0, object_to_world, world_to_object);
        let matte_material = Material::create_glass(1.0, 1.5, Spectrum::new(0.8, 0.8, 0.8), Spectrum::new(0.8, 0.8, 0.8));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world = Transform::translate(Vector3::new(0.0, -3.0, 25.0));
        let world_to_object = object_to_world.inverse();
        let sphere = Shape::create_sphere(7.0, object_to_world, world_to_object);
        let matte_material = Material::create_glass(1.5, 1.0, Spectrum::new(0.8, 0.8, 0.8), Spectrum::new(0.8, 0.8, 0.8));
        let ball = Primitive::create_geometric_primitive(sphere, matte_material);
        scene.add_primitive(ball);

        let object_to_world4 = Transform::translate(Vector3::new(0.0, 20.0, 25.0));
        let world_to_object4 = object_to_world4.inverse();
        let sphere2 = Shape::create_sphere(2.0, object_to_world4, world_to_object4);
        let sphere_light = Light::create_area_light(sphere2, Spectrum::new(500.0, 500.0, 500.0));
        scene.add_light(sphere_light);


        scene
    }
}