use cgmath::{Point3, Point2, InnerSpace};

use crate::{geometry::{shape::Shape, interaction::SurfaceInteraction, transform::Transform}, spectrum::Spectrum};

use super::Light;

pub struct AreaLight {
    shape: Box<dyn Shape>,
    le: Spectrum,
}

impl AreaLight {
    pub fn new(shape: Box<dyn Shape>, le: Spectrum) -> Self {
        AreaLight {
            shape,
            le,
        }
    }
}

impl Light for AreaLight {
    fn sample_li(&self, isect: &SurfaceInteraction, u: Point2::<f64>) -> (Spectrum, Point3::<f64>, f64) {
        let (p, n, area_pdf) = self.shape.sample(u);

        let distance2 = (p - isect.p).magnitude2();
        let we = isect.p - p;
        let cosine = we.dot(n).max(0.0);

        let pdf = if cosine != 0.0 { area_pdf * distance2 / cosine } else { 0.0 };

        (self.le, p, pdf)
    }
}