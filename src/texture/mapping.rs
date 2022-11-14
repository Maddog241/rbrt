use cgmath::Point2;

use crate::geometry::interaction::SurfaceInteraction;

pub mod uv;
pub mod spherical;

pub trait TextureMapping2D : Send + Sync{
    fn map(&self, isect: &SurfaceInteraction) -> Point2<f64>;
}