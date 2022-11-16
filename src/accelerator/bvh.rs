use cgmath::{Point3, EuclideanSpace};

use crate::{primitive::Primitive, geometry::bound3::Bound3};

pub struct BVH {
    primitives: Vec<Box<dyn Primitive>>,
    root: BVHNode, // the tree may contain nothing
}

#[allow(dead_code)]
impl BVH {
    pub fn new(primitives: Vec<Box<dyn Primitive>>) -> Self {
        let mut primitive_infos = Vec::new();
        for i in 0..primitives.len() {
            let info = PrimitiveInfo::new(i, primitives[i].world_bound());
            primitive_infos.push(info);
        }

        let root = Self::recursive_build(&primitives, primitive_infos).expect("do not pass in empty primitive lists");
 
        BVH {
            primitives,
            root,
        }
    }

    fn recursive_build(primitives:&Vec<Box<dyn Primitive>>, primitive_infos: Vec<PrimitiveInfo>) -> Option<BVHNode> {
        if primitive_infos.len() == 0 {
            return None;
        }

        if primitive_infos.len() == 1 {
            return Some(BVHNode::new_leaf(primitives, &[primitive_infos[0].index]));
        }

        // now cut the primitives along one axis
        // first compute the bounding volume for the centroids
        let mut cen_bound = Bound3::new(primitive_infos[0].center, primitive_infos[0].center);
        for i in 1..primitive_infos.len() {
            cen_bound = cen_bound.union_point3(primitive_infos[i].center);
        }

        // check if cen_bound contains only a single point
        if cen_bound.p_max == cen_bound.p_min {
            let mut indexes = Vec::new();

            for info in primitive_infos.iter() {
                indexes.push(info.index);
            }

            return Some(BVHNode::new_leaf(primitives, &indexes));
        }

        // then decide which axis to cut
        let axis = cen_bound.max_extent();

        // sort the infos array along the given axis
        let mut primitive_infos = primitive_infos;
        primitive_infos.sort_unstable_by(|info_a, info_b| {
            info_a.center[axis].partial_cmp(&info_b.center[axis]).unwrap()
        });

        let mid = primitive_infos.len() / 2;
        let right_infos = primitive_infos.split_off(mid);

        let l_child = Self::recursive_build(primitives, primitive_infos);
        let r_child = Self::recursive_build(primitives, right_infos);

        let node = BVHNode::new_interior(l_child, r_child);
        Some(node)
    }
}

impl Primitive for BVH {
    fn intersect(&self, r: &mut crate::geometry::ray::Ray) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        self.root.intersect(r, &self.primitives)
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray) -> Option<f64> {
        if let Some(_) = self.root.bound.intersect_p(r) {
            let mut res = None;

            if let Some(l_node) = &self.root.l_child {
                if let Some(t) = l_node.intersect_p(r, &self.primitives) {
                    res = Some(t);
                }
            } 

            if let Some(r_node) = &self.root.r_child {
                if let Some(t) = r_node.intersect_p(r, &self.primitives) {
                    match res {
                        None => res = Some(t),
                        Some(new_t) => res = Some(t.min(new_t)),
                    }
                }
            }

            res
        } else {
            None
        }
    }

    fn world_bound(&self) -> crate::geometry::bound3::Bound3 {
        self.root.bound.clone()
    }
}

struct BVHNode {
    bound: Bound3,
    l_child: Option<Box<BVHNode>>,
    r_child: Option<Box<BVHNode>>,
    prim_indexes: Vec<usize>, // the index of primitives
}

#[allow(dead_code)]
impl BVHNode {
    fn new_interior(left: Option<BVHNode>, right: Option<BVHNode>) -> Self {
        // left and right could not all be None

        let mut prim_indexes  = Vec::new();
        let mut l_child = None;
        let mut r_child = None;

        let bound = match left {
            Some(l_node) => {
                // push indexes 
                for index in l_node.prim_indexes.iter() {
                    prim_indexes.push(*index);
                }
                let mut left_bound = l_node.bound.clone();

                // check right node
                if let Some(r_node) = right {
                    for index in r_node.prim_indexes.iter() {
                        prim_indexes.push(*index);
                    }

                    left_bound = left_bound.union(&r_node.bound);

                    r_child = Some(Box::new(r_node));
                }

                // update l_child
                l_child = Some(Box::new(l_node));

                // finish
                left_bound
            },
            None => {
                let r_node = right.expect("wrongly passing two None parameters to create an interior node");

                // push indexes
                for index in r_node.prim_indexes.iter() {
                    prim_indexes.push(*index);
                }

                let bound = r_node.bound.clone();
                r_child = Some(Box::new(r_node));

                bound
            }
        };

        BVHNode {
            bound,
            l_child,
            r_child,
            prim_indexes,
        }
    }

    fn new_leaf(primitives: &Vec<Box<dyn Primitive>>, prim_indexes: &[usize]) -> Self {
        // prim_indexes is ensured to be non-empty
        let mut bound = primitives[prim_indexes[0]].world_bound();

        for index in prim_indexes {
            bound = bound.union(&primitives[*index].world_bound());
        }

        BVHNode {
            bound,
            l_child: None,
            r_child: None,
            prim_indexes: prim_indexes.to_vec(),
        }
    }

    fn is_leaf(&self) -> bool {
        if let None = self.l_child {
            if let None = self.r_child {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn intersect(&self, r: &mut crate::geometry::ray::Ray, primitives: &Vec<Box<dyn Primitive>>) -> Option<crate::geometry::interaction::SurfaceInteraction> {
        if let Some(_) = self.bound.intersect_p(r) {
            if self.is_leaf() {
                // check the actual objects
                let mut res = None;
                
                for index in self.prim_indexes.iter() {
                    if let Some(isect) = primitives[*index].intersect(r) {
                        res = Some(isect)
                    }
                }

                res
            } else {
                // is not leaf
                let mut res = None;
                // check left
                if let Some(l_node) = &self.l_child {
                    // check left nodes's bound
                    if let Some(isect) = l_node.intersect(r, primitives) {
                        res = Some(isect);
                    }
                }

                // check right
                if let Some(r_node) = &self.r_child {
                    if let Some(isect) = r_node.intersect(r, primitives) {
                        res = Some(isect);
                    }
                }

                res
            }
        } else {
            None
        }
    }

    fn intersect_p(&self, r: &crate::geometry::ray::Ray, primitives: &Vec<Box<dyn Primitive>>) -> Option<f64> {
        if let Some(_) = self.bound.intersect_p(r) {
            // check if it is leaf node
            if self.is_leaf() {
                // check the actual objects
                let mut res = None;
                
                for index in self.prim_indexes.iter() {
                    if let Some(new_t) = primitives[*index].intersect_p(r) {
                        if res == None {
                            res = Some(new_t);
                        } else {
                            let t = res.unwrap();
                            res = Some(t.min(new_t));
                        }
                    }
                }

                res
            } else {
                // is not leaf
                let mut res = None;
                // check left
                if let Some(l_node) = &self.l_child {
                    // check left nodes's bound
                    if let Some(t) = l_node.intersect_p(r, primitives) {
                        res = Some(t);
                    }
                }

                // check right
                if let Some(r_node) = &self.r_child {
                    if let Some(new_t) = r_node.intersect_p(r, primitives) {
                        match res {
                            None => res = Some(new_t),
                            Some(t) => res = Some(t.min(new_t)),
                        }
                    }
                }

                res
            }
        } else {
            None
        }
   }

    fn world_bound(&self) -> Bound3 {
        self.bound.clone()
    }
}

#[allow(dead_code)]
struct PrimitiveInfo {
    index: usize,
    bound: Bound3,
    center: Point3<f64>,
}

impl PrimitiveInfo {
    fn new(index: usize, bound: Bound3) -> PrimitiveInfo {
        let center = bound.p_min.midpoint(bound.p_max);

        PrimitiveInfo {
            index,
            bound,
            center,
        }
    }
}