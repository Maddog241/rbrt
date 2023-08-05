use cgmath::Point2;
use rand::random;

use super::Sampler;

pub struct UniformSampler {
}

impl UniformSampler {
    pub fn new() -> Self {
        Self {}
    }


}

impl Sampler for UniformSampler {
    fn get_2d(&self) -> Point2<f64> {
        Point2::new(random::<f64>(), random::<f64>())
    }
}