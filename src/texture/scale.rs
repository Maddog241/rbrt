use crate::spectrum::Spectrum;

use super::Texture;

pub struct ScaleTexture<T, U> {
    tex1: Box<dyn Texture<T>>,
    tex2: Box<dyn Texture<U>>,
}

impl Texture<f64> for ScaleTexture<f64, f64> {
    fn evaluate(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> f64 {
        self.tex1.evaluate(isect) * self.tex2.evaluate(isect)
    }
}

impl Texture<Spectrum> for ScaleTexture<f64, Spectrum> {
    fn evaluate(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> Spectrum {
        self.tex1.evaluate(isect) * self.tex2.evaluate(isect)
    }
}

impl Texture<Spectrum> for ScaleTexture<Spectrum, Spectrum> {
    fn evaluate(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> Spectrum {
        self.tex1.evaluate(isect) * self.tex2.evaluate(isect)
    }
}
