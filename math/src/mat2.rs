#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat2 {
    pub cols: [f32; 4],
}

impl Mat2 {
    pub fn new(x: Vec2, y: Vec2) -> Self {
        Self {
            cols: [x.x, x.y, y.x, y.y],
        }
    }

    pub fn identity() -> Self {
        Self {
            cols: [1.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn zero() -> Self {
        Self {
            cols: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn from_scale(scale: Vec2) -> Self {
        Self {
            cols: [scale.x, 0.0, 0.0, scale.y],
        }
    }

    pub fn from_rotation(angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        Self {
            cols: [cos, sin, -sin, cos],
        }
    }
}

impl Mat2 {
    pub fn determinant(&self) -> f32 {
        self.cols[0] * self.cols[3] - self.cols[2] * self.cols[1]
    }

    pub fn inverse(&mut self) {
        let det = if self.is_invertible() {
            1.0 / self.determinant()
        } else {
            0.0
        };

        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];

        self.cols[0] = yy * det;
        self.cols[1] = -xy * det;
        self.cols[2] = -yx * det;
        self.cols[3] = xx * det;
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn transpose(&mut self) {
        let xy = self.cols[1];
        self.cols[1] = self.cols[2];
        self.cols[2] = xy;
    }

    pub fn rotate(&mut self, angle_rad: f32) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        self.cols[0] = xx * cos + yx * sin;
        self.cols[1] = xy * cos + yy * sin;
        self.cols[2] = xx * -sin + yx * cos;
        self.cols[3] = xy * -sin + yy * cos;
    }

    pub fn scale(&mut self, scalar: Vec2) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];

        self.cols[0] = xx * scalar.x;
        self.cols[1] = xy * scalar.x;
        self.cols[2] = yx * scalar.y;
        self.cols[3] = yy * scalar.y;
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(&self) -> *const f32 {
        &self.cols[0]
    }
}

impl Default for Mat2 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mat2 {{ \n{}, {}\n{}, {}\n }}",
            self.cols[0], self.cols[1], self.cols[2], self.cols[3],
        )
    }
}

impl ops::Mul for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];

        let rxx = rhs.cols[0];
        let rxy = rhs.cols[1];
        let ryx = rhs.cols[2];
        let ryy = rhs.cols[3];

        Self::Output {
            cols: [
                xx * rxx + yx * rxy,
                xy * rxx + yy * rxy,
                xx * ryx + yx * ryy,
                xy * ryx + yy * ryy,
            ],
        }
    }
}

impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vec2 {
            x: self.cols[0] * rhs.x + self.cols[2] * rhs.y,
            y: self.cols[1] * rhs.x + self.cols[3] * rhs.y,
        }
    }
}

impl ops::Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            cols: [
                self.cols[0] * rhs,
                self.cols[1] * rhs,
                self.cols[2] * rhs,
                self.cols[3] * rhs,
            ],
        }
    }
}

impl ops::Mul<f64> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Mat2> for f32 {
    type Output = Mat2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Mat2> for f64 {
    type Output = Mat2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self as f32
    }
}

impl ops::MulAssign for Mat2 {
    fn mul_assign(&mut self, rhs: Mat2) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];

        let rxx = rhs.cols[0];
        let rxy = rhs.cols[1];
        let ryx = rhs.cols[2];
        let ryy = rhs.cols[3];

        self.cols[0] = xx * rxx + yx * rxy;
        self.cols[1] = xy * rxx + yy * rxy;
        self.cols[2] = xx * ryx + yx * ryy;
        self.cols[3] = xy * ryx + yy * ryy;
    }
}

impl ops::MulAssign<f32> for Mat2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.cols[0] = self.cols[0] * rhs;
        self.cols[1] = self.cols[1] * rhs;
        self.cols[2] = self.cols[2] * rhs;
        self.cols[3] = self.cols[3] * rhs;
    }
}

impl ops::MulAssign<f64> for Mat2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32;
    }
}
