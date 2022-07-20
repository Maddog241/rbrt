use std::{ops::{Add, Sub, Mul}};

use cgmath::*;

pub trait Beam {
    fn at(&self, t: f64) -> Point3<f64>;
}

pub struct Ray {
    pub o: Point3<f64>,
    pub d: Vector3<f64>,
    pub time: f64,
    // pub medium: 
}

impl Beam for Ray {
    fn at(&self, t: f64) -> Point3<f64> {
        self.o + self.d * t
    }
}

// pub struct RayDifferential {
//     pub o: Point3<f64>
// }

pub struct Bound3<T> 
{
    p_min: Point3<T>,
    p_max: Point3<T>,
}

impl<T> Bound3<T> 
where
    T: Ord + Copy,
    T: Add<T, Output=T> + Mul<T, Output = T>,
    Point3<T>: Sub<Vector3<T>, Output = Point3<T>> + Add<Vector3<T>, Output = Point3<T>> + Sub<Point3<T>, Output = Vector3<T>>,
{
    pub fn new(p: &Point3<T>, q: &Point3<T>) -> Bound3<T> {
        Bound3 {
            p_min: Point3::new(
                p.x.min(q.x),
                p.y.min(q.y),
                p.z.min(q.z),
            ),
            p_max: Point3::new(
                p.x.max(q.x),
                p.y.max(q.y),
                p.z.max(q.z)
            )
        }
    }

    pub fn corner(&self, i: usize) -> Point3<T> {
        // i = 0 -> p_min, i = 7 -> p_max
        let x = if i & 1 == 0 {self.p_min.x} else {self.p_max.x};
        let y = if i & 2 == 0 {self.p_min.y} else {self.p_max.y};
        let z = if i & 4 == 0 {self.p_min.z} else {self.p_max.z};
        Point3 {x, y, z}
    }

    pub fn union(&self, b: &Bound3<T>) -> Bound3<T> {
        Bound3 {
            p_min: Point3::new(
                self.p_min.x.min(b.p_min.x),
                self.p_min.y.min(b.p_min.y),
                self.p_min.z.min(b.p_min.z)
            ),
            p_max: Point3::new(
                self.p_max.x.max(b.p_max.x),
                self.p_max.y.max(b.p_max.y),
                self.p_max.z.max(b.p_max.z)
            )
        }
    }

    pub fn union_point(&self, p: &Point3<T>) -> Bound3<T> {
        Bound3 {
            p_min: Point3::new(
                self.p_min.x.min(p.x),
                self.p_min.y.min(p.y),
                self.p_min.z.min(p.z)
            ),
            p_max: Point3::new(
                self.p_max.x.max(p.x),
                self.p_max.y.max(p.y),
                self.p_max.z.max(p.z)
            )
        }
    }

    pub fn intersect(&self, b: &Bound3<T>) -> Bound3<T> {
        Bound3 {
            p_min: Point3::new(
                self.p_min.x.max(b.p_min.x),
                self.p_min.y.max(b.p_min.y),
                self.p_min.z.max(b.p_min.z)
            ),
            p_max: Point3::new(
                self.p_max.x.min(b.p_max.x),
                self.p_max.y.min(b.p_max.y),
                self.p_max.z.min(b.p_max.z)
            )
        }
    }

    pub fn overlaps(&self, b: &Bound3<T>) -> bool {
        let x = (self.p_max.x >= b.p_min.x) && (self.p_min.x <= b.p_max.x);
        let y = (self.p_max.y >= b.p_min.y) && (self.p_min.y <= b.p_max.y);
        let z = (self.p_max.z >= b.p_min.z) && (self.p_min.z <= b.p_max.z);
        x && y && z
    }

    pub fn contains(&self, p: &Point3<T>) -> bool {
        // this corresponds to the 'inside' function on page 79
        self.p_min.x <= p.x && p.x <= self.p_max.x &&
        self.p_min.y <= p.y && p.y <= self.p_max.y && 
        self.p_min.z <= p.z && p.z <= self.p_max.z
    }

    pub fn contains_exlusive(&self, p: &Point3<T>) -> bool {
        // do not consider the upper bound
        self.p_min.x <= p.x && p.x < self.p_max.x &&
        self.p_min.y <= p.y && p.y < self.p_max.y && 
        self.p_min.z <= p.z && p.z < self.p_max.z
    }

    pub fn expand(&self, delta: T) -> Bound3<T> {
        Bound3 {
            p_min: self.p_min - Vector3::new(delta, delta, delta),
            p_max: self.p_max + Vector3::new(delta, delta, delta),
        }
    }

    pub fn diagonal(&self) -> Vector3<T> {
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> T {
        let dia = self.diagonal();
        let a = dia.x * dia.y + dia.x * dia.z + dia.y * dia.z;
        a + a
    }

    pub fn volumn(&self) -> T {
        let dia = self.diagonal();
        dia.x * dia.y * dia.z
    }

    pub fn max_extent(&self) -> usize {
        // return the longest axis
        let dia = self.diagonal();
        if dia.x >= dia.y && dia.x >= dia.z { 0 }
        else if dia.y >= dia.x && dia.y >= dia.z { 1 }
        else { 2 }
    }
}

impl Bound3<f64> {
    pub fn lerp(&self, t: f64) -> Point3<f64> {
        Point3::new(
            self.p_min.x * t + self.p_max.x * (1.0 - t),
            self.p_min.y * t + self.p_max.y * (1.0 - t),
            self.p_min.z * t + self.p_max.z * (1.0 - t),
        )
    }

    pub fn offset(&self, p: &Point3<f64>) -> Vector3<f64> {
        let mut o = p - self.p_min;
        o.x /= self.p_max.x - self.p_min.x;
        o.y /= self.p_max.y - self.p_min.y;
        o.z /= self.p_max.z - self.p_min.z;
        o
    }

    pub fn bounding_sphere(&self, center: &mut Point3<f64>, radius: &mut f64) {
        *center = self.p_min.midpoint(self.p_max);
        *radius = (self.p_max - *center).magnitude();
    }
}