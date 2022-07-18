mod geometry;

use cgmath::*;

fn main() {
    let x = Vector3::new(1.0, 1.0, 2.0);
    let y = Vector3::new(0.0, 3.0, 2.0);
    let p = Point3::new(1.0, 0.0, 0.0);
    let p2 = Point3::new(1.0, 0.0, 0.0);
    println!("{:?}", x.cross(y));
    println!("{:?}", p);
    println!("{:?}", p + x);
}
