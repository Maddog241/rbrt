pub mod geometric_primitive;
pub mod scene;

use std::sync::Arc;

use crate::{geometry::{ray::Ray, interaction::SurfaceInteraction, bound3::Bound3, shape::Shape}, light::Light, material::Material};

pub enum Primitive {
    GeometricPrimitive {
        shape: Shape,
        material: Arc<Material>,
    },

}

impl Primitive {
    pub fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        match self {
            Self::GeometricPrimitive { shape, material } => {
                geometric_primitive::intersect(self, r)
            }
        }
    }


    pub fn intersect_p(&self, r: &Ray) -> Option<f64> {
        match self {
            Self::GeometricPrimitive { shape, material } => {
                geometric_primitive::intersect_p(self, r)
            }
        }
    }


    pub fn world_bound(&self) -> Bound3 {
        match self {
            Self::GeometricPrimitive { shape, material } => {
                geometric_primitive::world_bound(self)
            }
        }
    }
}

impl Primitive {
    pub fn create_geometric_primitive(shape: Shape, material: Material) -> Primitive {
        Self::GeometricPrimitive { shape, material: Arc::new(material)}
    }
}