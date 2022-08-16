mod camera;
mod geometry;
mod utils;

use camera::{film::Film, perspective::PerspectiveCamera, pixel::Pixel, Camera, CameraSample};
use cgmath::{Point2, Vector2, Vector3, Vector4};
use geometry::{shape::Shape, sphere::Sphere, transform::Transform};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const FRAME: f64 = (WIDTH as f64) / (HEIGHT as f64);

fn main() {
    // create camera
    let pos = Vector3::new(0.0, 0.0, 0.0);
    let look = Vector3::new(0.0, 0.0, 1.0);
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

    // create sphere
    let object_to_world = Transform::translate(Vector3::new(0.0, 0.0, 3.0));
    let world_to_object = object_to_world.inverse();
    let sphere = Sphere::new(0.1, object_to_world, world_to_object);

    // render
    // let now = std::time::Instant::now();
    // for i in 0..HEIGHT {
    //     for j in 0..WIDTH {
    //         let sample = CameraSample::new(Point2::new(j as f64, i as f64), 0.0);
    //         let r = camera.generate_ray(sample);
    //         let mut pixel = Pixel::new(0.0, 0.0, 0.0);
    //         // if let Some(_t) = sphere.intersect_p(&r) {
    //         //     pixel = Pixel::new(1.0, 0.0, 0.0);
    //         // }
    //         pixel = Pixel::new(r.d.x.max(0.0), r.d.y.max(0.0), r.d.z.max(0.0));
    //         println!("{:?}", r.d);
    //         camera.film.record(i, j, pixel);
    //     }
    // }
    // let cost = now.elapsed().as_millis();
    // println!("render cost: {} secs", (cost as f64) / 1000.0);

    // // write the film to the file
    // camera.film.write_to_image("./images/test2.ppm");
    // let cost2 = now.elapsed().as_millis();
    // println!("writing cost: {} secs", ((cost2 - cost) as f64) / 1000.0);

    let i = 0;
    let j = 3;
    let sample = CameraSample::new(Point2::new(j as f64, i as f64), 0.0);
    let r = camera.generate_ray(sample);
    println!("{:?}", r.d);
}
