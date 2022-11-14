pub mod constant;
pub mod scale;
pub mod mix;
pub mod imagemap;
pub mod mapping;

use crate::geometry::interaction::SurfaceInteraction;

pub trait Texture<T>: Send + Sync{
    fn evaluate(&self, isect: &SurfaceInteraction) -> T;
}
