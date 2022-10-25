mod camera;
mod geometry;
mod utils;
mod bxdf;
mod spectrum;
mod primitive;
mod material;
mod light;
mod integrator;
mod sampler;

use camera::{film::Film, perspective::PerspectiveCamera, pixel::Pixel, Camera, CameraSample};
use cgmath::{Matrix4, Point2, Point3, Vector2, Vector3, Vector4};
use geometry::{sphere::Sphere, transform::Transform};
use std::f64::consts::PI;
use std::rc::Rc;

use crate::geometry::cylinder::{Cylinder, self};
use crate::integrator::path_integrator::PathIntegrator;
use crate::light::point_light::PointLight;
use crate::primitive::scene::Scene;
use crate::{primitive::geometric_primitive::GeometricPrimitive, spectrum::Spectrum, material::matte::Matte};


const WIDTH: usize = 600;
const HEIGHT: usize = 500;
const FRAME: f64 = (WIDTH as f64) / (HEIGHT as f64);


fn main() {
    // create camera
    let pos = Vector3::new(0.0, 1.0, -3.0);
    let look = Vector3::new(0.0, 0.0, 1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let camera_to_world = Transform::look_at(pos, look, up).inverse();

    let camera = PerspectiveCamera::new(
        camera_to_world,
        (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
        0.0,
        1.0,
        90.0,
        Film::new(WIDTH, HEIGHT),
    );

    let scene = Scene::world_two();

    // render
    let now = std::time::Instant::now();

    let mut integrator = PathIntegrator::new(50, camera);
    integrator.render(&scene, "./results/glass.ppm");

    let cost = now.elapsed().as_millis();
    println!("render cost: {} secs", (cost as f64) / 1000.0);
}
