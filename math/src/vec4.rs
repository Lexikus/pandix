#![allow(dead_code)]

extern crate nalgebra;

use nalgebra::Vector4;

pub struct Vec4 {
    inner: Vector4<f32>,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 {
            inner: Vector4::new(x, y, z, w),
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }

    pub fn y(&self) -> f32 {
        self.inner.y
    }

    pub fn z(&self) -> f32 {
        self.inner.z
    }

    pub fn w(&self) -> f32 {
        self.inner.w
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }
}
