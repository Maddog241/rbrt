use crate::geometry::{bound3::Bound3, interaction::GeometryInfo, ray::{Beam, Ray}, transform::Transform}; 
use cgmath::{Point3, Point2, Vector3, EuclideanSpace, InnerSpace};
use std::{sync::Arc, collections::HashMap};

use tobj;

use super::Shape;

pub struct TriangleMesh {
    pub positions: Vec<Point3<f64>>,
    pub texcoords: Vec<Point2<f64>>,
    pub normals: Vec<Vector3<f64>>,
    pub indices: Vec<usize>,
}

impl TriangleMesh {
    pub fn new(positions: Vec<Point3<f64>>, texcoords: Vec<Point2<f64>>, normals: Vec<Vector3<f64>>, indices: Vec<usize>) -> Self {
        TriangleMesh {
            positions,
            texcoords,
            normals,
            indices,
        }
    }

    pub fn load(file_name: &str) -> HashMap<String, Arc<TriangleMesh>> {
        let obj =  tobj::load_obj(
            file_name, 
            &tobj::LoadOptions{
                single_index: true,
                triangulate: true,
                ..Default::default()
            });

        match obj {
            Ok((models, _)) => {
                let mut meshes = HashMap::new();

                for m in models {
                    let name = m.name;
                    println!("loading model: {}", name);

                    let mesh = m.mesh;

                    // check texture coordinates and normals
                    if mesh.normals.is_empty() || mesh.texcoords.is_empty() {
                        println!("Missing mesh normals or texture coordinates in {:?}", file_name);
                    }
                    println!("{} has {} triangles", name, mesh.indices.len() / 3);

                    // 
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
                    meshes.insert(name, Arc::new(TriangleMesh::new(positions, texcoords, normals, indices)));
                }

                meshes
            } ,
            Err(e) => {
                panic!("Failed to load {:?} due to {:?}", file_name, e);
            }
        }
    }
}

pub struct Triangle {
    a: usize,
    b: usize, 
    c: usize,
    mesh: Arc<TriangleMesh>,
    object_to_world: Transform,
}

impl Triangle {
    pub fn new(a:usize, b:usize, c:usize, mesh: Arc<TriangleMesh>, object_to_world: Transform) -> Self {
        Triangle { a, b, c, mesh, object_to_world}
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

impl Shape for Triangle {
    fn intersect(&self, r: &Ray) -> Option<GeometryInfo> {
        let mut p0 = self.object_to_world.transform_point3(self.mesh.positions[self.a]);
        let mut p1 = self.object_to_world.transform_point3(self.mesh.positions[self.b]);
        let mut p2 = self.object_to_world.transform_point3(self.mesh.positions[self.c]);
        
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

        let p = r.at(t);
        // compute normal 
        let a = self.object_to_world.transform_point3(self.mesh.positions[self.a]);
        let b = self.object_to_world.transform_point3(self.mesh.positions[self.b]);
        let c = self.object_to_world.transform_point3(self.mesh.positions[self.c]);
        let n = (b-a).cross(c-a).normalize();
        //

        let wo = -r.d.normalize();
        let geo = GeometryInfo {p, n, t, wo};

        Some(geo)
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        let mut p0 = self.object_to_world.transform_point3(self.mesh.positions[self.a]);
        let mut p1 = self.object_to_world.transform_point3(self.mesh.positions[self.b]);
        let mut p2 = self.object_to_world.transform_point3(self.mesh.positions[self.c]);
        
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
        let mut bound = self.object_to_world.transform_bound3(&self.object_bound());
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

    fn object_bound(&self) -> Bound3 {
        Bound3::new(self.mesh.positions[self.a], self.mesh.positions[self.b]).union_point3(self.mesh.positions[self.c])
    }
}
