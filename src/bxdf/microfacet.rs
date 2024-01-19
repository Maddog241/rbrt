use std::f64::consts::{PI, E};

use cgmath::{InnerSpace, Vector3};

use crate::{utils::{tan2_theta, sin2_phi, cos2_phi, cos2_theta, cos_theta}, spectrum::Spectrum};

use super::{Bxdf, BxdfType, fresnel::Fresnel};

pub enum MicrofacetDistribution {
    TrowbridgeReitz {
        alpha_x: f64,
        alpha_y: f64,
    }
}

impl MicrofacetDistribution {
    pub fn d(&self, wh: Vector3<f64>) -> f64 {
        match self {
            Self::TrowbridgeReitz { alpha_x, alpha_y } => {
                let tan2_theta = tan2_theta(wh);
                if tan2_theta.is_infinite() {
                    return 0.0;
                }

                let sin2_phi = sin2_phi(wh);
                let cos2_phi = cos2_phi(wh);
                let cos2_theta = cos2_theta(wh);
                let cos4_theta = cos2_theta * cos2_theta;

                let e = 1.0 + tan2_theta * 
                        (cos2_phi / (alpha_x*alpha_x) +  sin2_phi / (alpha_y*alpha_y));

                let inv_d = PI * alpha_x * alpha_y * cos4_theta * e * e;

                1.0 / inv_d
            }
        }
    }

    fn lambda(&self, w: Vector3<f64>) -> f64 {
        match self {
            Self::TrowbridgeReitz { alpha_x, alpha_y } => {
                let tan2_theta = tan2_theta(w);
                if tan2_theta.is_infinite() {
                    return 0.0;
                }

                let alpha2 = cos2_phi(w) * alpha_x * alpha_x + sin2_phi(w) * alpha_y * alpha_y;

                (-1.0 + (1.0 + alpha2 * tan2_theta).sqrt()) / 2.0
            }
        }
    }

    pub fn g(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> f64 {
        1.0 / (1.0 + self.lambda(wo) + self.lambda(wi))
    }

    // fn sample_wh(&self, wo: Vector3<f64>, sample: Point2<f64>) -> Vector3<f64> {

    // }

    pub fn roughness_to_alpha(roughness: f64) -> f64 {
        let roughness = roughness.max(1e-3);
        let x = roughness.log(E);
        1.62142 + 0.819955 * x + 0.1734 * x * x + 0.0171201 * x * x * x + 0.000640711 * x * x * x * x
    }
}

// Torrance-Sparrow Model
pub struct MicrofacetReflection {
    distribution: MicrofacetDistribution,
    fresnel: Box<dyn Fresnel>,
    reflectance: Spectrum,
}

impl MicrofacetReflection {
    pub fn new(distribution: MicrofacetDistribution, fresnel: Box<dyn Fresnel>, reflectance: Spectrum) -> Self {
        Self {
            distribution,
            fresnel,
            reflectance,
        }
    }
}


impl Bxdf for MicrofacetReflection {
    fn f(&self, wo: cgmath::Vector3<f64>, wi: cgmath::Vector3<f64>) -> crate::spectrum::Spectrum {
        let wh = (wo + wi).normalize();
        let cos_o = cos_theta(wo);
        let cos_i = cos_theta(wi);

        let (fresnel_term, _, _) = self.fresnel.evaluate(wo.normalize().dot(wh));

        let res = self.reflectance * fresnel_term * self.distribution.g(wo, wi) * self.distribution.d(wh) / (4.0 * cos_o * cos_i);

        // if fresnel_term > 0.6 {
        //     println!("fresnel: {:?}", fresnel_term);
        // }
        // println!("g: {:?}", self.distribution.g(wo, wi));
        // println!("d: {:?}", self.distribution.d(wh));
        // println!("res: {:?}", res);

        res
    }

    fn types(&self) -> i32 {
        BxdfType::Reflection | BxdfType::Glossy
    }

    fn is_delta(&self) -> bool {
        false
    }
}