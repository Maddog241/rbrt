mod camera;
mod geometry;
mod utils;
mod bxdf;
mod spectrum;
mod primitive;
mod material;
mod light;

use camera::{film::Film, perspective::PerspectiveCamera, pixel::Pixel, Camera, CameraSample};
use cgmath::{Matrix4, Point2, Point3, Vector2, Vector3, Vector4};
use geometry::{sphere::Sphere, transform::Transform};
use std::rc::Rc;

use crate::{primitive::geometric_primitive::GeometricPrimitive, spectrum::Spectrum, material::matte::Matte};
use crate::primitive::Primitive;


const WIDTH: usize = 600;
const HEIGHT: usize = 500;
const FRAME: f64 = (WIDTH as f64) / (HEIGHT as f64);


fn main() {
    // create camera
    let pos = Vector3::new(0.0, 0.0, 0.0);
    let look = Vector3::new(0.0, 0.0, 3.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let camera_to_world = Transform::look_at(pos, look, up).inverse();

    let mut camera = PerspectiveCamera::new(
        camera_to_world,
        (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
        0.0,
        1.0,
        90.0,
        Film::new(WIDTH, HEIGHT),
    );
    // create ball
    //// create sphere
    let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 3.0));
    let world_to_object = object_to_world.inverse();
    let sphere = Sphere::new(1.0, object_to_world, world_to_object);
    //// create lambertian material
    let matte_material = Matte::new(Spectrum::new(0.4, 0.6, 0.2));
    let ball = GeometricPrimitive::new(Box::new(sphere), Rc::new(matte_material));

    // render
    let now = std::time::Instant::now();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // first render the upper left pixel, then go rightwards and downwards
            let sample = CameraSample::new(Point2::new(j as f64, i as f64), 0.0);
            let mut r = camera.generate_ray(sample);
            let mut pixel = Pixel::new(0.0, 0.0, 0.0);
            // println!("{:?}", r.d);
            if let Some(inter) = ball.intersect(&mut r) {
                pixel = Pixel::new(inter.n.x.max(0.0), inter.n.y.max(0.0), inter.n.z.max(0.0));
            }
            camera.film.record(i, j, pixel);
        }
    }
    let cost = now.elapsed().as_millis();
    println!("render cost: {} secs", (cost as f64) / 1000.0);

    // write the film to the file
    camera.film.write_to_image("./images/material.ppm");
    let cost2 = now.elapsed().as_millis();
    println!("writing cost: {} secs", ((cost2 - cost) as f64) / 1000.0);
}
