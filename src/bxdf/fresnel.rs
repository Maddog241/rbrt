use cgmath::{Vector3, InnerSpace};

use crate::{spectrum::Spectrum, utils::cos_theta};

use super::{BxdfType, Bxdf};

pub struct FresnelSpecular {
    eta_a: f64,
    eta_b: f64,
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

    fn evaluate(&self, wi: Vector3<f64>) -> (f64, Option<Vector3<f64>>) {
        // compute the fresnel term, and the refracted direction(if it exists)
        let cos_theta_i = wi.z;
        assert!(cos_theta_i >= -1.0 && cos_theta_i <= 1.0);
        let (eta_i, eta_t) = if cos_theta_i > 0.0 { (self.eta_a, self.eta_b) } else { (self.eta_b, self.eta_a )};
    
        let cos_theta_i = cos_theta_i.abs();
        let sin_theta_i = (1.0 - cos_theta_i * cos_theta_i).sqrt();
        let sin_theta_t = eta_i / eta_t * sin_theta_i;
        if sin_theta_t > 1.0 {
            // total internal reflection
            println!("total internal reflection");
            return (1.0, None);
        }

        let cos_theta_t = (1.0 - sin_theta_t * sin_theta_t).sqrt();
        assert!(cos_theta_t >= 0.0 && cos_theta_t <= 1.0);

        let fresnel_parl:f64= (eta_t * cos_theta_i - eta_i * cos_theta_t)/
                            (eta_t * cos_theta_i + eta_i * cos_theta_t);
        let fresnel_perp:f64= (eta_i * cos_theta_i - eta_t * cos_theta_t)/
                            (eta_i * cos_theta_i + eta_t * cos_theta_t);
        let fresnel = (fresnel_parl * fresnel_parl + fresnel_perp * fresnel_perp) / 2.0;

        let wi_parl = Vector3::new(-wi.x, -wi.y, 0.0).normalize() * sin_theta_t;
        let wi_perp = Vector3::new(0.0, 0.0, -wi.z).normalize() * cos_theta_t;
        let wi = wi_parl + wi_perp;

        (fresnel, Some(wi))
    }
}

impl Bxdf for FresnelSpecular {
    fn f(&self, _wo: cgmath::Vector3<f64>, _wi: cgmath::Vector3<f64>) -> Spectrum {
        Spectrum::new(0.0, 0.0, 0.0)
    }

    fn sample_f(&self, wo: cgmath::Vector3<f64>, sample: cgmath::Point2<f64>) -> (Spectrum, cgmath::Vector3<f64>, f64) {
        let (fresnel_term, refracted)= self.evaluate(wo);
        if sample.x < fresnel_term {
            // reflect 
            let wi = Vector3::new(-wo.x, -wo.y, wo.z);
            let pdf = fresnel_term;
            (self.r * fresnel_term / cos_theta(wi).abs() , wi, pdf)
        } else {
            // refract
            let pdf = 1.0 - fresnel_term;
            let mut ratio2 = (self.eta_a * self.eta_a) / (self.eta_b * self.eta_b);
            if wo.z <= 0.0 { ratio2 = 1.0 / ratio2; }

            let wi = refracted.unwrap();

            (self.t * (1.0-fresnel_term) * ratio2 / cos_theta(wi).abs(), wi, pdf)
        }
    }

    fn types(&self) -> i32 {
        BxdfType::Specular | BxdfType::Reflection | BxdfType::Transmission
    }
}
