use super::bound3::Bound3;
use super::interaction::*;
use super::ray::Ray;
use cgmath::{InnerSpace, Matrix4, Point3, SquareMatrix, Vector3, Vector4};
use std::ops::Mul;

pub struct Transform {
    pub m: Matrix4<f64>,
    pub m_inv: Matrix4<f64>,
}

impl Transform {
    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn transform_point3(&self, p: &Point3<f64>) -> Point3<f64> {
        let np = self.m * Vector4::new(p.x, p.y, p.z, 1.0);
        if np.w == 1.0 {
            Point3::new(np.x, np.y, np.z)
        } else {
            Point3::new(np.x / np.w, np.y / np.w, np.z / np.w)
        }
    }

    pub fn transform_vector3(&self, v: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }

    pub fn transform_normal(&self, n: &Vector3<f64>) -> Vector3<f64> {
        // n must be normalized, which is different from that in pbrt
        // do not transpose the inverse matrix explicitly (change iteration method instead).
        // just multiplying the matrix may result in a non-normalized vector, so normalize it in the end.
        Vector3::new(
            self.m_inv[0][0] * n.x + self.m_inv[1][0] * n.y + self.m_inv[2][0] * n.z,
            self.m_inv[0][1] * n.x + self.m_inv[1][1] * n.y + self.m_inv[2][1] * n.z,
            self.m_inv[0][2] * n.x + self.m_inv[1][2] * n.y + self.m_inv[2][2] * n.z,
        )
        .normalize()
    }

    pub fn transform_ray(&self, r: &Ray) -> Ray {
        Ray {
            o: self.transform_point3(&r.o),
            d: self.transform_vector3(&r.d),
            time: r.time,
            t_max: r.t_max,
        }
    }

    pub fn transform_bound3(&self, b: &Bound3) -> Bound3 {
        let p0 = self.transform_point3(&b.corner(0));
        let p1 = self.transform_point3(&b.corner(1));
        let mut new_bound = Bound3::new(p0, p1);
        for i in 2..8 {
            new_bound = new_bound.union_point3(&self.transform_point3(&b.corner(i)));
        }
        new_bound
    }

    pub fn transform_surface_interaction(&self, si: &SurfaceInteraction) -> SurfaceInteraction {
        SurfaceInteraction {
            p: self.transform_point3(&si.p),
            n: self.transform_normal(&si.n),
            t: si.t,
            time: si.time,
            wo: self.transform_vector3(&si.wo),
        }
    }
}

impl Transform {
    pub fn new(m: Matrix4<f64>, m_inv: Matrix4<f64>) -> Transform {
        Transform { m, m_inv }
    }

    pub fn translate(delta: Vector3<f64>) -> Transform {
        Transform {
            m: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                delta.x, delta.y, delta.z, 1.0,
            ),
            m_inv: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                -delta.x, -delta.y, -delta.z, 1.0,
            ),
        }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Transform {
        Transform {
            m: Matrix4::new(
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0, 
                0.0, 0.0, z, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix4::new(
                1.0 / x, 0.0, 0.0, 0.0,
                0.0, 1.0 / y, 0.0, 0.0,
                0.0, 0.0, 1.0 / z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn rotate_x(theta: f64) -> Transform {
        // theta in degree
        // rotate in the counter-clockwise direction
        let theta = theta.to_radians();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Transform {
            m: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, cos_theta, -sin_theta, 0.0, 
                0.0, sin_theta, cos_theta, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, cos_theta, sin_theta, 0.0, 
                0.0, -sin_theta, cos_theta, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn rotate_y(theta: f64) -> Transform {
        // rotate in the counter-clockwise direction
        let theta = theta.to_radians();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Transform {
            m: Matrix4::new(
                cos_theta, 0.0, sin_theta, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                -sin_theta, 0.0, cos_theta, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix4::new(
                cos_theta, 0.0, -sin_theta, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                sin_theta, 0.0, cos_theta, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn rotate_z(theta: f64) -> Transform {
        // rotate in the counter-clockwise direction
        let theta = theta.to_radians();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Transform {
            m: Matrix4::new(
                cos_theta, -sin_theta, 0.0, 0.0, 
                sin_theta, cos_theta, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix4::new(
                cos_theta, sin_theta, 0.0, 0.0, 
                -sin_theta, cos_theta, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn rorate(theta: f64, axis: Vector3<f64>) -> Transform {
        let theta = theta.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let k = 1.0 - cos_theta;
        let (x, y, z) = (axis.x, axis.y, axis.z);

        let m = Matrix4::new(
            x * x * k + cos_theta, x * y * k + z * sin_theta, x * z * k - y * sin_theta, 0.0,
            x * y * k - z * sin_theta, y * y * k + cos_theta, y * z * k + x * sin_theta, 0.0,
            x * z * k + y * sin_theta, y * z * k - x * sin_theta, z * z * k + cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let m_inv = Matrix4::new(
            x * x * k + cos_theta, x * y * k - z * sin_theta, x * z * k + y * sin_theta, 0.0,
            x * y * k + z * sin_theta, y * y * k + cos_theta, y * z * k - x * sin_theta, 0.0,
            x * z * k - y * sin_theta, y * z * k + x * sin_theta, z * z * k + cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Transform { m, m_inv }
    }

    pub fn look_at(pos: Vector3<f64>, look: Vector3<f64>, up: Vector3<f64>) -> Transform {
        // m is the world-to-camera transformation matrix
        let new_z = (look - pos).normalize();
        let new_x = up.cross(new_z).normalize();
        let new_y = new_z.cross(new_x);
        let camera_to_world = Matrix4::new(
            new_x.x, new_x.y, new_x.z, 0.0, 
            new_y.x, new_y.y, new_y.z, 0.0, 
            new_z.x, new_z.y, new_z.z, 0.0, 
            pos.x, pos.y, pos.z, 1.0,
        );

        Transform {
            m: SquareMatrix::invert(&camera_to_world).unwrap(),
            m_inv: camera_to_world,
        }
    }

    pub fn orthographic(n: f64, f: f64) -> Transform {
        Transform::scale(1.0, 1.0, 1.0 / (f - n)) * Transform::translate(Vector3::new(0.0, 0.0, -n))
    }

    pub fn perspective(fov: f64, n: f64, f: f64) -> Transform {
        let persp = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, f / (f - n), 1.0,
            0.0, 0.0, -f * n / (f - n), 0.0,
        );

        let fov = fov.to_radians();
        let inv_tan = 1.0 / (fov / 2.0).tan();
        Transform::scale(inv_tan, inv_tan, 1.0) * Transform::new(persp, persp.invert().unwrap())
    }
}

impl Mul<Transform> for Transform {
    type Output = Self;
    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            m: self.m * rhs.m,
            m_inv: rhs.m_inv * self.m_inv,
        }
    }
}
