use crate::{primitive::Primitive, accelerator::bvh::BVH, geometry::{bound3::Bound3, interaction::SurfaceInteraction, ray::{Beam, Ray}}, material::Material};
use cgmath::{Point3, Point2, Vector3, EuclideanSpace, InnerSpace};
use std::sync::Arc;


pub struct TriangleMesh {
    bvh: BVH,
}

impl TriangleMesh {
    pub fn new(positions: Vec<Point3<f64>>, tex_coords: Vec<Point2<f64>>, normals: Vec<Vector3<f64>>, indices: Vec<usize>, material: Arc<dyn Material>) -> Self {
        let positions = Arc::new(positions);
        let tex_coords = Arc::new(tex_coords);
        let normals = Arc::new(normals);

        let mut triangles: Vec<Box<dyn Primitive>> = Vec::new();
        for i in 0..indices.len()/3 {
            triangles.push(Box::new(Triangle::new(i, i+1, i+2, positions.clone(), tex_coords.clone(), normals.clone(), material.clone())));
        }

        TriangleMesh {
            bvh: BVH::new(triangles)
        }
    }

}

impl Primitive for TriangleMesh {
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


pub struct Triangle {
    a: usize,
    b: usize, 
    c: usize,
    positions: Arc<Vec<Point3<f64>>>,
    tex_coords: Arc<Vec<Point2<f64>>>,
    normals: Arc<Vec<Vector3<f64>>>,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a:usize, b:usize, c:usize, positions: Arc<Vec<Point3<f64>>>, tex_coords: Arc<Vec<Point2<f64>>>, normals: Arc<Vec<Vector3<f64>>>, material: Arc<dyn Material>) -> Self {
        Triangle { a, b, c, positions, tex_coords, normals, material }
    }

    fn permute(p: Point3<f64>, kx: usize, ky: usize, kz: usize) -> Point3<f64> {
        Point3::new(p[kx], p[ky], p[kz])
    }

    fn max_dimension(p: Vector3<f64>) -> usize {
        if p[0] >= p[1] && p[0] >= p[2] { 0 }
        else if p[1] >= p[0] && p[1] >= p[2] { 1 }
        else { 2}
    }
}

impl Primitive for Triangle {
    fn intersect(&self, r: &mut Ray) -> Option<SurfaceInteraction> {
        let mut p0 = self.positions[self.a];
        let mut p1 = self.positions[self.a];
        let mut p2 = self.positions[self.a];
        // first translate, let r.o in origin
        p0 -= r.o.to_vec(); 
        p1 -= r.o.to_vec();
        p2 -= r.o.to_vec();
        // second permute the coordinates
        let kz = Self::max_dimension(r.d);
        let kx = (kz + 1) % 3;
        let ky = (kx + 1) % 3;
        p0 = Self::permute(p0, kx, ky, kz);
        p1 = Self::permute(p1, kx, ky, kz);
        p2 = Self::permute(p2, kx, ky, kz);
        let d = Vector3::new(r.d[kx], r.d[ky], r.d[kz]);
        // shear transform 
        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = 1.0 / d.z;
        p0.x -= sx * p0.z;
        p0.y -= sy * p0.z;
        p0.z *= sz;
        p1.x -= sx * p1.z;
        p1.y -= sy * p1.z;
        p1.z *= sz;
        p2.x -= sx * p2.z;
        p2.y -= sy * p2.z;
        p2.z *= sz;
        // complete transform 
        
        // compute area
        let e0 = p1.x*p2.y - p1.y*p2.x;
        let e1 = p2.x*p0.y - p2.y*p0.x;
        let e2 = p0.x*p1.y - p0.y*p1.x;
        let e = e0 + e1 + e2;

        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return None;
        } else if e == 0.0 {
            return None;
        }

        // barycentric coordinate
        let inv_e = 1.0 / e;
        let b0 = e0 * inv_e;
        let b1 = e1 * inv_e;
        let b2 = e2 * inv_e;

        let t = b0*p0.z + b1*p1.z + b2*p2.z;

        // check if t is valid
        if t < 0.0001 || t >= r.t_max {
            return None;
        }

        // update ray's t_max
        r.t_max = t;
        //

        let p = r.at(t);
        let n0 = self.normals[self.a];
        let n1 = self.normals[self.b];
        let n2 = self.normals[self.c];
        let n = b0*n0 + b1*n1 + b2*n2;
        let wo = -r.d.normalize();

        let isect = SurfaceInteraction {
            p,
            n,
            t,
            time: r.time,
            wo,
            material: Some(self.material.clone()),
            hit_light: false,
            radiance: None,
        };

        Some(isect)
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        // in world space
        let mut p0 = self.positions[self.a];
        let mut p1 = self.positions[self.a];
        let mut p2 = self.positions[self.a];
        // first translate, let r.o in origin
        p0 -= r.o.to_vec(); 
        p1 -= r.o.to_vec();
        p2 -= r.o.to_vec();
        // second permute the coordinates
        let kz = Self::max_dimension(r.d);
        let kx = (kz + 1) % 3;
        let ky = (kx + 1) % 3;
        p0 = Self::permute(p0, kx, ky, kz);
        p1 = Self::permute(p1, kx, ky, kz);
        p2 = Self::permute(p2, kx, ky, kz);
        let d = Vector3::new(r.d[kx], r.d[ky], r.d[kz]);
        // shear transform 
        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = 1.0 / d.z;
        p0.x -= sx * p0.z;
        p0.y -= sy * p0.z;
        p0.z *= sz;
        p1.x -= sx * p1.z;
        p1.y -= sy * p1.z;
        p1.z *= sz;
        p2.x -= sx * p2.z;
        p2.y -= sy * p2.z;
        p2.z *= sz;
        // complete transform 
        
        // compute area
        let e0 = p1.x*p2.y - p1.y*p2.x;
        let e1 = p2.x*p0.y - p2.y*p0.x;
        let e2 = p0.x*p1.y - p0.y*p1.x;
        let e = e0 + e1 + e2;

        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return None;
        } else if e == 0.0 {
            return None;
        }

        // barycentric coordinate
        let inv_e = 1.0 / e;
        let b0 = e0 * inv_e;
        let b1 = e1 * inv_e;
        let b2 = e2 * inv_e;

        let t = b0*p0.z + b1*p1.z + b2*p2.z;

        // check if t is valid
        if t < 0.0001 || t >= r.t_max {
            return None;
        }

        Some(t)
    }

    fn world_bound(&self) -> crate::geometry::bound3::Bound3 {
        Bound3::new(self.positions[self.a], self.positions[self.b]).union_point3(self.positions[self.c])
    }
}
