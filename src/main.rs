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
use crate::spectrum::Spectrum;
use crate::camera::{Camera, CameraSample};
use crate::integrator::Integrator;

use rand::random;

use std::sync::{Arc, Mutex};
use std::thread;

const WIDTH: usize = 600;
const HEIGHT: usize = 500;
const FRAME: f64 = (WIDTH as f64) / (HEIGHT as f64);


fn main() {
    // create camera
    let pos = Vector3::new(0.0, 0.0, -3.0);
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

    let scene = Scene::cornell_box();

    // render
    let now = std::time::Instant::now();

    let integrator = PathIntegrator::new(camera, 50);
    render(integrator, scene, "./results/cornell_glass_thread.ppm");


    let cost = now.elapsed().as_millis();
    println!("render cost: {} secs", (cost as f64) / 1000.0);
}


fn render(integrator: PathIntegrator, scene: Scene, filename: &str) {
    let res = integrator.camera.film.resolution;
    let (width, height) = (res.x, res.y);

    let integrator = Arc::new(integrator);
    let scene = Arc::new(scene);

    let mut handlers = Vec::new();

    for _tid in 0..integrator.n_thread {
        let int = Arc::clone(&integrator);
        let scene = Arc::clone(&scene);

        let handler = thread::spawn(move || {
            for i in 0..height {
                for j in 0..width {
                    // first render the upper left pixel, then go rightwards and downwards
                    let mut radiance = Spectrum::new(0.0, 0.0, 0.0);
                    
                    for _ in 0..int.n_sample {
                        let sample = CameraSample::new(Point2::new(j as f64 + random::<f64>(), i as f64 + random::<f64>()), 0.0);
                        let mut r = int.camera.generate_ray(sample);

                        radiance += int.li(&mut r, &scene);
                    }

                    radiance /= int.n_sample as f64 * int.n_thread as f64;

                    int.camera.film.record(i, j, radiance);
                }
            }
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    integrator.camera.film.write_to_image(filename);
}