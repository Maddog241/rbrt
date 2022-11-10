pub mod point_light;
pub mod area_light;

use crate::{spectrum::Spectrum, geometry::{interaction::SurfaceInteraction, ray::Ray, shape::Shape}};
use cgmath::{Point2, Point3};

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Light {
    pub fn sample_li(&self, isect: &SurfaceInteraction, u: Point2<f64>) -> (Spectrum, Point3<f64>, f64) {
        match self {
            Self::PointLight { p:_, intensity:_ } => {
                point_light::sample_li(self, isect, u)
            },

            Self::AreaLight { shape:_, emit:_ } => {
                area_light::sample_li(self, isect, u)
            }
        }
    }


    pub fn le(&self) -> Spectrum {
        match self {
            Self::PointLight { p:_, intensity:_ } => {
                point_light::le(self)
            },

            Self::AreaLight { shape:_, emit:_ } => {
                area_light::le(self)
            }
        }
    }


    pub fn intersect_p(&self, r: &Ray) -> Option<f64> {
        match self {
            Self::PointLight { p:_, intensity:_ } => {
                point_light::intersect_p(self, r)
            },

            Self::AreaLight { shape:_, emit:_ } => {
                area_light::intersect_p(self, r)
            }
        }
    }


    pub fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        match self {
            Self::PointLight { p:_, intensity:_ } => {
                point_light::intersect(self, r)
            },

            Self::AreaLight { shape:_, emit:_ } => {
                area_light::intersect(self, r)
            }
        }
    }
}

#[allow(dead_code)]
impl Light {
    pub fn create_area_light(shape: Shape, emit: Spectrum) -> Light {
        Light::AreaLight { shape, emit}
    }

    pub fn create_point_light(p: Point3<f64>, intensity: Spectrum) -> Light {
        Light::PointLight { p, intensity}
    }
}