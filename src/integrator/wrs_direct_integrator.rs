use core::panic;

use cgmath::{Point2, InnerSpace};
use rand::random;

use crate::{spectrum::Spectrum, geometry::ray::Ray, scene::Scene};
use super::{Integrator, visibility_test};

pub struct WRSDirectIntegrator {
    pub m: usize, // number of sample candiates
}

impl WRSDirectIntegrator {
    pub fn new(m: usize) -> Self {
        WRSDirectIntegrator { m: 32 }
    }
}


impl Integrator for WRSDirectIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene) -> Spectrum {
        
    }
}