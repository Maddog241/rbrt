use std::f64::INFINITY;
// use std::sync::Arc;

// use crate::accelerator::bvh::BVH;
// use crate::camera::film::Film;
// use crate::camera::perspective::PerspectiveCamera;
// use crate::geometry::shape::cylinder::Cylinder;
// use crate::geometry::shape::disk::Disk;
// use crate::mesh::TriangleMesh;
// // use crate::geometry::shape::sphere::Sphere;
use crate::light::LightList;
// use crate::light::area::AreaLight;
// use crate::material::glass::Glass;
// use crate::material::matte::Matte;
// use crate::material::plastic::Plastic;
// use crate::primitive::mesh_primitive::MeshPrimitive;
// use crate::texture::constant::ConstantTexture;
// use crate::texture::imagemap::{ImageTexture, Texels};
// use crate::texture::mapping::spherical::SphericalMapping;
// use crate::{light::Light, geometry::ray::Ray, spectrum::Spectrum};
use crate::geometry::ray::Ray;
// use crate::Transform;
use crate::primitive::Primitive;
// use crate::primitive::geometric_primitive::GeometricPrimitive;
// use cgmath::{Vector3, Point2};



pub struct Scene {
    pub lightlist: LightList,
    pub aggregate: Box<dyn Primitive>,
}

#[allow(dead_code)]
impl Scene {
    pub fn new(lightlist: LightList, aggregate: Box<dyn Primitive>) -> Self {
        Scene {
            lightlist,
            aggregate,
        }
    }

    pub fn intersect(&self, r: &mut crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        let mut ret = self.aggregate.intersect(r);

        for light in self.lightlist.lights.iter() {
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

        for light in self.lightlist.lights.iter() {
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


    // pub fn cornell_box() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let mut lights: Vec<Arc<dyn Light>> = Vec::new();

    //     let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
    //     let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(right_wall));

    //     let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
    //     let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(left_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
    //     let world_to_object = object_to_world.inverse();
    //     let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(back_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(upper_wall));


    //     let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(-90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
    //     let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(bot_wall));

    //     let object_to_world = Transform::translate(Vector3::new(-3.0, -6.0, 20.0));
    //     let world_to_object = object_to_world.inverse();
    //     let sphere = Sphere::new(object_to_world, world_to_object, 3.0);
    //     let glass_material = Glass::new(1.0, 1.5, Spectrum::new(1.0, 1.0, 1.0), Spectrum::new(1.0, 1.0, 1.0));
    //     let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(glass_material));
    //     primitives.push(Box::new(ball));

    //     let object_to_world = Transform::translate(Vector3::new(2.0, -8.5, 18.5));
    //     let world_to_object = object_to_world.inverse();
    //     let sphere = Sphere::new(object_to_world, world_to_object, 1.5);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.5, 0.1, 0.1))));
    //     let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
    //     primitives.push(Box::new(ball));

    //     let object_to_world = Transform::translate(Vector3::new(5.5, -5.5, 24.0));
    //     let world_to_object = object_to_world.inverse();
    //     let sphere = Sphere::new(object_to_world, world_to_object, 4.5);
    //     let plastic_material = Plastic::new(0.001, Spectrum::new(0.4, 0.5, 0.1), Spectrum::new(0.4, 0.5, 0.1));
    //     let plastic_ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(plastic_material));
    //     primitives.push(Box::new(plastic_ball));


    //     let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.99, 20.0)) * Transform::rotate_x(90.0);
    //     let world_to_object4 = object_to_world4.inverse();
    //     let disk = Disk::new(object_to_world4, world_to_object4, 3.0);
    //     let disk_light = AreaLight::new(Box::new(disk), Spectrum::new(1.0, 1.0, 1.0));
    //     lights.push(Arc::new(disk_light));

    //     // let object_to_world4 = Transform::translate(Vector3::new(7.5, 8.0, 30.0));
    //     // let world_to_object4 = object_to_world4.inverse();
    //     // let ball =  Sphere::new(object_to_world4, world_to_object4, 1.0);
    //     // let disk_light = AreaLight::new(Box::new(ball), Spectrum::new(1.0, 1.0, 1.0));
    //     // lights.push(Arc::new(disk_light));

    //     // let object_to_world4 = Transform::translate(Vector3::new(-7.5, 8.0, 30.0));
    //     // let world_to_object4 = object_to_world4.inverse();
    //     // let ball =  Sphere::new(object_to_world4, world_to_object4, 1.0);
    //     // let disk_light = AreaLight::new(Box::new(ball), Spectrum::new(1.0, 1.0, 1.0));
    //     // lights.push(Arc::new(disk_light));


    //     let bvh = BVH::new(primitives);
    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));

    //     // camera
    //     const WIDTH: usize = 500;
    //     const HEIGHT: usize = 500;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 0.0, 0.0);
    //     let look = Vector3::new(0.0, 0.0, 1.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         60.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }



    // pub fn test_texture() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let mut lights: Vec<Arc<dyn Light>> = Vec::new();

    //     let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
    //     let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(right_wall));

    //     let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
    //     let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(left_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
    //     let world_to_object = object_to_world.inverse();
    //     let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(back_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(upper_wall));


    //     let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(-90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
    //     let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(bot_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, -5.0, 22.0));
    //     let world_to_object = object_to_world.inverse();
    //     let sphere = Sphere::new(object_to_world.clone(), world_to_object, 5.0);
    //     let world_to_texture = (object_to_world * Transform::rotate_x(90.0)).inverse();
    //     let mapping = SphericalMapping::new(world_to_texture);
    //     let matte_material = Matte::new(Box::new(ImageTexture::new(Box::new(mapping), Texels::new("./texture.png"))));
    //     let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(matte_material));
    //     primitives.push(Box::new(ball));

    //     let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.99, 20.0)) * Transform::rotate_x(90.0);
    //     let world_to_object4 = object_to_world4.inverse();
    //     let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
    //     let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
    //     lights.push(Arc::new(disk_light));


    //     let bvh = BVH::new(primitives);
    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));

    //     // camera
    //     const WIDTH: usize = 500;
    //     const HEIGHT: usize = 500;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 0.0, 0.0);
    //     let look = Vector3::new(0.0, 0.0, 1.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         60.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }

    // pub fn test_bunny() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let mut lights: Vec<Arc<dyn Light>> = Vec::new();

    //     let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
    //     let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(right_wall));

    //     let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
    //     let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(left_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
    //     let world_to_object = object_to_world.inverse();
    //     let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(back_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(upper_wall));


    //     let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(-90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
    //     let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(bot_wall));

    //     // load models 
    //     let models = TriangleMesh::load("./models/bunny.obj");
        
    //     let bunny_mesh = models.get("Bunny").unwrap();
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.5, 0.5, 0.4))));
    //     let object_to_world = Transform::translate(Vector3::new(5.0, -11.5, 24.0)) * Transform::scale(50.0, 50.0, 50.0) * Transform::rotate_y(-30.0);
    //     let bunny = MeshPrimitive::new(bunny_mesh.clone(), Arc::new(matte_material), object_to_world);
    //     primitives.push(Box::new(bunny));

    //     let object_to_world = Transform::translate(Vector3::new(-3.0, -11.5, 18.0)) * Transform::scale(50.0, 50.0, 50.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(1.0, 0.5, 0.4))));
    //     let bunny2 = MeshPrimitive::new(bunny_mesh.clone(), Arc::new(matte_material), object_to_world);
    //     primitives.push(Box::new(bunny2));

    //     // lights

    //     let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.999, 20.0)) * Transform::rotate_x(90.0);
    //     let world_to_object4 = object_to_world4.inverse();
    //     let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
    //     let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
    //     lights.push(Arc::new(disk_light));


    //     let bvh = BVH::new(primitives);
    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));

    //     // camera
    //     const WIDTH: usize = 1920;
    //     const HEIGHT: usize = 1080;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 0.0, 0.0);
    //     let look = Vector3::new(0.0, 0.0, 1.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         60.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }   

    // pub fn test_dragon() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let lights: Vec<Arc<dyn Light>> = Vec::new();


    //     // load models 
    //     eprintln!("loading models");
    //     let models = TriangleMesh::load("./models/dragon.obj");
    //     eprintln!("finish loading models");
        
    //     let dragon_mesh = models.get("xyzrgb_dragon").unwrap();
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.5, 0.5, 0.4))));
    //     let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 200.0)) * Transform::rotate_y(-130.0) * Transform::rotate_x(90.0);
    //     let xyzrgb_dragon = MeshPrimitive::new(dragon_mesh.clone(), Arc::new(matte_material), object_to_world);
    //     primitives.push(Box::new(xyzrgb_dragon));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, -40.0, 200.0)) * Transform::rotate_x(90.0) ;
    //     let world_to_object = object_to_world.inverse();
    //     let disk = Disk::new(object_to_world, world_to_object, 1000.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
    //     let ground = GeometricPrimitive::new(Box::new(disk), Arc::new(matte_material));
    //     primitives.push(Box::new(ground));

    //     // lights

    //     eprintln!("constructing bvh");
    //     let bvh = BVH::new(primitives);
    //     eprintln!("finish bvh construction");

    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));


    //     // camera 
    //     const WIDTH: usize = 1920;
    //     const HEIGHT: usize = 1080;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 30.0, 20.0);
    //     let look = Vector3::new(0.0, 0.0, 100.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         60.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }   

    // pub fn test_microfacet() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let mut lights: Vec<Arc<dyn Light>> = Vec::new();

    //     // init the cornell box
    //     let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
    //     let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(right_wall));

    //     let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
    //     let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(left_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
    //     let world_to_object = object_to_world.inverse();
    //     let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(back_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(upper_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(-90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
    //     let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(bot_wall));


    //     // place objects inside the box
    //     let object_to_world = Transform::translate(Vector3::new(0.0, -6.0, 20.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let sphere = Sphere::new(object_to_world, world_to_object, 3.0);
    //     let metal_material = Plastic::new(0.0, Spectrum::new(0.2, 0.2, 0.3), Spectrum::new(0.2, 0.2, 0.3));
    //     let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(metal_material));
    //     primitives.push(Box::new(ball));

    //     // lights
    //     // let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.9999, 20.0)) * Transform::rotate_x(90.0);
    //     // let world_to_object4 = object_to_world4.inverse();
    //     // let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
    //     // let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
    //     // lights.push(Arc::new(disk_light));

    //     // let object_to_world5 = Transform::translate(Vector3::new(3.0, -3.0, 15.0));
    //     let object_to_world5 = Transform::translate(Vector3::new(0.0, 2.0, 15.0));
    //     let world_to_object5 = object_to_world5.inverse();
    //     let sphere_light = Sphere::new(object_to_world5, world_to_object5, 0.2);
    //     let sphere_light = AreaLight::new(Box::new(sphere_light), Spectrum::new(50.0, 50.0, 50.0));
    //     lights.push(Arc::new(sphere_light));

    //     let bvh = BVH::new(primitives);
    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));

    //     // camera
    //     const WIDTH: usize = 500;
    //     const HEIGHT: usize = 500;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 0.0, 0.0);
    //     let look = Vector3::new(0.0, 0.0, 1.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         60.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }

    // pub fn test_plastic() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let mut lights: Vec<Arc<dyn Light>> = Vec::new();

    //     // init the cornell box
    //     let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
    //     let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(right_wall));

    //     let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
    //     let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(left_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
    //     let world_to_object = object_to_world.inverse();
    //     let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(back_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
    //     let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(upper_wall));

    //     let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(-90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
    //     let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.8, 0.8, 0.8))));
    //     let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
    //     primitives.push(Box::new(bot_wall));


    //     // place objects inside the box
    //     let object_to_world = Transform::translate(Vector3::new(0.0, -6.0, 20.0)) * Transform::rotate_x(90.0);
    //     let world_to_object = object_to_world.inverse();
    //     let sphere = Sphere::new(object_to_world, world_to_object, 3.0);
    //     let metal_material = Plastic::new(0.02, Spectrum::new(0.2, 0.2, 0.3), Spectrum::new(0.2, 0.2, 0.3));
    //     let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(metal_material));
    //     primitives.push(Box::new(ball));

    //     // lights
    //     // let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.9999, 20.0)) * Transform::rotate_x(90.0);
    //     // let world_to_object4 = object_to_world4.inverse();
    //     // let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
    //     // let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
    //     // lights.push(Arc::new(disk_light));

    //     // let object_to_world5 = Transform::translate(Vector3::new(3.0, -3.0, 15.0));
    //     let object_to_world5 = Transform::translate(Vector3::new(3.0, -2.0, 15.0));
    //     let world_to_object5 = object_to_world5.inverse();
    //     let sphere_light = Sphere::new(object_to_world5, world_to_object5, 0.2);
    //     let sphere_light = AreaLight::new(Box::new(sphere_light), Spectrum::new(50.0, 50.0, 50.0));
    //     lights.push(Arc::new(sphere_light));

    //     let bvh = BVH::new(primitives);
    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));

    //     // camera
    //     const WIDTH: usize = 500;
    //     const HEIGHT: usize = 500;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 0.0, 0.0);
    //     let look = Vector3::new(0.0, 0.0, 1.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         60.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }

    // pub fn many_light() -> (PerspectiveCamera, Scene) {
    //     let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
    //     let mut lights: Vec<Arc<dyn Light>> = Vec::new();

    //     for i in 0..7 {
    //         let object_to_world = Transform::translate(Vector3::new(-15.0 + i as f64 * 5.0, -6.0, 20.0)) * Transform::rotate_x(90.0);
    //         let world_to_object = object_to_world.inverse();
    //         let disk = Disk::new(object_to_world, world_to_object, 0.5 + i as f64 * 0.5);
    //         let metal_material = Plastic::new(0.02, Spectrum::new(0.2, 0.2, 0.3), Spectrum::new(0.2, 0.2, 0.3));
    //         let disk_primitive = GeometricPrimitive::new(Box::new(disk), Arc::new(metal_material));
    //         primitives.push(Box::new(disk_primitive));

    //         let object_to_world = Transform::translate(Vector3::new(-15.0 + i as f64 * 5.0, -5.0, 20.0)) * Transform::rotate_x(90.0);
    //         let world_to_object = object_to_world.inverse();
    //         let sphere = Sphere::new(object_to_world, world_to_object, 0.02 + i as f64 * 0.05);
    //         let sphere_light = AreaLight::new(Box::new(sphere), Spectrum::new(1.0, 1.0, 1.0));
    //         lights.push(Arc::new(sphere_light));
    //     }


    //     let bvh = BVH::new(primitives);
    //     let scene = Scene::new(LightList::new(lights), Box::new(bvh));

    //     const WIDTH: usize = 1000;
    //     const HEIGHT: usize = 500;
    //     const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

    //     let pos = Vector3::new(0.0, 0.0, 0.0);
    //     let look = Vector3::new(0.0, 0.0, 1.0);
    //     let up = Vector3::new(0.0, 1.0, 0.0);
    //     let camera_to_world = Transform::look_at(pos, look, up).inverse();

    //     let camera = PerspectiveCamera::new(
    //         camera_to_world,
    //         (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
    //         0.0,
    //         1.0,
    //         45.0,
    //         Film::new(WIDTH, HEIGHT),
    //     );

    //     (camera, scene)
    // }

//     pub fn test_layerd_diffuse() -> (PerspectiveCamera, Scene) {
//         let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
//         let mut lights: Vec<Arc<dyn Light>> = Vec::new();

//         let object_to_world = Transform::translate(Vector3::new(10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
//         let world_to_object = object_to_world.inverse();
//         let right_wall = Disk::new(object_to_world, world_to_object, 150.0);
//         let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.65, 0.05, 0.05))));
//         let right_wall = GeometricPrimitive::new(Box::new(right_wall), Arc::new(matte_material));
//         primitives.push(Box::new(right_wall));

//         let object_to_world = Transform::translate(Vector3::new(-10.0, 0.0, 0.0)) * Transform::rotate_y(90.0);
//         let world_to_object = object_to_world.inverse();
//         let left_wall = Disk::new(object_to_world, world_to_object, 150.0);
//         let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.12, 0.45, 0.15))));
//         let left_wall = GeometricPrimitive::new(Box::new(left_wall), Arc::new(matte_material));
//         primitives.push(Box::new(left_wall));

//         let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 30.0));
//         let world_to_object = object_to_world.inverse();
//         let back_wall = Disk::new(object_to_world, world_to_object, 150.0);
//         let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
//         let back_wall = GeometricPrimitive::new(Box::new(back_wall), Arc::new(matte_material));
//         primitives.push(Box::new(back_wall));

//         let object_to_world = Transform::translate(Vector3::new(0.0, 10.0, 0.0)) * Transform::rotate_x(90.0);
//         let world_to_object = object_to_world.inverse();
//         let upper_wall = Disk::new(object_to_world, world_to_object, 150.0);
//         let matte_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.73, 0.73, 0.73))));
//         let upper_wall = GeometricPrimitive::new(Box::new(upper_wall), Arc::new(matte_material));
//         primitives.push(Box::new(upper_wall));

//         let object_to_world = Transform::translate(Vector3::new(0.0, -10.0, 0.0)) * Transform::rotate_x(90.0);
//         let world_to_object = object_to_world.inverse();
//         let bot_wall = Disk::new(object_to_world, world_to_object, 150.0);
//         let matte_material = LayeredDiffuse::new(0.0, Spectrum::new(1.0, 1.0, 1.0), Spectrum::new(0.2, 0.3, 0.6));
//         let bot_wall = GeometricPrimitive::new(Box::new(bot_wall), Arc::new(matte_material));
//         primitives.push(Box::new(bot_wall));


//         let object_to_world = Transform::translate(Vector3::new(0.0, -6.0, 20.0));
//         let world_to_object = object_to_world.inverse();
//         let sphere = Sphere::new(object_to_world, world_to_object, 3.0);
//         let metal_material = Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.6, 0.4, 0.5))));
//         let ball = GeometricPrimitive::new(Box::new(sphere), Arc::new(metal_material));
//         primitives.push(Box::new(ball));

//         let object_to_world4 = Transform::translate(Vector3::new(0.0, 9.9999, 20.0)) * Transform::rotate_x(90.0);
//         let world_to_object4 = object_to_world4.inverse();
//         let disk_light = Disk::new(object_to_world4, world_to_object4, 3.0);
//         let disk_light = AreaLight::new(Box::new(disk_light), Spectrum::new(10.0, 10.0, 10.0));
//         lights.push(Arc::new(disk_light));

//         let bvh = BVH::new(primitives);
//         let scene = Scene::new(LightList::new(lights), Box::new(bvh));

//         // camera
//         const WIDTH: usize = 500;
//         const HEIGHT: usize = 500;
//         const FRAME: f64 = WIDTH as f64 / HEIGHT as f64;

//         let pos = Vector3::new(0.0, 0.0, 0.0);
//         let look = Vector3::new(0.0, 0.0, 1.0);
//         let up = Vector3::new(0.0, 1.0, 0.0);
//         let camera_to_world = Transform::look_at(pos, look, up).inverse();

//         let camera = PerspectiveCamera::new(
//             camera_to_world,
//             (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
//             0.0,
//             1.0,
//             60.0,
//             Film::new(WIDTH, HEIGHT),
//         );

//         (camera, scene)
//     }

}