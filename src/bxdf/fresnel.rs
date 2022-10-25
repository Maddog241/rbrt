use cgmath::Vector3;

pub trait FresnelDielectric {
    fn evaluate(&self, wi: Vector3<f64>) ->(f64, Option<Vector3<f64>>);
}
