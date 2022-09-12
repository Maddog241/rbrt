use cgmath::Vector3;

// w here are supposed to be in the local coordinate system
pub fn cos_theta(w: Vector3<f64>) -> f64 {
    w.z
}

pub fn cos_theta_square(w: Vector3<f64>) -> f64 {
    w.z * w.z
}

pub fn sin_theta(w: Vector3<f64>) -> f64 {
    sin_theta_square(w).sqrt()
}

pub fn sin_theta_square(w: Vector3<f64>) -> f64 {
    (1.0 - cos_theta_square(w)).clamp(0.0, 1.0)
}

pub fn cos_phi(w: Vector3<f64>) -> f64 {
    if sin_theta_square(w) != 0.0 {
        w.x / sin_theta(w)
    } else {
        1.0
    }
}

pub fn sin_phi(w: Vector3<f64>) -> f64 {
    if sin_theta(w) != 0.0 {
        w.y / sin_theta(w)
    } else {
        0.0
    }
}