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

impl Ray {
    pub fn new(o: Point3<f64>, d: Vector3<f64>, time: f64, t_max: f64) -> Self {
        Ray {
            o, 
            d, 
            time, 
            t_max
        }
    }
}

impl Beam for Ray {
    fn at(&self, t: f64) -> Point3<f64> {
        self.o + self.d * t
    }
}

// pub struct RayDifferential {
//     pub o: Point3<f64>
// }
