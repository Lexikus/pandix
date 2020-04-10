#![allow(dead_code)]

use std::f32::EPSILON;
use std::fmt;
use std::ops;

use super::Quat;
use super::Vec3;
use super::Vec4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub cols: [f32; 16],
}

impl Mat4 {
    pub fn new(x: Vec4, y: Vec4, w: Vec4, z: Vec4) -> Self {
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
        let x = Vec3::new_normalized(
            up.y * z.z - up.z * z.x,
            up.z * z.x - up.x * z.z,
            up.x * z.y - up.y * z.x,
        );
        let y = Vec3::new_normalized(
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

    pub fn from_translate(translate: Vec3) {
        // TODO
    }

    pub fn from_scale(scale: Vec3) {
        // TODO
    }

    pub fn from_rotation(rotation: Quat) {
        // TODO
    }

    pub fn from_rotation_x(angle: f32) {
        // TODO
    }

    pub fn from_rotation_y(angle: f32) {
        // TODO
    }

    pub fn from_rotation_z(angle: f32) {
        // TODO
    }
}

impl Mat4 {
    pub fn determinant() {
        // TODO
    }

    pub fn inverse() {
        // TODO
    }

    pub fn transpose() {
        // TODO
    }

    pub fn is_identity() {
        // TODO
    }
}
