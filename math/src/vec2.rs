#![allow(dead_code)]

extern crate nalgebra;

use super::vec3::Vec3;
use super::vec4::Vec4;

use nalgebra::Vector2;
use nalgebra::Vector3;
use nalgebra::Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    inner: Vector2<f32>,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {
            inner: Vector2::new(x, y),
        }
    }

    pub fn x(self) -> f32 {
        self.inner.x
    }

    pub fn y(self) -> f32 {
        self.inner.y
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0)
    }

    pub fn left() -> Self {
        Self::new(-1.0, 0.0)
    }

    pub fn right() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0)
    }

    pub fn down() -> Self {
        Self::new(0.0, -1.0)
    }

    // TODO: as_ptr
    // TODO: self magnitude
    // TODO: self normalized
    // TODO: self sqr_magnitude
    // TODO: types https://docs.unity3d.com/ScriptReference/Vector2.html
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::new(0.0, 0.0)
    }
}

impl From<Vector2<f32>> for Vec2 {
    fn from(vector: Vector2<f32>) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

impl From<Vector2<f64>> for Vec2 {
    fn from(vector: Vector2<f64>) -> Self {
        Vec2::new(vector.x as f32, vector.y as f32)
    }
}

impl From<Vec3> for Vec2 {
    fn from(vector: Vec3) -> Self {
        Vec2::new(vector.x(), vector.y())
    }
}

impl From<Vector3<f32>> for Vec2 {
    fn from(vector: Vector3<f32>) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

impl From<Vector3<f64>> for Vec2 {
    fn from(vector: Vector3<f64>) -> Self {
        Vec2::new(vector.x as f32, vector.y as f32)
    }
}

impl From<Vec4> for Vec2 {
    fn from(vector: Vec4) -> Self {
        Vec2::new(vector.x(), vector.y())
    }
}

impl From<Vector4<f32>> for Vec2 {
    fn from(vector: Vector4<f32>) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

impl From<Vector4<f64>> for Vec2 {
    fn from(vector: Vector4<f64>) -> Self {
        Vec2::new(vector.x as f32, vector.y as f32)
    }
}
