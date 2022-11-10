use super::{Bxdf, BxdfType};
use super::Spectrum;
use cgmath::{Vector3, Point2};
use std::f64::consts::PI;

const INV_PI: f64 = 1.0 / PI;


pub fn f(lambertian_reflection: &Bxdf, _wo: Vector3<f64>, _wi: Vector3<f64>) -> Spectrum {
    if let Bxdf::LambertianReflection { reflectance } = lambertian_reflection {
        reflectance * INV_PI
    } else {
        panic!()
    }
}

pub fn sample_f(lambertian_reflection: &Bxdf, _wo: Vector3<f64>, sample: Point2<f64>) -> (Spectrum, Vector3<f64>, f64) {
    if let Bxdf::LambertianReflection { reflectance } = lambertian_reflection {
        let pdf = INV_PI / 2.0;

        let theta = sample[0].acos();
        let phi = sample[1] * 2.0 * PI;
        let wi = Vector3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos());

        if wi[0].is_nan() || wi[1].is_nan() || wi[2].is_nan() {
            println!("Not a Number in lambertian");
        }
        (reflectance *  INV_PI, wi, pdf)
    } else {
        panic!()
    }
}

pub fn types(lambertian_reflection: &Bxdf) -> i32 {
    if let Bxdf::LambertianReflection { reflectance: _ } = lambertian_reflection {
        BxdfType::Diffuse | BxdfType::Reflection
    } else {
        panic!()
    }
}