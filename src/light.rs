pub mod point_light;
pub mod area_light;

use crate::{spectrum::Spectrum, geometry::{interaction::SurfaceInteraction, ray::Ray, shape::Shape}};
use cgmath::{Point2, Point3, Vector3};

pub enum Light {
    PointLight {
        p: Point3<f64>,
        intensity: Spectrum,
    },
    AreaLight {
        shape: Shape,
        emit: Spectrum,
    }
}

impl Light {
    pub fn sample_li(&self, isect: &SurfaceInteraction, u: Point2<f64>) -> (Spectrum, Point3<f64>, f64) {
        match self {
            Self::PointLight { p, intensity } => {
                point_light::sample_li(self, isect, u)
            },

            Self::AreaLight { shape, emit } => {
                area_light::sample_li(self, isect, u)
            }
        }
    }


    pub fn le(&self, n: Vector3<f64>, d: Vector3<f64>) -> Spectrum {
        match self {
            Self::PointLight { p, intensity } => {
                point_light::le(self, n, d)
            },

            Self::AreaLight { shape, emit } => {
                area_light::le(self, n, d)
            }
        }
    }


    pub fn intersect_p(&self, r: &Ray) -> Option<f64> {
        match self {
            Self::PointLight { p, intensity } => {
                point_light::intersect_p(self, r)
            },

            Self::AreaLight { shape, emit } => {
                area_light::intersect_p(self, r)
            }
        }
    }


    pub fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        match self {
            Self::PointLight { p, intensity } => {
                point_light::intersect(self, r)
            },

            Self::AreaLight { shape, emit } => {
                area_light::intersect(self, r)
            }
        }
    }
}

impl Light {
    pub fn create_area_light(shape: Shape, emit: Spectrum) -> Light {
        Light::AreaLight { shape, emit}
    }

    pub fn create_point_light(p: Point3<f64>, intensity: Spectrum) -> Light {
        Light::PointLight { p, intensity}
    }
}