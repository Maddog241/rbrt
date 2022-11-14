use crate::spectrum::Spectrum;

use super::Texture;

pub struct MixTexture<T> {
    tex1: Box<dyn Texture<T>>,
    tex2: Box<dyn Texture<T>>,
    amount: Box<dyn Texture<f64>>,
}

impl Texture<f64> for MixTexture<f64> {
    fn evaluate(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> f64 {
        let lambda = self.amount.evaluate(isect);
        assert!(lambda >= 0.0 && lambda <= 1.0);
        (1.0 - lambda) * self.tex1.evaluate(isect) + lambda * self.tex2.evaluate(isect)
    }
}

impl Texture<Spectrum> for MixTexture<Spectrum> {
    fn evaluate(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> Spectrum {
        let lambda = self.amount.evaluate(isect);
        assert!(lambda >= 0.0 && lambda <= 1.0);
        (1.0 - lambda) * self.tex1.evaluate(isect) + lambda * self.tex2.evaluate(isect)
    }
}