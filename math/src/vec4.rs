#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec2;
use super::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vec4 {{ x: {}, y: {} z: {}, w: {} }}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl From<Vec2> for Vec4 {
    fn from(vector: Vec2) -> Self {
        Vec4::new(vector.x, vector.y, 0.0, 0.0)
    }
}

impl From<Vec3> for Vec4 {
    fn from(vector: Vec3) -> Self {
        Vec4::new(vector.x, vector.y, vector.z, 0.0)
    }
}

impl ops::Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Add<Vec2> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl ops::Add<Vec3> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w,
        }
    }
}

impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        };
    }
}

impl ops::AddAssign<Vec2> for Vec4 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z,
            w: self.w,
        };
    }
}

impl ops::AddAssign<Vec3> for Vec4 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w,
        };
    }
}

impl ops::Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Sub<Vec2> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl ops::Sub<Vec3> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w,
        }
    }
}

impl ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl ops::SubAssign<Vec2> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z,
            w: self.w,
        };
    }
}

impl ops::SubAssign<Vec3> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w,
        };
    }
}

impl ops::Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<i8> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<i16> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<i32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<i64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<i128> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<u8> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<u16> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<u32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<u64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<u128> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: u128) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            w: self.w * rhs as f32,
        }
    }
}

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        };
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        };
    }
}

impl ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
