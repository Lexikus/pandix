#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat2d {
    pub cols: [f32; 6],
}

impl Mat2d {
    pub fn new(x: Vec2, y: Vec2, z: Vec2) -> Self {
        Self {
            cols: [x.x, x.y, y.x, y.y, z.x, z.y],
        }
    }

    pub fn identity() -> Self {
        Self {
            cols: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }

    pub fn zero() -> Self {
        Self {
            cols: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn trs(translate: Vec2, angle_rad: f32, scale: Vec2) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        Self {
            cols: [
                cos * scale.x,
                sin * scale.x,
                -sin * scale.y,
                cos * scale.y,
                translate.x,
                translate.y,
            ],
        }
    }

    pub fn tr(translate: Vec2, angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        Self {
            cols: [cos, sin, -sin, cos, translate.x, translate.y],
        }
    }

    pub fn from_translate(translate: Vec2) -> Self {
        Self {
            cols: [1.0, 0.0, 0.0, 1.0, translate.x, translate.y],
        }
    }

    pub fn from_scale(scale: Vec2) -> Self {
        Self {
            cols: [scale.x, 0.0, 0.0, scale.y, 0.0, 0.0],
        }
    }

    pub fn from_rotation(angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        Self {
            cols: [cos, sin, -sin, cos, 0.0, 0.0],
        }
    }
}

impl Mat2d {
    pub fn determinant(&self) -> f32 {
        self.cols[0] * self.cols[3] - self.cols[1] * self.cols[2]
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

        let tx = self.cols[4];
        let ty = self.cols[5];

        self.cols[0] = yy * det;
        self.cols[1] = -xy * det;
        self.cols[2] = -yx * det;
        self.cols[3] = xx * det;
        self.cols[4] = (yx * ty - yy * tx) * det;
        self.cols[5] = (xy * tx - xx * ty) * det;
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn translate(&mut self, translation: Vec2) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];
        let tx = self.cols[4];
        let ty = self.cols[5];

        self.cols[0] = xx;
        self.cols[1] = xy;
        self.cols[2] = yx;
        self.cols[3] = yy;
        self.cols[4] = xx * translation.x + yx * translation.y + tx;
        self.cols[5] = xy * translation.x + yy * translation.y + ty;
    }

    pub fn rotate(&mut self, angle_rad: f32) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];
        let tx = self.cols[4];
        let ty = self.cols[5];

        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        self.cols[0] = xx * cos + yx * sin;
        self.cols[1] = xy * cos + yy * sin;
        self.cols[2] = xx * -sin + yx * cos;
        self.cols[3] = xy * -sin + yy * cos;
        self.cols[4] = tx;
        self.cols[5] = ty;
    }

    pub fn scale(&mut self, scalar: Vec2) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];
        let tx = self.cols[4];
        let ty = self.cols[5];

        self.cols[0] = xx * scalar.x;
        self.cols[1] = xy * scalar.x;
        self.cols[2] = yx * scalar.y;
        self.cols[3] = yy * scalar.y;
        self.cols[4] = tx;
        self.cols[5] = ty;
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(&self) -> *const f32 {
        &self.cols[0]
    }
}

impl Default for Mat2d {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for Mat2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mat2d {{ \n{}, {}\n{}, {}\n{}, {}\n }}",
            self.cols[0], self.cols[1], self.cols[2], self.cols[3], self.cols[4], self.cols[5],
        )
    }
}

impl ops::Mul for Mat2d {
    type Output = Self;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];
        let tx = self.cols[4];
        let ty = self.cols[5];

        let rxx = rhs.cols[0];
        let rxy = rhs.cols[1];
        let ryx = rhs.cols[2];
        let ryy = rhs.cols[3];
        let rtx = rhs.cols[4];
        let rty = rhs.cols[5];

        Self::Output {
            cols: [
                xx * rxx + yx * rxy,
                xy * rxx + yy * rxy,
                xx * ryx + yx * ryy,
                xy * ryx + yy * ryy,
                xx * rtx + yx * rty + tx,
                xy * rtx + yy * rty + ty,
            ],
        }
    }
}

impl ops::Mul<Vec2> for Mat2d {
    type Output = Vec2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.cols[0] * rhs.x + self.cols[2] * rhs.y + self.cols[4],
            y: self.cols[1] * rhs.x + self.cols[3] * rhs.y + self.cols[5],
        }
    }
}

impl ops::Mul<f32> for Mat2d {
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
            ],
        }
    }
}

impl ops::Mul<f64> for Mat2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Mat2d> for f32 {
    type Output = Mat2d;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Mat2d> for f64 {
    type Output = Mat2d;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self as f32
    }
}

impl ops::MulAssign for Mat2d {
    fn mul_assign(&mut self, rhs: Mat2d) {
        let xx = self.cols[0];
        let xy = self.cols[1];
        let yx = self.cols[2];
        let yy = self.cols[3];
        let tx = self.cols[4];
        let ty = self.cols[5];

        let rxx = rhs.cols[0];
        let rxy = rhs.cols[1];
        let ryx = rhs.cols[2];
        let ryy = rhs.cols[3];
        let rtx = rhs.cols[4];
        let rty = rhs.cols[5];

        self.cols[0] = xx * rxx + yx * rxy;
        self.cols[1] = xy * rxx + yy * rxy;
        self.cols[2] = xx * ryx + yx * ryy;
        self.cols[3] = xy * ryx + yy * ryy;
        self.cols[4] = xx * rtx + yx * rty + tx;
        self.cols[5] = xy * rtx + yy * rty + ty;
    }
}

impl ops::MulAssign<f32> for Mat2d {
    fn mul_assign(&mut self, rhs: f32) {
        self.cols[0] = self.cols[0] * rhs;
        self.cols[1] = self.cols[1] * rhs;
        self.cols[2] = self.cols[2] * rhs;
        self.cols[3] = self.cols[3] * rhs;
        self.cols[4] = self.cols[4] * rhs;
        self.cols[5] = self.cols[5] * rhs;
    }
}

impl ops::MulAssign<f64> for Mat2d {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32;
    }
}
