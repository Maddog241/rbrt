use cgmath::*;

pub struct Transform {
    m: Matrix4<f64>,
    m_inv: Matrix4<f64>
}

impl Transform {
    pub fn new(m: Matrix4<f64>, m_inv: Matrix4<f64>) -> Transform {
        Transform { m, m_inv }
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn translate(delta: Vector3<f64>) -> Transform {
        Transform {
            m: Matrix4::new(
                1.0, 0.0, 0.0, delta.x,
                0.0, 1.0, 0.0, delta.y,
                0.0, 0.0, 1.0, delta.z,
                0.0, 0.0, 0.0, 1.0
            ),
            m_inv: Matrix4::new(
                1.0, 0.0, 0.0, -delta.x,
                0.0, 1.0, 0.0, -delta.y,
                0.0, 0.0, 1.0, -delta.z,
                0.0, 0.0, 0.0, 1.0
            )
        }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Transform {
        Transform {
            m: Matrix4::new(
                x, 0.0, 0.0, 0.0, 
                0.0, y, 0.0, 0.0, 
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            m_inv: Matrix4::new(
                1.0/x, 0.0, 0.0, 0.0,
                0.0, 1.0/y, 0.0, 0.0,
                0.0, 0.0, 1.0/z, 0.0,
                0.0, 0.0, 0.0, 1.0
            )
        }
    }

    pub fn rotate_x(theta: f64) -> Transform {
        // rotate in the counter-clockwise direction
        let theta = theta.to_radians();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Transform {
            m: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, cos_theta, sin_theta, 0.0,
                0.0, -sin_theta, cos_theta, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            m_inv: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, cos_theta, -sin_theta, 0.0,
                0.0, sin_theta, cos_theta, 0.0,
                0.0, 0.0, 0.0, 1.0
            )
        }
    }

    pub fn rotate_y(theta: f64) -> Transform {
        // rotate in the counter-clockwise direction
        let theta = theta.to_radians();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Transform {
            m: Matrix4::new(
                cos_theta, 0.0, -sin_theta, 0.0,
                0.0, 1.0, 0.0, 0.0,
                sin_theta, 0.0, cos_theta, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            m_inv: Matrix4::new(
                cos_theta, 0.0, sin_theta, 0.0,
                0.0, 1.0, 0.0, 0.0,
                -sin_theta, 0.0, cos_theta, 0.0,
                0.0, 0.0, 0.0, 1.0
            )
        }
    }

    pub fn rotate_z(theta: f64) -> Transform {
        // rotate in the counter-clockwise direction
        let theta = theta.to_radians();
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Transform {
            m: Matrix4::new(
                cos_theta, sin_theta, 0.0, 0.0,
                -sin_theta, cos_theta, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            m_inv: Matrix4::new(
                cos_theta, -sin_theta, 0.0, 0.0,
                sin_theta, cos_theta, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            )
        }
    }

    pub fn rorate(theta: f64, axis: Vector3<f64>) -> Transform {
        let theta = theta.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let k = 1.0 - cos_theta;
        let (x, y, z) = (axis.x, axis.y, axis.z);

        let m = Matrix4::new(
            x * x * k + cos_theta, x * y * k - z * sin_theta, x * z * k + y * sin_theta, 0.0,
            x * y * k + z * sin_theta, y * y * k + cos_theta, y * z * k - x * sin_theta, 0.0, 
            x * z * k - y * sin_theta, y * z * k + x * sin_theta, z * z * k + cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let m_inv = Matrix4::new(
            x * x * k + cos_theta, x * y * k + z * sin_theta, x * z * k - y * sin_theta, 0.0,
            x * y * k - z * sin_theta, y * y * k + cos_theta, y * z * k + x * sin_theta, 0.0, 
            x * z * k + y * sin_theta, y * z * k - x * sin_theta, z * z * k + cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        Transform { m, m_inv }
    }

    pub fn look_at(pos: Vector3<f64>, look: Vector3<f64>, up: Vector3<f64>) -> Transform {
        // m is the world-to-camera transformation matrix
        let new_z = (look - pos).normalize();
        let new_x = new_z.cross(up.normalize());
        let new_y = new_x.cross(new_z);
        let camera_to_world = Matrix4::new(
            new_x.x, new_y.x, new_z.x, 0.0,
            new_x.y, new_y.y, new_z.y, 0.0,
            new_x.z, new_y.z, new_z.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        Transform {
            m: SquareMatrix::invert(&camera_to_world).unwrap(),
            m_inv: camera_to_world,
        }
    }
}
