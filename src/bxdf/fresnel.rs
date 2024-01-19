use cgmath::{Vector3, InnerSpace};

use crate::{spectrum::Spectrum, utils::{cos_theta, is_nan}};

use super::{BxdfType, Bxdf, BxdfSample};

pub trait Fresnel {
    /// returns (f_value, sin_theta_t, cos_theta_t)
    fn evaluate(&self, cos_theta_i: f64) -> (f64, f64, f64);
}

pub struct FresnelSpecular {
    eta_a: f64,  // refractive index outside 
    eta_b: f64,  // refractive index inside
    r: Spectrum,
    t: Spectrum,
}

impl FresnelSpecular {
    pub fn new(eta_a: f64, eta_b: f64, r: Spectrum, t: Spectrum) -> Self {
        Self {
            eta_a,
            eta_b,
            r,
            t,
        }
    }
    fn refract(&self, wi: Vector3<f64>, sin_theta_t: f64, cos_theta_t: f64) -> Vector3<f64> {
        if cos_theta_t.is_nan() {
            panic!()
        }
        let wo_parl = Vector3::new(-wi.x, -wi.y, 0.0).normalize() * sin_theta_t;
        let wo_perp = Vector3::new(0.0, 0.0, -wi.z).normalize() * cos_theta_t;
        let wo = wo_parl + wo_perp;

        if is_nan(wo) {
            panic!("refracted direction is nan")
        }

        wo
    }
}

impl Bxdf for FresnelSpecular {
    fn f(&self, _wo: cgmath::Vector3<f64>, _wi: cgmath::Vector3<f64>) -> Spectrum {
        Spectrum::new(0.0, 0.0, 0.0)
    }

    fn sample_f(&self, wo: cgmath::Vector3<f64>, sample: cgmath::Point2<f64>) -> BxdfSample {
        let (fresnel_term, sin_theta_t, cos_theta_t)= self.evaluate(cos_theta(wo));
        if sample.x < fresnel_term {
            // reflect 
            let wi = Vector3::new(-wo.x, -wo.y, wo.z);
            let pdf = fresnel_term;
            let rho = self.r * fresnel_term / cos_theta(wi).abs();

            BxdfSample {
                rho,
                wi,
                pdf,
                is_delta: self.is_delta()
            }
        } else {
            // refract
            let pdf = 1.0 - fresnel_term;
            let mut ratio2 = (self.eta_a * self.eta_a) / (self.eta_b * self.eta_b);
            if wo.z <= 0.0 { ratio2 = 1.0 / ratio2; }

            let wi = self.refract(wo, sin_theta_t, cos_theta_t);

            let rho = self.t * (1.0-fresnel_term) * ratio2 / cos_theta(wi).abs();

            BxdfSample {
                rho,
                wi,
                pdf, 
                is_delta: self.is_delta()
            }
        }
    }

    fn pdf(&self, _wo: Vector3<f64>, _wi: Vector3<f64>) -> f64 {
        0.0
    }

    fn types(&self) -> i32 {
        BxdfType::Specular | BxdfType::Reflection | BxdfType::Transmission
    }

    fn is_delta(&self) -> bool {
        true
    }
}

impl Fresnel for FresnelSpecular {
    fn evaluate(&self, cos_theta_i: f64) -> (f64, f64, f64) {
        // returns (fresnel, sin_theta_t, cos_theta_t)
        // compute the fresnel term, and the refracted direction(if it exists)
        assert!(cos_theta_i >= -1.0 && cos_theta_i <= 1.0);
        let (eta_i, eta_t) = if cos_theta_i > 0.0 { (self.eta_a, self.eta_b) } else { (self.eta_b, self.eta_a )};
    
        let cos_theta_i = cos_theta_i.abs();
        let sin_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0).sqrt();
        let sin_theta_t = eta_i / eta_t * sin_theta_i;
        if sin_theta_t > 1.0 {
            // total internal reflection
            return (1.0, sin_theta_t, f64::NAN);
        }

        let cos_theta_t = (1.0 - sin_theta_t * sin_theta_t).max(0.0).sqrt();
        assert!(cos_theta_t >= 0.0 && cos_theta_t <= 1.0);

        let fresnel_parl:f64= (eta_t * cos_theta_i - eta_i * cos_theta_t)/
                            (eta_t * cos_theta_i + eta_i * cos_theta_t);
        let fresnel_perp:f64= (eta_i * cos_theta_i - eta_t * cos_theta_t)/
                            (eta_i * cos_theta_i + eta_t * cos_theta_t);
        let fresnel = (fresnel_parl * fresnel_parl + fresnel_perp * fresnel_perp) / 2.0;

        (fresnel, sin_theta_t, cos_theta_t)
    }
}


pub struct FresnelNoOp { }

#[allow(dead_code)]
impl FresnelNoOp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fresnel for FresnelNoOp {
    fn evaluate(&self, _cos_theta_i: f64) -> (f64, f64, f64) {
        (1.0, 1.0, 0.0)
    }
}

pub struct FresnelSchlick {
    eta_a: f64, // outside
    eta_b: f64 // inside
}

impl FresnelSchlick {
    pub fn new(eta_a: f64, eta_b: f64) -> Self {
        Self {
            eta_a, 
            eta_b
        }
    }
}

impl Fresnel for FresnelSchlick {
    fn evaluate(&self, cos_theta_i: f64) -> (f64, f64, f64) {
        let r0 = (self.eta_a - self.eta_b).powf(2.0) / (self.eta_a + self.eta_b).powf(2.0);
        let r = r0 + (1.0 - r0) * (1.0 - cos_theta_i).powf(5.0);

        (r, 1.0, 0.0)
    }
}