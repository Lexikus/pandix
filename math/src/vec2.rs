#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec3;
use super::Vec4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalize(x: f32, y: f32) -> Self {
        let mut len = x.powi(2) + y.powi(2);

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self {
            x: x * len,
            y: y * len,
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

    pub fn distance(from: Self, to: Self) -> f32 {
        (from - to).magnitude()
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn lerp(lhs: Self, rhs: Self, t: f32) -> Self {
        Self {
            x: lhs.x + t * (rhs.x - lhs.x),
            y: lhs.y + t * (rhs.y - lhs.y),
        }
    }

    pub fn angle_rad(lhs: Self, rhs: Self) -> f32 {
        let lhs_len = lhs.x.powi(2) + lhs.y.powi(2);
        let rhs_len = rhs.x.powi(2) + rhs.y.powi(2);

        let magnitude = lhs_len.sqrt() * rhs_len.sqrt();

        let cos = if magnitude == 0.0 {
            0.0
        } else {
            (lhs.x * rhs.x + lhs.y * rhs.y) / magnitude
        };

        cos.max(-1.0).min(1.0).acos()
    }

    pub fn scale(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.x * rhs.x,
            y: lhs.y * rhs.y,
        }
    }

    // TODO(alex): max
    // TODO(alex): min
    // TODO(alex): move_towards
    // TODO(alex): rotate_towards
    // TODO(alex): sqrt_distance
}

impl Vec2 {
    pub fn magnitude(self) -> f32 {
        self.sqrt_magnitude().sqrt()
    }

    pub fn sqrt_magnitude(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn normalized(self) -> Self {
        let mut len = self.sqrt_magnitude();

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self {
            x: self.x * len,
            y: self.y * len,
        }
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(self) -> *const f32 {
        &self.x
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<Vec3> for Vec2 {
    fn from(vector: Vec3) -> Self {
        Self::new(vector.x, vector.y)
    }
}

impl From<Vec4> for Vec2 {
    fn from(vector: Vec4) -> Self {
        Self::new(vector.x, vector.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<Vec3> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<Vec4> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::AddAssign<Vec3> for Vec2 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::AddAssign<Vec4> for Vec2 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<Vec3> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<Vec4> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl ops::SubAssign<Vec3> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl ops::SubAssign<Vec4> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec4) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self as f32
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / rhs as f32
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        *self /= rhs as f32
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
