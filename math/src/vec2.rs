#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::vec3::Vec3;
use super::vec4::Vec4;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
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

    pub fn new_normalized(x: f32, y: f32) -> Self {
        let mut len = x.powi(2) + y.powi(2);

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self { x: x * len, y: y * len }
    }

    pub fn distance(from: Self, to: Self) -> f32 {
        (from - to).magnitude()
    }

    pub fn magnitude(self) -> f32 {
        self.sqrt_magnitude().sqrt()
    }

    pub fn sqrt_magnitude(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn normalize(&mut self) {
        let mut len = self.x.powi(2) + self.y.powi(2);

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        self.x *= len;
        self.y *= len;
    }

    pub fn as_normalized(self) -> Self {
        let mut len = self.sqrt_magnitude();

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self { x: self.x * len, y: self.y * len }
    }

    /// Gets the pointer to the first element.
    pub fn as_ptr(self) -> *const f32 {
        &self.x
    }

    // TODO: types https://docs.unity3d.com/ScriptReference/Vector2.html
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::new(0.0, 0.0)
    }
}

impl From<Vec3> for Vec2 {
    fn from(vector: Vec3) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

impl From<Vec4> for Vec2 {
    fn from(vector: Vec4) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Add<Vec3> for Vec2 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vec3> for Vec2 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Add<Vec4> for Vec2 {
    type Output = Self;

    fn add(self, other: Vec4) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vec4> for Vec2 {
    type Output = Self;

    fn sub(self, other: Vec4) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
