use core::panic;

use cgmath::{Vector3, InnerSpace};

// w here are supposed to be in the local coordinate system
pub fn cos_theta(w: Vector3<f64>) -> f64 {
    w.z
}

// pub fn cos_theta_square(w: Vector3<f64>) -> f64 {
//     w.z * w.z
// }

// pub fn sin_theta(w: Vector3<f64>) -> f64 {
//     sin_theta_square(w).sqrt()
// }

// pub fn sin_theta_square(w: Vector3<f64>) -> f64 {
//     (1.0 - cos_theta_square(w)).clamp(0.0, 1.0)
// }

// pub fn cos_phi(w: Vector3<f64>) -> f64 {
//     if sin_theta_square(w) != 0.0 {
//         w.x / sin_theta(w)
//     } else {
//         1.0
//     }
// }

// pub fn sin_phi(w: Vector3<f64>) -> f64 {
//     if sin_theta(w) != 0.0 {
//         w.y / sin_theta(w)
//     } else {
//         0.0
//     }
// }

fn is_nan(color: Vector3<f64>) -> bool {
    color.x.is_nan() || color.y.is_nan() || color.z.is_nan()
}

pub fn perpendicular(n: Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    // n is a normal vector
    assert!(n.magnitude2() != 0.0);
    let n = n.normalize();
    let mut w = n;

    w.x = w.x.abs();
    w.y = w.y.abs();
    w.z = w.z.abs();

    if w.x <= w.y && w.x <= w.z { w.x = 1.0; }
    else if w.y <= w.x && w.y <= w.z { w.y = 1.0; }
    else { w.z = 1.0; }


    let u = w.cross(n).normalize();
    let v = n.cross(u);

    if is_nan(u) || is_nan(v) {
        panic!("not a num error")
    }

    (u, v)
}

// pub fn assert_spectrum(spectrum: &Spectrum) -> bool {
//     spectrum.r >= 0.0 && spectrum.g >= 0.0 && spectrum.b >= 0.0
// }

// pub fn random_2d() -> Point2<f64> {
//     Point2::new(random(), random())
// }

#[cfg(test)]
mod tests {
    use rand::random;
    use super::*;
    #[test]
    fn test_perpendicular() {
        for _ in 0..10 {
            let w: Vector3<f64> = Vector3::new(random(), random(), random());
            let (u, v) = perpendicular(w);
            assert!((u.dot(v)-1.0) < 1e-3);
            assert!((u.dot(w)-1.0) < 1e-3);
            assert!((v.dot(w)-1.0) < 1e-3);
        }
    }
}