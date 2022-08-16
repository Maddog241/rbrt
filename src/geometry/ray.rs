use cgmath::{Point3, Vector3};

pub trait Beam {
    fn at(&self, t: f64) -> Point3<f64>;
}

pub struct Ray {
    pub o: Point3<f64>,
    pub d: Vector3<f64>,
    pub time: f64,
    pub t_max: f64,
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
