use crate::light::Light;

use super::Primitive;

pub struct Scene {
    pub lights: Vec<Box<dyn Light>>,
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            primitives: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_primitive(&mut self, primitive: Box<dyn Primitive>) {
        self.primitives.push(primitive);
    }

    pub fn intersect(&self, r: &mut crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        let mut ret = None;        
        for prim in self.primitives.iter() {
            if let Some(isect) = prim.intersect(r) {
                ret = Some(isect);
            }
        }

        ret
    }
}