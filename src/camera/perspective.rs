use std::f64::INFINITY;

use cgmath::{Vector3, Point2, Point3, EuclideanSpace, InnerSpace};

use super::{Camera, CameraSample};
use super::film::Film;
use super::super::geometry::transform::Transform;
use super::super::geometry::ray::Ray;

pub struct PerspectiveCamera {
    world_to_camera: Transform, // view matrix
    raster_to_camera: Transform,
    shutter_open: f64,
    shutter_close: f64,
    film: Film,
}

impl PerspectiveCamera {
    pub fn new(world_to_camera: Transform, screen_window: (Point2<f64>, Point2<f64>), shutter_open: f64, shutter_close: f64, fov: f64, film: Film) -> Self {
        // first compute screen to raster
        let res  = film.resolution;
        let screen_to_raster = 
            Transform::scale(res.x as f64, res.y as f64, 1.0) * 
            Transform::scale(1.0/(screen_window.1.x - screen_window.0.x), 1.0/(screen_window.1.y-screen_window.0.y), 1.0) * 
            Transform::translate(Vector3::new(-screen_window.0.x, -screen_window.0.y, 1.0));

        let camera_to_screen = Transform::perspective(fov, 0.01, 1e6); // the near and far plane are set arbitrarily

        PerspectiveCamera {
            world_to_camera,
            raster_to_camera: (camera_to_screen * screen_to_raster).inverse(), // to be implelmented,
            shutter_open,
            shutter_close,
            film,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: CameraSample) -> crate::geometry::ray::Ray {
        let p_camera = self.raster_to_camera.transform_point3(&sample.p_film);
        // cast ray in the camera space
        let r = Ray {
            o: Point3::new(0.0, 0.0, 0.0),
            d: p_camera.to_vec().normalize(),
            time: sample.time,
            t_max: INFINITY,
        };

        self.world_to_camera.inverse().transform_ray(&r)
    }
}