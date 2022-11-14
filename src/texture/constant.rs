use crate::spectrum::Spectrum;

use super::Texture;

pub struct ConstantTexture<T> {
    value: T,
}

impl Texture<f64> for ConstantTexture<f64> {
    fn evaluate(&self, _isect: &crate::geometry::interaction::SurfaceInteraction) -> f64 {
        self.value
    }
}

impl Texture<Spectrum> for ConstantTexture<Spectrum> {
    fn evaluate(&self, _isect: &crate::geometry::interaction::SurfaceInteraction) -> Spectrum {
        self.value
    }
}