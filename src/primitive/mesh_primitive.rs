use crate::{geometry::{shape::mesh::{TriangleMesh, Triangle}, transform::Transform}, material::Material, accelerator::bvh::BVH};

use std::sync::Arc;

use super::{geometric_primitive::GeometricPrimitive, Primitive};

pub struct MeshPrimitive {
    bvh: BVH,
}

impl MeshPrimitive {
    pub fn new(mesh: Arc<TriangleMesh>, material: Arc<dyn Material>, object_to_world: Transform) -> Self {
        let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();

        for ind in mesh.indices.chunks(3) {
            let triangle = Triangle::new(ind[0], ind[1], ind[2], mesh.clone(), object_to_world.clone());
            let triangle_obj = GeometricPrimitive::new(Box::new(triangle), material.clone());
            primitives.push(Box::new(triangle_obj));
        }

        let bvh = BVH::new(primitives);

        Self {
            bvh
        }
    }
}

impl Primitive for MeshPrimitive {
    fn intersect(&self, r: &mut crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        self.bvh.intersect(r)
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        self.bvh.intersect_p(r)
    }

    fn world_bound(&self) -> crate::geometry::bound3::Bound3 {
        self.bvh.world_bound()
    }
}