use std::f64::INFINITY;

use cgmath::{EuclideanSpace, InnerSpace, Point2, Point3, Vector3};

use super::super::geometry::ray::Ray;
use super::super::geometry::transform::Transform;
use super::film::Film;
use super::{Camera, CameraSample};

pub struct PerspectiveCamera {
    camera_to_world: Transform, // view matrix
    raster_to_camera: Transform,
    shutter_open: f64,
    shutter_close: f64,
    pub film: Film,
}

impl PerspectiveCamera {
    pub fn new(
        camera_to_world: Transform,
        screen_window: (Point2<f64>, Point2<f64>),
        shutter_open: f64,
        shutter_close: f64,
        fov: f64,
        film: Film,
    ) -> Self {
        // first compute screen to raster
        let res = film.resolution;
        let screen_to_raster = Transform::scale(res.x as f64, res.y as f64, 1.0)
            * Transform::scale(
                1.0 / (screen_window.1.x - screen_window.0.x),
                -1.0 / (screen_window.1.y - screen_window.0.y),
                1.0,
            )
            * Transform::translate(Vector3::new(-screen_window.0.x, -screen_window.1.y, 0.0));

        let camera_to_screen = Transform::perspective(fov, 1.0, 1e6); // the near and far plane are set arbitrarily

        PerspectiveCamera {
            camera_to_world,
            raster_to_camera: (screen_to_raster * camera_to_screen).inverse(), // to be implelmented,
            shutter_open,
            shutter_close,
            film,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: CameraSample) -> crate::geometry::ray::Ray {
        let p_camera = self.raster_to_camera.transform_point3(&Point3::new(
            sample.p_film.x,
            sample.p_film.y,
            0.0,
        ));
        // cast ray in the camera space
        let r = Ray {
            o: Point3::new(0.0, 0.0, 0.0),
            d: p_camera.to_vec(),
            time: sample.time,
            t_max: INFINITY,
        };

        self.camera_to_world.transform_ray(&r)
    }
}
