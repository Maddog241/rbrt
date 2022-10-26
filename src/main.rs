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

use camera::{film::Film, perspective::PerspectiveCamera};
use cgmath::{Point2, Vector3};
use geometry::transform::Transform;

use crate::integrator::path_integrator::PathIntegrator;
use crate::primitive::scene::Scene;


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

    let scene = Scene::world_three();

    // render
    let now = std::time::Instant::now();

    let mut integrator = PathIntegrator::new(50, camera);
    integrator.render(&scene, "./results/arealight4.ppm");

    let cost = now.elapsed().as_millis();
    println!("render cost: {} secs", (cost as f64) / 1000.0);
}
