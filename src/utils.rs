use cgmath::{Vector3, InnerSpace};

use crate::spectrum::Spectrum;

// w here are supposed to be in the local coordinate system
pub fn cos_theta(w: Vector3<f64>) -> f64 {
    w.z.abs()
}

pub fn cos2_theta(w: Vector3<f64>) -> f64 {
    w.z * w.z
}

pub fn sin2_theta(w: Vector3<f64>) -> f64 {
    (1.0 - cos2_theta(w)).clamp(0.0, 1.0)
}

pub fn sin_theta(w: Vector3<f64>) -> f64 {
    sin2_theta(w).sqrt()
}

pub fn tan2_theta(w: Vector3<f64>) -> f64 {
    if w.z == 0.0 {
        f64::INFINITY
    } else {
        (w.x*w.x + w.y*w.y) / (w.z*w.z)
    }
}

#[allow(dead_code)]
pub fn tan_theta(w: Vector3<f64>) -> f64 {
    tan2_theta(w).sqrt()
}

pub fn cos_phi(w: Vector3<f64>) -> f64 {
    if sin2_theta(w) != 0.0 {
        w.x / sin_theta(w)
    } else {
        1.0
    }
}

pub fn sin_phi(w: Vector3<f64>) -> f64 {
    if sin2_theta(w) != 0.0 {
        w.y / sin_theta(w)
    } else {
        0.0
    }
}

pub fn cos2_phi(w: Vector3<f64>) -> f64 {
    cos_phi(w) * cos_phi(w)
}

pub fn sin2_phi(w: Vector3<f64>) -> f64 {
    sin_phi(w) * sin_phi(w)
}

#[allow(dead_code)]
pub fn tan_phi(w: Vector3<f64>) -> f64 {
    w.y / w.x
}

pub fn is_nan(color: Vector3<f64>) -> bool {
    color.x.is_nan() || color.y.is_nan() || color.z.is_nan()
}

pub fn perpendicular(n: Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    // n is a normal vector
    assert!(n.magnitude2() != 0.0);
    let n = n.normalize();

    if n.x != 1.0 {
        let v = Vector3::new(0.0, -n.z, n.y).normalize();
        let u = v.cross(n);
        (u, v)
    } else {
        (Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0))
    }
    // let mut w = n;

    // w.x = w.x.abs();
    // w.y = w.y.abs();
    // w.z = w.z.abs();

    // if w.x <= w.y && w.x <= w.z { w.x = 1.0; }
    // else if w.y <= w.x && w.y <= w.z { w.y = 1.0; }
    // else { w.z = 1.0; }


    // let v = w.cross(n).normalize();
    // let u = v.cross(n);

    // if is_nan(u) || is_nan(v) {
    //     panic!("not a num error")
    // }

    // (u, v)
}

pub fn sphere_tangent(n: Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    // n is a normal vector
    assert!(n.magnitude2() != 0.0);
    let h = Vector3::new(0.0, 1.0, 0.0);
    let v = n.cross(h);

    if v.magnitude2() < 1e-3 {
        // h and u is colinear
        return perpendicular(n);
    } 

    let u = v.cross(n);

    (u, v)
}

#[allow(unused)]
pub fn check_spectrum(spectrum: &Spectrum) -> bool {
    spectrum.r >= 0.0 && spectrum.g >= 0.0 && spectrum.b >= 0.0
}

pub fn reflect(wo: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    let cosine = wo.dot(n);

    -wo + 2.0 * n * cosine
}

// pub fn random_2d() -> Point2<f64> {
//     Point2::new(random(), random())
// }

pub fn spherical_direction(sin_theta: f64, cos_theta: f64, phi: f64) -> Vector3<f64> {
    let x = sin_theta * phi.cos();
    let y = sin_theta * phi.sin();
    let z = cos_theta;
    Vector3::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use rand::random;
    use super::*;
    #[test]
    fn test_perpendicular() {
        for _ in 0..10 {
            let w: Vector3<f64> = Vector3::new(random(), random(), random()).normalize();
            let (u, v) = perpendicular(w);
            assert!((u.dot(v)-1.0) < 1e-3);
            assert!((u.dot(w)-1.0) < 1e-3);
            assert!((v.dot(w)-1.0) < 1e-3);
        }
    }

    #[test]
    fn theta() {
        for _ in 0..10 {
            let w= Vector3::new(random(), random(), random()).normalize();

            let sin2_theta = sin2_theta(w);
            let cos2_theta = cos2_theta(w);
            assert!((sin2_theta+cos2_theta-1.0).abs() < 0.0001);

            let tan2_theta = tan2_theta(w);
            let div = sin2_theta / cos2_theta;
            assert!((tan2_theta-div).abs() < 0.0001);
        }
    }

    #[test]
    fn phi() {
        for _ in 0..10 {
            let w =  Vector3::new(random(), random(), random()).normalize();

            let sin_phi = sin_phi(w);
            let cos_phi = cos_phi(w);

            let tan_phi = tan_phi(w);
            assert!((sin_phi/cos_phi - tan_phi).abs() < 0.0001);

            let my_sin_phi = w.y / (w.x*w.x + w.y*w.y).sqrt();
            let my_cos_phi = w.x / (w.x*w.x + w.y*w.y).sqrt();

            assert!((sin_phi - my_sin_phi).abs() < 0.0001);
            assert!((cos_phi - my_cos_phi).abs() < 0.0001);
        }
    }
}