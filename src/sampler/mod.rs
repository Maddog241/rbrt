use cgmath::Point2;

pub mod wrs;
pub mod uniform_sampler;

pub trait Sampler : Sync + Send {
    fn get_2d(&self) -> Point2<f64>;
}