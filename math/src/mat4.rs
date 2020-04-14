#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Quat;
use super::Vec3;
use super::Vec4;
use super::EPSILON;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub cols: [f32; 16],
}

impl Mat4 {
    pub fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self {
            cols: [
                x.x, x.y, x.z, x.w, y.x, y.y, y.z, y.w, z.x, z.y, z.z, z.w, w.x, w.y, w.z, w.w,
            ],
        }
    }

    pub fn identity() -> Self {
        Self {
            cols: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn zero() -> Self {
        Self {
            cols: [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);

        let xx = f / aspect;
        let yy = f;
        let zz = (far + near) * nf;
        let zw: f32 = -1.0;
        let wz = 2.0 * far * near * nf;

        Self {
            cols: [
                xx, 0.0, 0.0, 0.0, 0.0, yy, 0.0, 0.0, 0.0, 0.0, zz, zw, 0.0, 0.0, wz, 0.0,
            ],
        }
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let lr = 1.0 / (left - right);
        let bt = 1.0 / (bottom - top);
        let nf = 1.0 / (near - far);

        let xx = -2.0 * lr;
        let yy = -2.0 * bt;
        let zz = 2.0 * nf;
        let wx = (left + right) * lr;
        let wy = (top + bottom) * bt;
        let wz = (far + near) * nf;
        let ww: f32 = 1.0;

        Self {
            cols: [
                xx, 0.0, 0.0, 0.0, 0.0, yy, 0.0, 0.0, 0.0, 0.0, zz, 0.0, wx, wy, wz, ww,
            ],
        }
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = 1.0 / (right - left);
        let tb = 1.0 / (top - bottom);
        let nf = 1.0 / (near - far);

        let xx = 2.0 * near * rl;
        let yy = near * 2.0 * tb;
        let zx = (right + left) * rl;
        let zy = (top + bottom) * tb;
        let zz = (far + near) * nf;
        let zw = -1.0;
        let wz = far * near * 2.0 * nf;

        Self {
            cols: [
                xx, 0.0, 0.0, 0.0, 0.0, yy, 0.0, 0.0, zx, zy, zz, zw, 0.0, 0.0, wz, 0.0,
            ],
        }
    }

    pub fn look_at(from: Vec3, to: Vec3, up: Vec3) -> Self {
        if (from.x - to.x).abs() < EPSILON
            && (from.y - to.y).abs() < EPSILON
            && (from.z - to.z).abs() < EPSILON
        {
            return Self::identity();
        }

        let z = (from - to).normalized();
        let x = Vec3::normalize(
            up.y * z.z - up.z * z.x,
            up.z * z.x - up.x * z.z,
            up.x * z.y - up.y * z.x,
        );
        let y = Vec3::normalize(
            z.y * x.z - z.z * x.y,
            z.z * x.x - z.x * x.z,
            z.x * x.y - z.y * x.x,
        );

        Self {
            cols: [
                x.x,
                y.x,
                z.x,
                0.0,
                x.y,
                y.y,
                z.y,
                0.0,
                x.z,
                y.z,
                z.z,
                0.0,
                -(x.x * from.x + x.y * from.y + x.z * from.z),
                -(y.x * from.x + y.y * from.y + y.z * from.z),
                -(z.x * from.x + z.y * from.y + z.z * from.z),
                1.0,
            ],
        }
    }

    pub fn trs(translate: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let qx2 = rotation.x + rotation.x;
        let qy2 = rotation.y + rotation.y;
        let qz2 = rotation.z + rotation.z;
        let qxx = rotation.x * qx2;
        let qxy = rotation.x * qy2;
        let qxz = rotation.x * qz2;
        let qyy = rotation.y * qy2;
        let qyz = rotation.y * qz2;
        let qzz = rotation.z * qz2;
        let qwx = rotation.w * qx2;
        let qwy = rotation.w * qy2;
        let qwz = rotation.w * qz2;

        Self {
            cols: [
                (1.0 - (qyy + qzz)) * scale.x,
                (qxy + qwz) * scale.x,
                (qxz - qwy) * scale.x,
                0.0,
                (qxy - qwz) * scale.y,
                (1.0 - (qxx + qzz)) * scale.y,
                (qyz + qwx) * scale.y,
                0.0,
                (qxz + qwy) * scale.z,
                (qyz - qwx) * scale.z,
                (1.0 - (qxx + qyy)) * scale.z,
                0.0,
                translate.x,
                translate.y,
                translate.z,
                1.0,
            ],
        }
    }

    pub fn tr(translate: Vec3, rotation: Quat) -> Self {
        let qx2 = rotation.x + rotation.x;
        let qy2 = rotation.y + rotation.y;
        let qz2 = rotation.z + rotation.z;
        let qxx = rotation.x * qx2;
        let qxy = rotation.x * qy2;
        let qxz = rotation.x * qz2;
        let qyy = rotation.y * qy2;
        let qyz = rotation.y * qz2;
        let qzz = rotation.z * qz2;
        let qwx = rotation.w * qx2;
        let qwy = rotation.w * qy2;
        let qwz = rotation.w * qz2;

        Self {
            cols: [
                1.0 - (qyy + qzz),
                qxy + qwz,
                qxz - qwy,
                0.0,
                qxy - qwz,
                1.0 - (qxx + qzz),
                qyz + qwx,
                0.0,
                qxz + qwy,
                qyz - qwx,
                1.0 - (qxx + qyy),
                0.0,
                translate.x,
                translate.y,
                translate.z,
                1.0,
            ],
        }
    }

    pub fn from_translate(translate: Vec3) -> Self {
        Self {
            cols: [
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                translate.x,
                translate.y,
                translate.z,
                1.0,
            ],
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            cols: [
                scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ],
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        let qx2 = rotation.x + rotation.x;
        let qy2 = rotation.y + rotation.y;
        let qz2 = rotation.z + rotation.z;
        let qxx = rotation.x * qx2;
        let qyx = rotation.y * qx2;
        let qyy = rotation.y * qy2;
        let qzx = rotation.z * qx2;
        let qzy = rotation.z * qy2;
        let qzz = rotation.z * qz2;
        let qwx = rotation.w * qx2;
        let qwy = rotation.w * qy2;
        let qwz = rotation.w * qz2;

        Self {
            cols: [
                1.0 - qyy - qzz,
                qyx + qwz,
                qzx - qwy,
                0.0,
                qyx - qwz,
                1.0 - qxx - qzz,
                qzy + qwx,
                0.0,
                qzx + qwy,
                qzy - qwx,
                1.0 - qxx - qyy,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        }
    }

    pub fn from_rotation_axis(angle_rad: f32, axis: Vec3) -> Self {
        if axis.sqrt_magnitude() < EPSILON.powi(2) {
            return Self::identity();
        }

        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        let tan = 1.0 - cos;

        let xx = axis.x * axis.x * tan + cos;
        let xy = axis.y * axis.x * tan + axis.z * sin;
        let xz = axis.z * axis.x * tan - axis.y * sin;
        let yx = axis.x * axis.y * tan - axis.z * sin;
        let yy = axis.y * axis.y * tan + cos;
        let yz = axis.z * axis.y * tan + axis.x * sin;
        let zx = axis.x * axis.z * tan + axis.y * sin;
        let zy = axis.y * axis.z * tan - axis.x * sin;
        let zz = axis.z * axis.z * tan + cos;

        Self {
            cols: [
                xx, xy, xz, 0.0, yx, yy, yz, 0.0, zx, zy, zz, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_rotation_unnormalized_axis(angle_rad: f32, axis: Vec3) -> Self {
        let axis = axis.normalized();
        Self::from_rotation_axis(angle_rad, axis)
    }

    pub fn from_rotation_x(angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        Self {
            cols: [
                1.0, 0.0, 0.0, 0.0, 0.0, cos, sin, 0.0, 0.0, -sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_rotation_y(angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        Self {
            cols: [
                cos, 0.0, -sin, 0.0, 0.0, 1.0, 0.0, 0.0, sin, 0.0, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_rotation_z(angle_rad: f32) -> Self {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        Self {
            cols: [
                cos, sin, 0.0, 0.0, -sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl Mat4 {
    pub fn determinant(&self) -> f32 {
        let d00 = self.cols[0] * self.cols[5] - self.cols[1] * self.cols[4];
        let d01 = self.cols[0] * self.cols[6] - self.cols[2] * self.cols[4];
        let d02 = self.cols[0] * self.cols[7] - self.cols[3] * self.cols[4];
        let d03 = self.cols[1] * self.cols[6] - self.cols[2] * self.cols[5];
        let d04 = self.cols[1] * self.cols[7] - self.cols[3] * self.cols[5];
        let d05 = self.cols[2] * self.cols[7] - self.cols[3] * self.cols[6];
        let d06 = self.cols[8] * self.cols[13] - self.cols[9] * self.cols[12];
        let d07 = self.cols[8] * self.cols[14] - self.cols[10] * self.cols[12];
        let d08 = self.cols[8] * self.cols[15] - self.cols[11] * self.cols[12];
        let d09 = self.cols[9] * self.cols[14] - self.cols[10] * self.cols[13];
        let d10 = self.cols[9] * self.cols[15] - self.cols[11] * self.cols[13];
        let d11 = self.cols[10] * self.cols[15] - self.cols[11] * self.cols[14];

        d00 * d11 - d01 * d10 + d02 * d09 + d03 * d08 - d04 * d07 + d05 * d06
    }

    pub fn inverse(&mut self) {
        let det = if self.is_invertible() {
            1.0 / self.determinant()
        } else {
            0.0
        };

        let a00 = self.cols[0];
        let a01 = self.cols[1];
        let a02 = self.cols[2];
        let a03 = self.cols[3];
        let a10 = self.cols[4];
        let a11 = self.cols[5];
        let a12 = self.cols[6];
        let a13 = self.cols[7];
        let a20 = self.cols[8];
        let a21 = self.cols[9];
        let a22 = self.cols[10];
        let a23 = self.cols[11];
        let a30 = self.cols[12];
        let a31 = self.cols[13];
        let a32 = self.cols[14];
        let a33 = self.cols[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        self.cols[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
        self.cols[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
        self.cols[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
        self.cols[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
        self.cols[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
        self.cols[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
        self.cols[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
        self.cols[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
        self.cols[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
        self.cols[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
        self.cols[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
        self.cols[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
        self.cols[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
        self.cols[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
        self.cols[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
        self.cols[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn transpose(&mut self) {
        let c01 = self.cols[1];
        let c02 = self.cols[2];
        let c03 = self.cols[3];
        let c12 = self.cols[6];
        let c13 = self.cols[7];
        let c23 = self.cols[11];

        self.cols[1] = self.cols[4];
        self.cols[2] = self.cols[8];
        self.cols[3] = self.cols[12];
        self.cols[4] = c01;
        self.cols[6] = self.cols[9];
        self.cols[7] = self.cols[13];
        self.cols[8] = c02;
        self.cols[9] = c12;
        self.cols[11] = self.cols[14];
        self.cols[12] = c03;
        self.cols[13] = c13;
        self.cols[14] = c23;
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.cols[12] = self.cols[0] * translation.x
            + self.cols[4] * translation.y
            + self.cols[8] * translation.z
            + self.cols[12];
        self.cols[13] = self.cols[1] * translation.x
            + self.cols[5] * translation.y
            + self.cols[9] * translation.z
            + self.cols[13];
        self.cols[14] = self.cols[2] * translation.x
            + self.cols[6] * translation.y
            + self.cols[10] * translation.z
            + self.cols[14];
        self.cols[15] = self.cols[3] * translation.x
            + self.cols[7] * translation.y
            + self.cols[11] * translation.z
            + self.cols[15];
    }

    pub fn rotate_axis(&mut self, angle_rad: f32, axis: Vec3) {
        if axis.sqrt_magnitude() < EPSILON.powi(2) {
            return;
        }

        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        let tan = 1.0 - cos;

        let a00 = self.cols[0];
        let a01 = self.cols[1];
        let a02 = self.cols[2];
        let a03 = self.cols[3];
        let a10 = self.cols[4];
        let a11 = self.cols[5];
        let a12 = self.cols[6];
        let a13 = self.cols[7];
        let a20 = self.cols[8];
        let a21 = self.cols[9];
        let a22 = self.cols[10];
        let a23 = self.cols[11];

        let b00 = axis.x * axis.x * tan + cos;
        let b01 = axis.y * axis.x * tan + axis.z * sin;
        let b02 = axis.z * axis.x * tan - axis.y * sin;
        let b10 = axis.x * axis.y * tan - axis.z * sin;
        let b11 = axis.y * axis.y * tan + cos;
        let b12 = axis.z * axis.y * tan + axis.x * sin;
        let b20 = axis.x * axis.z * tan + axis.y * sin;
        let b21 = axis.y * axis.z * tan - axis.x * sin;
        let b22 = axis.z * axis.z * tan + cos;

        self.cols[0] = a00 * b00 + a10 * b01 + a20 * b02;
        self.cols[1] = a01 * b00 + a11 * b01 + a21 * b02;
        self.cols[2] = a02 * b00 + a12 * b01 + a22 * b02;
        self.cols[3] = a03 * b00 + a13 * b01 + a23 * b02;
        self.cols[4] = a00 * b10 + a10 * b11 + a20 * b12;
        self.cols[5] = a01 * b10 + a11 * b11 + a21 * b12;
        self.cols[6] = a02 * b10 + a12 * b11 + a22 * b12;
        self.cols[7] = a03 * b10 + a13 * b11 + a23 * b12;
        self.cols[8] = a00 * b20 + a10 * b21 + a20 * b22;
        self.cols[9] = a01 * b20 + a11 * b21 + a21 * b22;
        self.cols[10] = a02 * b20 + a12 * b21 + a22 * b22;
        self.cols[11] = a03 * b20 + a13 * b21 + a23 * b22;
    }

    pub fn rotate_unnormalized_axis(&mut self, angle_rad: f32, axis: Vec3) {
        let axis = axis.normalized();
        self.rotate_axis(angle_rad, axis);
    }

    pub fn rotate_x(&mut self, angle_rad: f32) {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        let a10 = self.cols[4];
        let a11 = self.cols[5];
        let a12 = self.cols[6];
        let a13 = self.cols[7];
        let a20 = self.cols[8];
        let a21 = self.cols[9];
        let a22 = self.cols[10];
        let a23 = self.cols[11];

        self.cols[4] = a10 * cos + a20 * sin;
        self.cols[5] = a11 * cos + a21 * sin;
        self.cols[6] = a12 * cos + a22 * sin;
        self.cols[7] = a13 * cos + a23 * sin;
        self.cols[8] = a20 * cos - a10 * sin;
        self.cols[9] = a21 * cos - a11 * sin;
        self.cols[10] = a22 * cos - a12 * sin;
        self.cols[11] = a23 * cos - a13 * sin;
    }

    pub fn rotate_y(&mut self, angle_rad: f32) {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        let a00 = self.cols[0];
        let a01 = self.cols[1];
        let a02 = self.cols[2];
        let a03 = self.cols[3];
        let a20 = self.cols[8];
        let a21 = self.cols[9];
        let a22 = self.cols[10];
        let a23 = self.cols[11];

        self.cols[0] = a00 * cos - a20 * sin;
        self.cols[1] = a01 * cos - a21 * sin;
        self.cols[2] = a02 * cos - a22 * sin;
        self.cols[3] = a03 * cos - a23 * sin;
        self.cols[8] = a00 * sin + a20 * cos;
        self.cols[9] = a01 * sin + a21 * cos;
        self.cols[10] = a02 * sin + a22 * cos;
        self.cols[11] = a03 * sin + a23 * cos;
    }

    pub fn rotate_z(&mut self, angle_rad: f32) {
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();

        let a00 = self.cols[0];
        let a01 = self.cols[1];
        let a02 = self.cols[2];
        let a03 = self.cols[3];
        let a10 = self.cols[4];
        let a11 = self.cols[5];
        let a12 = self.cols[6];
        let a13 = self.cols[7];

        self.cols[0] = a00 * cos + a10 * sin;
        self.cols[1] = a01 * cos + a11 * sin;
        self.cols[2] = a02 * cos + a12 * sin;
        self.cols[3] = a03 * cos + a13 * sin;
        self.cols[4] = a10 * cos - a00 * sin;
        self.cols[5] = a11 * cos - a01 * sin;
        self.cols[6] = a12 * cos - a02 * sin;
        self.cols[7] = a13 * cos - a03 * sin;
    }

    pub fn scale(&mut self, scalar: Vec3) {
        self.cols[0] = self.cols[0] * scalar.x;
        self.cols[1] = self.cols[1] * scalar.x;
        self.cols[2] = self.cols[2] * scalar.x;
        self.cols[3] = self.cols[3] * scalar.x;
        self.cols[4] = self.cols[4] * scalar.y;
        self.cols[5] = self.cols[5] * scalar.y;
        self.cols[6] = self.cols[6] * scalar.y;
        self.cols[7] = self.cols[7] * scalar.y;
        self.cols[8] = self.cols[8] * scalar.z;
        self.cols[9] = self.cols[9] * scalar.z;
        self.cols[10] = self.cols[10] * scalar.z;
        self.cols[11] = self.cols[11] * scalar.z;
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(&self) -> *const f32 {
        &self.cols[0]
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mat4 {{ \n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n }}",
            self.cols[0],
            self.cols[1],
            self.cols[2],
            self.cols[3],
            self.cols[4],
            self.cols[5],
            self.cols[6],
            self.cols[7],
            self.cols[8],
            self.cols[9],
            self.cols[10],
            self.cols[11],
            self.cols[12],
            self.cols[13],
            self.cols[14],
            self.cols[15],
        )
    }
}

impl ops::Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output::new(
            Vec4::new(
                self.cols[0] * rhs.cols[0]
                    + self.cols[4] * rhs.cols[1]
                    + self.cols[8] * rhs.cols[2]
                    + self.cols[12] * rhs.cols[3],
                self.cols[1] * rhs.cols[0]
                    + self.cols[5] * rhs.cols[1]
                    + self.cols[9] * rhs.cols[2]
                    + self.cols[13] * rhs.cols[3],
                self.cols[2] * rhs.cols[0]
                    + self.cols[6] * rhs.cols[1]
                    + self.cols[10] * rhs.cols[2]
                    + self.cols[14] * rhs.cols[3],
                self.cols[3] * rhs.cols[0]
                    + self.cols[7] * rhs.cols[1]
                    + self.cols[11] * rhs.cols[2]
                    + self.cols[15] * rhs.cols[3],
            ),
            Vec4::new(
                self.cols[0] * rhs.cols[4]
                    + self.cols[4] * rhs.cols[5]
                    + self.cols[8] * rhs.cols[6]
                    + self.cols[12] * rhs.cols[7],
                self.cols[1] * rhs.cols[4]
                    + self.cols[5] * rhs.cols[5]
                    + self.cols[9] * rhs.cols[6]
                    + self.cols[13] * rhs.cols[7],
                self.cols[2] * rhs.cols[4]
                    + self.cols[6] * rhs.cols[5]
                    + self.cols[10] * rhs.cols[6]
                    + self.cols[14] * rhs.cols[7],
                self.cols[3] * rhs.cols[4]
                    + self.cols[7] * rhs.cols[5]
                    + self.cols[11] * rhs.cols[6]
                    + self.cols[15] * rhs.cols[7],
            ),
            Vec4::new(
                self.cols[0] * rhs.cols[8]
                    + self.cols[4] * rhs.cols[9]
                    + self.cols[8] * rhs.cols[10]
                    + self.cols[12] * rhs.cols[11],
                self.cols[1] * rhs.cols[8]
                    + self.cols[5] * rhs.cols[9]
                    + self.cols[9] * rhs.cols[10]
                    + self.cols[13] * rhs.cols[11],
                self.cols[2] * rhs.cols[8]
                    + self.cols[6] * rhs.cols[9]
                    + self.cols[10] * rhs.cols[10]
                    + self.cols[14] * rhs.cols[11],
                self.cols[3] * rhs.cols[8]
                    + self.cols[7] * rhs.cols[9]
                    + self.cols[11] * rhs.cols[10]
                    + self.cols[15] * rhs.cols[11],
            ),
            Vec4::new(
                self.cols[0] * rhs.cols[12]
                    + self.cols[4] * rhs.cols[13]
                    + self.cols[8] * rhs.cols[14]
                    + self.cols[12] * rhs.cols[15],
                self.cols[1] * rhs.cols[12]
                    + self.cols[5] * rhs.cols[13]
                    + self.cols[9] * rhs.cols[14]
                    + self.cols[13] * rhs.cols[15],
                self.cols[2] * rhs.cols[12]
                    + self.cols[6] * rhs.cols[13]
                    + self.cols[10] * rhs.cols[14]
                    + self.cols[14] * rhs.cols[15],
                self.cols[3] * rhs.cols[12]
                    + self.cols[7] * rhs.cols[13]
                    + self.cols[11] * rhs.cols[14]
                    + self.cols[15] * rhs.cols[15],
            ),
        )
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.cols[0] * rhs.x
                + self.cols[4] * rhs.y
                + self.cols[8] * rhs.z
                + self.cols[12] * rhs.w,
            y: self.cols[1] * rhs.x
                + self.cols[5] * rhs.y
                + self.cols[9] * rhs.z
                + self.cols[13] * rhs.w,
            z: self.cols[2] * rhs.x
                + self.cols[6] * rhs.y
                + self.cols[10] * rhs.z
                + self.cols[14] * rhs.w,
            w: self.cols[3] * rhs.x
                + self.cols[7] * rhs.y
                + self.cols[11] * rhs.z
                + self.cols[15] * rhs.w,
        }
    }
}

impl ops::Mul<f32> for Mat4 {
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
                self.cols[9] * rhs,
                self.cols[10] * rhs,
                self.cols[11] * rhs,
                self.cols[12] * rhs,
                self.cols[13] * rhs,
                self.cols[14] * rhs,
                self.cols[15] * rhs,
            ],
        }
    }
}

impl ops::Mul<f64> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Mat4> for f64 {
    type Output = Mat4;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self as f32
    }
}

impl ops::MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        let xx = self.cols[0] * rhs.cols[0]
            + self.cols[4] * rhs.cols[1]
            + self.cols[8] * rhs.cols[2]
            + self.cols[12] * rhs.cols[3];

        let xy = self.cols[1] * rhs.cols[0]
            + self.cols[5] * rhs.cols[1]
            + self.cols[9] * rhs.cols[2]
            + self.cols[13] * rhs.cols[3];

        let xz = self.cols[2] * rhs.cols[0]
            + self.cols[6] * rhs.cols[1]
            + self.cols[10] * rhs.cols[2]
            + self.cols[14] * rhs.cols[3];

        let xw = self.cols[3] * rhs.cols[0]
            + self.cols[7] * rhs.cols[1]
            + self.cols[11] * rhs.cols[2]
            + self.cols[15] * rhs.cols[3];

        let yx = self.cols[0] * rhs.cols[4]
            + self.cols[4] * rhs.cols[5]
            + self.cols[8] * rhs.cols[6]
            + self.cols[12] * rhs.cols[7];

        let yy = self.cols[1] * rhs.cols[4]
            + self.cols[5] * rhs.cols[5]
            + self.cols[9] * rhs.cols[6]
            + self.cols[13] * rhs.cols[7];

        let yz = self.cols[2] * rhs.cols[4]
            + self.cols[6] * rhs.cols[5]
            + self.cols[10] * rhs.cols[6]
            + self.cols[14] * rhs.cols[7];

        let yw = self.cols[3] * rhs.cols[4]
            + self.cols[7] * rhs.cols[5]
            + self.cols[11] * rhs.cols[6]
            + self.cols[15] * rhs.cols[7];

        let zx = self.cols[0] * rhs.cols[8]
            + self.cols[4] * rhs.cols[9]
            + self.cols[8] * rhs.cols[10]
            + self.cols[12] * rhs.cols[11];

        let zy = self.cols[1] * rhs.cols[8]
            + self.cols[5] * rhs.cols[9]
            + self.cols[9] * rhs.cols[10]
            + self.cols[13] * rhs.cols[11];

        let zz = self.cols[2] * rhs.cols[8]
            + self.cols[6] * rhs.cols[9]
            + self.cols[10] * rhs.cols[10]
            + self.cols[14] * rhs.cols[11];

        let zw = self.cols[3] * rhs.cols[8]
            + self.cols[7] * rhs.cols[9]
            + self.cols[11] * rhs.cols[10]
            + self.cols[15] * rhs.cols[11];

        let wx = self.cols[0] * rhs.cols[12]
            + self.cols[4] * rhs.cols[13]
            + self.cols[8] * rhs.cols[14]
            + self.cols[12] * rhs.cols[15];

        let wy = self.cols[1] * rhs.cols[12]
            + self.cols[5] * rhs.cols[13]
            + self.cols[9] * rhs.cols[14]
            + self.cols[13] * rhs.cols[15];

        let wz = self.cols[2] * rhs.cols[12]
            + self.cols[6] * rhs.cols[13]
            + self.cols[10] * rhs.cols[14]
            + self.cols[14] * rhs.cols[15];

        let ww = self.cols[3] * rhs.cols[12]
            + self.cols[7] * rhs.cols[13]
            + self.cols[11] * rhs.cols[14]
            + self.cols[15] * rhs.cols[15];

        self.cols[0] = xx;
        self.cols[1] = xy;
        self.cols[2] = xz;
        self.cols[3] = xw;
        self.cols[4] = yx;
        self.cols[5] = yy;
        self.cols[6] = yz;
        self.cols[7] = yw;
        self.cols[8] = zx;
        self.cols[9] = zy;
        self.cols[10] = zz;
        self.cols[11] = zw;
        self.cols[12] = wx;
        self.cols[13] = wy;
        self.cols[14] = wz;
        self.cols[15] = ww;
    }
}

impl ops::MulAssign<f32> for Mat4 {
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
        self.cols[9] = self.cols[9] * rhs;
        self.cols[10] = self.cols[10] * rhs;
        self.cols[11] = self.cols[11] * rhs;
        self.cols[12] = self.cols[12] * rhs;
        self.cols[13] = self.cols[13] * rhs;
        self.cols[14] = self.cols[14] * rhs;
        self.cols[15] = self.cols[15] * rhs;
    }
}

impl ops::MulAssign<f64> for Mat4 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32;
    }
}
