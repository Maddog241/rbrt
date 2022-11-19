use crate::{primitive::Primitive, accelerator::bvh::BVH, geometry::{bound3::Bound3, interaction::SurfaceInteraction, ray::{Beam, Ray}, transform::Transform}, material::{Material, matte::Matte}, spectrum::Spectrum, texture::constant::ConstantTexture};
use cgmath::{Point3, Point2, Vector3, EuclideanSpace, InnerSpace};
use std::sync::Arc;

use tobj;

pub struct TriangleMesh {
    bvh: BVH,
}

impl TriangleMesh {
    pub fn new(positions: Arc<Vec<Point3<f64>>>, texcoords: Arc<Vec<Point2<f64>>>, normals: Arc<Vec<Vector3<f64>>>, indices: Vec<usize>, material: Arc<dyn Material>) -> Self {
        let mut triangles: Vec<Box<dyn Primitive>> = Vec::new();
        for i in 0..indices.len()/3 {
            triangles.push(Box::new(Triangle::new(indices[3*i] as usize, indices[3*i+1] as usize, indices[3*i+2] as usize, positions.clone(), texcoords.clone(), normals.clone(), material.clone())));
        }

        TriangleMesh {
            bvh: BVH::new(triangles),
        }
    }

    pub fn load(file_name: &str) -> Vec<TriangleMesh> {
        let obj =  tobj::load_obj(
            file_name, 
            &tobj::LoadOptions{
                single_index: true,
                triangulate: true,
                ..Default::default()
            });

        match obj {
            Ok((models, _)) => {
                let mut meshes = Vec::new();

                for m in models {
                    println!("loading model: {}", m.name);
                    let mesh = m.mesh;
                    if mesh.normals.is_empty() || mesh.texcoords.is_empty() {
                        println!("Missing mesh normals or texture coordinates in {:?}", file_name);
                    }
                    println!("{} has {} triangles", m.name, mesh.indices.len() / 3);

                    let mut positions = Vec::new();
                    for chunk in mesh.positions.chunks(3) {
                        positions.push(Point3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
                    }

                    let mut texcoords = Vec::new();
                    for chunk in mesh.texcoords.chunks(2) {
                        texcoords.push(Point2::new(chunk[0] as f64, chunk[1] as f64));
                    }
                    
                    let mut normals = Vec::new();
                    for chunk in mesh.normals.chunks(3) {
                        normals.push(Vector3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
                    }

                    let indices: Vec<usize> = mesh.indices.into_iter().map(|f| f as usize).collect();
                    meshes.push(TriangleMesh::new(Arc::new(positions), Arc::new(texcoords), Arc::new(normals), indices, Arc::new(Matte::new(Box::new(ConstantTexture::new(Spectrum::new(0.2, 0.3, 0.6)))))));
                }

                meshes
            } ,
            Err(e) => {
                panic!("Failed to load {:?} due to {:?}", file_name, e);
            }
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
    texcoords: Arc<Vec<Point2<f64>>>,
    normals: Arc<Vec<Vector3<f64>>>,
    material: Arc<dyn Material>,
    object_to_world: Transform,
}

impl Triangle {
    pub fn new(a:usize, b:usize, c:usize, positions: Arc<Vec<Point3<f64>>>, texcoords: Arc<Vec<Point2<f64>>>, normals: Arc<Vec<Vector3<f64>>>, material: Arc<dyn Material>) -> Self {
        Triangle { a, b, c, positions, texcoords, normals, material, 
            // object_to_world: Transform::translate(Vector3::new(5.0, -6.0, 25.0)) * Transform::scale(3.0, 3.0, 3.0),
            object_to_world: Transform::translate(Vector3::new(-2.0, -13.0, 22.0)) * Transform::scale(70.0, 70.0, 70.0) * Transform::rotate_y(180.0)
        }
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
        let mut p0 = self.object_to_world.transform_point3(self.positions[self.a]);
        let mut p1 = self.object_to_world.transform_point3(self.positions[self.b]);
        let mut p2 = self.object_to_world.transform_point3(self.positions[self.c]);
        
        // let mut p0 = self.positions[self.a];
        // let mut p1 = self.positions[self.b];
        // let mut p2 = self.positions[self.c];
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
        p0.x += sx * p0.z;
        p0.y += sy * p0.z;
        p0.z *= sz;
        p1.x += sx * p1.z;
        p1.y += sy * p1.z;
        p1.z *= sz;
        p2.x += sx * p2.z;
        p2.y += sy * p2.z;
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
        // compute normal 
        let a = self.object_to_world.transform_point3(self.positions[self.a]);
        let b = self.object_to_world.transform_point3(self.positions[self.b]);
        let c = self.object_to_world.transform_point3(self.positions[self.c]);
        let n = (b-a).cross(c-a).normalize();
        //
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
        let mut p0 = self.object_to_world.transform_point3(self.positions[self.a]);
        let mut p1 = self.object_to_world.transform_point3(self.positions[self.b]);
        let mut p2 = self.object_to_world.transform_point3(self.positions[self.c]);
        
        // in world space
        // let mut p0 = self.positions[self.a];
        // let mut p1 = self.positions[self.a];
        // let mut p2 = self.positions[self.a];
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
        p0.x += sx * p0.z;
        p0.y += sy * p0.z;
        p0.z *= sz;
        p1.x += sx * p1.z;
        p1.y += sy * p1.z;
        p1.z *= sz;
        p2.x += sx * p2.z;
        p2.y += sy * p2.z;
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
        let mut bound = self.object_to_world.transform_bound3(&Bound3::new(self.positions[self.a], self.positions[self.b]).union_point3(self.positions[self.c]));
        let diag = bound.diagonal();
        if diag.x < 0.01 {
            bound.p_max.x += 0.005;
            bound.p_min.x -= 0.005;
        }
        if diag.y < 0.01 {
            bound.p_max.y += 0.005;
            bound.p_min.y -= 0.005;
        }
        if diag.z < 0.01 {
            bound.p_max.z += 0.005;
            bound.p_min.z -= 0.005;
        }

        bound
    }
}
