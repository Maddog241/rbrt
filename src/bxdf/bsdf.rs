use super::{Bxdf, BxdfType, Spectrum, BxdfSample};
use cgmath::{Vector3, InnerSpace, Point2};

pub struct Bsdf {
    pub ns: Vector3<f64>, // shading normal
    pub ng: Vector3<f64>, // geometry normal
    pub ss: Vector3<f64>, // s axis in the shading coordinate
    pub ts: Vector3<f64>, // t axis in the shading coordinate
    pub bxdfs: Vec<Box<dyn Bxdf>>,
    pub n_bxdfs: usize, // the number of bxdfs 
}

impl Bsdf {
    pub fn local_to_world(&self, v: Vector3<f64>) -> Vector3<f64> {
        let (ss, ts, ns) = (self.ss, self.ts, self.ns);

        Vector3::new(
            ss.x * v.x + ts.x * v.y + ns.x * v.z,
            ss.y * v.x + ts.y * v.y + ns.y * v.z,
            ss.z * v.x + ts.z * v.y + ns.z * v.z
        ) 
    }

    pub fn world_to_local(&self, v: Vector3<f64>) -> Vector3<f64> {
        let (ss, ts, ns) = (self.ss, self.ts, self.ns);

        Vector3::new(
            ss.dot(v),
            ts.dot(v),
            ns.dot(v)
        )
    }
}

impl Bsdf{
    pub fn sample_f(&self, wo: Vector3<f64>, sample: Point2<f64>) -> BxdfSample {
        let (u, v) = (sample.x, sample.y);
        // choose the bxdf to sample
        let index = (u * self.n_bxdfs as f64) as usize;
        let u = (u - index as f64 / self.n_bxdfs as f64) * self.n_bxdfs as f64;

        let sample = Point2::new(u, v);

        let wo: Vector3<f64> = self.world_to_local(wo);
        let mut bxdf_sample =  self.bxdfs[index].sample_f(wo, sample);

        // transform wi to world space
        bxdf_sample.wi = self.local_to_world(bxdf_sample.wi);

        // compute pdf
        let wo = self.local_to_world(wo);
        if !bxdf_sample.is_delta {
            bxdf_sample.pdf = self.pdf(wo, bxdf_sample.wi);
        }
        // compute brdf value in world space
        if !bxdf_sample.is_delta {
            bxdf_sample.rho = self.f(wo, bxdf_sample.wi);
        }

        bxdf_sample
    }

    pub fn f(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> Spectrum {
        let reflect: bool = wo.dot(self.ng) * wi.dot(self.ng) > 0.0;

        let wo = self.world_to_local(wo);
        let wi = self.world_to_local(wi);

        let mut ans = Spectrum::black();

        for i in 0..self.n_bxdfs {
            if (reflect && (self.bxdfs[i].types() & BxdfType::Reflection as i32) != 0) || 
                (!reflect && (self.bxdfs[i].types() & BxdfType::Transmission as i32) != 0) 
            {
                ans += self.bxdfs[i].f(wo, wi);
            }
        }

        ans
    }

    // receive world coordinates for wo and wi
    pub fn pdf(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> f64 {
        let mut pdf = 0.0;

        let wo = self.world_to_local(wo);
        let wi = self.world_to_local(wi);

        for i in 0..self.n_bxdfs {
            pdf += self.bxdfs[i].pdf(wo, wi);
        }

        pdf / self.n_bxdfs as f64
    }
}

