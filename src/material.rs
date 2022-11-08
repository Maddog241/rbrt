use crate::{bxdf::bsdf::Bsdf, geometry::interaction::SurfaceInteraction, spectrum::Spectrum};

pub mod matte;
pub mod glass;

pub enum Material {
    Matte {
        kd: Spectrum,
    },
    Glass {
        eta_a: f64,
        eta_b: f64,
        r: Spectrum,
        t: Spectrum,
    }
}

impl Material {
    pub fn compute_scattering(&self, isect: &SurfaceInteraction) -> Bsdf {
        match self {
            Self::Matte { kd } => {
                matte::compute_scattering(self, isect)
            },

            Self::Glass { eta_a, eta_b, r, t } => {
                glass::compute_scattering(self, isect)
            }
        }
    }


    pub fn is_specular(&self) -> bool {
        match self {
            Self::Matte { kd } => {
                matte::is_specular(self)
            },
            
            Self::Glass { eta_a, eta_b, r, t } => {
                glass::is_specular(self)
            }
        }
    }
}

impl Material {
    pub fn create_matte(kd: Spectrum) -> Material {
        Material::Matte { kd }
    }

    pub fn create_glass(eta_a: f64, eta_b: f64, r: Spectrum, t: Spectrum) -> Material {
        Material::Glass { eta_a, eta_b, r, t}
    }
}
