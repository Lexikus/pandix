#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec2;
use super::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3 {
    pub cols: [f32; 9],
}

impl Mat3 {
    pub fn new(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self {
            cols: [x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z],
        }
    }

    pub fn identity() -> Self {
        Self {
            cols: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn zero() -> Self {
        Self {
            cols: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn trs(translate: Vec2, angle_rad: f32, scale: Vec2) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        Self {
            cols: [
                cos * scale.x,
                sin * scale.x,
                0.0,
                -sin * scale.y,
                cos * scale.y,
                0.0,
                translate.x,
                translate.y,
                1.0,
            ],
        }
    }

    pub fn tr(translate: Vec2, angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        Self {
            cols: [cos, sin, 0.0, -sin, cos, 0.0, translate.x, translate.y, 1.0],
        }
    }

    pub fn from_translate(translate: Vec2) -> Self {
        Self {
            cols: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, translate.x, translate.y, 1.0],
        }
    }

    pub fn from_scale(scale: Vec2) -> Self {
        Self {
            cols: [scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn from_rotation(angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        Self {
            cols: [cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Mat3 {
    pub fn determinant(&self) -> f32 {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yx = self.cols[3];
        let yy = self.cols[4];
        let yz = self.cols[5];
        let zx = self.cols[6];
        let zy = self.cols[7];
        let zz = self.cols[8];

        xx * (zz * yy - yz * zy) + xy * (-zz * yx + yz * zx) + xz * (zy * yx - yy * zx)
    }

    pub fn inverse(&mut self) {
        let det = if self.is_invertible() {
            1.0 / self.determinant()
        } else {
            0.0
        };

        let xx = self.cols[0];
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yx = self.cols[3];
        let yy = self.cols[4];
        let yz = self.cols[5];
        let zx = self.cols[6];
        let zy = self.cols[7];
        let zz = self.cols[8];

        let bx = zz * yy - yz * zy;
        let by = -zz * yx + yz * zx;
        let bz = zy * yx - yy * zx;

        self.cols[0] = bx * det;
        self.cols[1] = (-zz * xy + xz * zy) * det;
        self.cols[2] = (yz * xy - xz * yy) * det;
        self.cols[3] = by * det;
        self.cols[4] = (zz * xx - xz * zx) * det;
        self.cols[5] = (-yz * xx + xz * yx) * det;
        self.cols[6] = bz * det;
        self.cols[7] = (-zy * xx + xy * zx) * det;
        self.cols[8] = (yy * xx - xy * yx) * det;
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn transpose(&mut self) {
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yz = self.cols[5];

        self.cols[1] = self.cols[3];
        self.cols[2] = self.cols[6];
        self.cols[3] = xy;
        self.cols[5] = self.cols[7];
        self.cols[6] = xz;
        self.cols[7] = yz;
    }

    pub fn translate(&mut self, translation: Vec2) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yx = self.cols[3];
        let yy = self.cols[4];
        let yz = self.cols[5];
        let zx = self.cols[6];
        let zy = self.cols[7];
        let zz = self.cols[8];

        self.cols[0] = xx;
        self.cols[1] = xy;
        self.cols[2] = xz;
        self.cols[3] = yx;
        self.cols[4] = yy;
        self.cols[5] = yz;
        self.cols[6] = translation.x * xx + translation.y * yx + zx;
        self.cols[7] = translation.x * xy + translation.y * yy + zy;
        self.cols[8] = translation.x * xz + translation.y * yz + zz;
    }

    pub fn rotate(&mut self, angle_rad: f32) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yx = self.cols[3];
        let yy = self.cols[4];
        let yz = self.cols[5];
        let zy = self.cols[6];
        let zx = self.cols[7];
        let zz = self.cols[8];

        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        self.cols[0] = cos * xx + sin * yx;
        self.cols[1] = cos * xy + sin * yy;
        self.cols[2] = cos * xz + sin * yz;
        self.cols[3] = cos * yx - sin * xx;
        self.cols[4] = cos * yy - sin * xy;
        self.cols[5] = cos * yz - sin * xz;
        self.cols[6] = zy;
        self.cols[7] = zx;
        self.cols[8] = zz;
    }

    pub fn scale(&mut self, scalar: Vec2) {
        self.cols[0] = scalar.x * self.cols[0];
        self.cols[1] = scalar.x * self.cols[1];
        self.cols[2] = scalar.x * self.cols[2];
        self.cols[3] = scalar.y * self.cols[3];
        self.cols[4] = scalar.y * self.cols[4];
        self.cols[5] = scalar.y * self.cols[5];
        self.cols[6] = self.cols[6];
        self.cols[7] = self.cols[7];
        self.cols[8] = self.cols[8];
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(&self) -> *const f32 {
        &self.cols[0]
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mat3 {{ \n{}, {}, {}\n{}, {}, {}\n{}, {}, {}\n }}",
            self.cols[0],
            self.cols[1],
            self.cols[2],
            self.cols[3],
            self.cols[4],
            self.cols[5],
            self.cols[6],
            self.cols[7],
            self.cols[8],
        )
    }
}

impl ops::Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yx = self.cols[3];
        let yy = self.cols[4];
        let yz = self.cols[5];
        let zx = self.cols[6];
        let zy = self.cols[7];
        let zz = self.cols[8];
        let rxx = rhs.cols[0];
        let rxy = rhs.cols[1];
        let rxz = rhs.cols[2];
        let ryx = rhs.cols[3];
        let ryy = rhs.cols[4];
        let ryz = rhs.cols[5];
        let rzx = rhs.cols[6];
        let rzy = rhs.cols[7];
        let rzz = rhs.cols[8];

        Self::Output {
            cols: [
                rxx * xx + rxy * yx + rxz * zx,
                rxx * xy + rxy * yy + rxz * zy,
                rxx * xz + rxy * yz + rxz * zz,
                ryx * xx + ryy * yx + ryz * zx,
                ryx * xy + ryy * yy + ryz * zy,
                ryx * xz + ryy * yz + ryz * zz,
                rzx * xx + rzy * yx + rzz * zx,
                rzx * xy + rzy * yy + rzz * zy,
                rzx * xz + rzy * yz + rzz * zz,
            ],
        }
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: rhs.x * self.cols[0] + rhs.y * self.cols[3] + rhs.z * self.cols[6],
            y: rhs.x * self.cols[1] + rhs.y * self.cols[4] + rhs.z * self.cols[7],
            z: rhs.x * self.cols[2] + rhs.y * self.cols[5] + rhs.z * self.cols[8],
        }
    }
}

impl ops::Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            cols: [
                self.cols[0] * rhs,
                self.cols[1] * rhs,
                self.cols[2] * rhs,
                self.cols[3] * rhs,
                self.cols[4] * rhs,
                self.cols[5] * rhs,
                self.cols[6] * rhs,
                self.cols[7] * rhs,
                self.cols[8] * rhs,
            ],
        }
    }
}

impl ops::Mul<f64> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Mat3> for f32 {
    type Output = Mat3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Mat3> for f64 {
    type Output = Mat3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self as f32
    }
}

impl ops::MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Mat3) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let xz = self.cols[2];
        let yx = self.cols[3];
        let yy = self.cols[4];
        let yz = self.cols[5];
        let zx = self.cols[6];
        let zy = self.cols[7];
        let zz = self.cols[8];
        let rxx = rhs.cols[0];
        let rxy = rhs.cols[1];
        let rxz = rhs.cols[2];
        let ryx = rhs.cols[3];
        let ryy = rhs.cols[4];
        let ryz = rhs.cols[5];
        let rzx = rhs.cols[6];
        let rzy = rhs.cols[7];
        let rzz = rhs.cols[8];

        self.cols[0] = rxx * xx + rxy * yx + rxz * zx;
        self.cols[1] = rxx * xy + rxy * yy + rxz * zy;
        self.cols[2] = rxx * xz + rxy * yz + rxz * zz;
        self.cols[3] = ryx * xx + ryy * yx + ryz * zx;
        self.cols[4] = ryx * xy + ryy * yy + ryz * zy;
        self.cols[5] = ryx * xz + ryy * yz + ryz * zz;
        self.cols[6] = rzx * xx + rzy * yx + rzz * zx;
        self.cols[7] = rzx * xy + rzy * yy + rzz * zy;
        self.cols[8] = rzx * xz + rzy * yz + rzz * zz;
    }
}

impl ops::MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.cols[0] = self.cols[0] * rhs;
        self.cols[1] = self.cols[1] * rhs;
        self.cols[2] = self.cols[2] * rhs;
        self.cols[3] = self.cols[3] * rhs;
        self.cols[4] = self.cols[4] * rhs;
        self.cols[5] = self.cols[5] * rhs;
        self.cols[6] = self.cols[6] * rhs;
        self.cols[7] = self.cols[7] * rhs;
        self.cols[8] = self.cols[8] * rhs;
    }
}

impl ops::MulAssign<f64> for Mat3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32;
    }
}
