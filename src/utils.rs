use cgmath::Vector3;


// the solid angle is in the local shading coordinate system
pub struct SolidAngle {
    w: Vector3<f64>,
}


impl SolidAngle {
    pub fn new(w: Vector3<f64>) -> Self {
        SolidAngle {w}
    }

    pub fn cos_theta(&self) -> f64 {
        self.w.z
    }

    pub fn cos_theta_square(&self) -> f64 {
        self.w.z * self.w.z
    }

    pub fn sin_theta(&self) -> f64 {
        self.sin_theta_square().sqrt()
    }

    pub fn sin_theta_square(&self) -> f64 {
        (1.0 - self.cos_theta_square()).clamp(0.0, 1.0)
    }

    pub fn cos_phi(&self) -> f64 {
        if self.sin_theta() != 0.0 {
            self.w.x / self.sin_theta()
        } else {
            1.0
        }
    }

    pub fn sin_phi(&self) -> f64 {
        if self.sin_theta() != 0.0 {
            self.w.y / self.sin_theta()
        } else {
            0.0
        }
    }
}