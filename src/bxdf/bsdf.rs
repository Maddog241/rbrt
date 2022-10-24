use super::{Bxdf, BxdfType, Spectrum, match_flags};
use cgmath::{Vector3, InnerSpace, Point2};

pub struct Bsdf {
    pub eta_i: f64,  // refractive index inside
    pub eta_o: f64,  // ... outside
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
    pub fn sample_f(&self, wo: Vector3<f64>, sample: Point2<f64>) -> (Spectrum, Vector3<f64>, f64) {
        let (u, v) = (sample.x, sample.y);
        // choose the bxdf to sample
        let index = (u * self.n_bxdfs as f64) as usize;
        let u = (u - index as f64* self.n_bxdfs as f64) * self.n_bxdfs as f64;

        let sample = Point2::new(u, v);

        let wo = self.world_to_local(wo);
        let (f_value, wi, pdf) =  self.bxdfs[index].sample_f(wo, sample);
        let wi = self.local_to_world(wi);
        (f_value, wi, pdf)
    }

    pub fn f(&self, wo: Vector3<f64>, wi: Vector3<f64>) -> Spectrum {
        let reflect: bool = wo.dot(self.ng) * wi.dot(self.ng) > 0.0;

        let wo = self.world_to_local(wo);
        let wi = self.world_to_local(wi);

        let mut ans = Spectrum::new(0.0, 0.0, 0.0);

        for i in 0..self.n_bxdfs {
            if (reflect && (self.bxdfs[i].types() & BxdfType::Reflection as i32) != 0) || 
                (!reflect && (self.bxdfs[i].types() & BxdfType::Transmission as i32) != 0) 
            {
                ans += self.bxdfs[i].f(wo, wi);
            }
        }

        ans
    }
}

