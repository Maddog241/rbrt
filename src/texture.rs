pub mod constant;
pub mod scale;
pub mod mix;

use crate::geometry::interaction::SurfaceInteraction;

pub trait Texture<T> {
    fn evaluate(&self, isect: &SurfaceInteraction) -> T;
}
