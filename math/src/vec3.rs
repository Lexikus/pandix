#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec2;
use super::Vec4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(x: f32, y: f32, z: f32) -> Self {
        let mut len = x.powi(2) + y.powi(2) + z.powi(2);

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self {
            x: x * len,
            y: y * len,
            z: z * len,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn left() -> Self {
        Self::new(-1.0, 0.0, 0.0)
    }

    pub fn right() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Self {
        Self::new(0.0, -1.0, 0.0)
    }

    pub fn back() -> Self {
        Self::new(0.0, 0.0, -1.0)
    }

    pub fn forward() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn distance(from: Self, to: Self) -> f32 {
        (from - to).magnitude()
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn lerp(lhs: Self, rhs: Self, t: f32) -> Self {
        Self {
            x: lhs.x + t * (rhs.x - lhs.x),
            y: lhs.y + t * (rhs.y - lhs.y),
            z: lhs.z + t * (rhs.z - lhs.z),
        }
    }

    pub fn angle_rad(lhs: Self, rhs: Self) -> f32 {
        let lhs_len = lhs.x.powi(2) + lhs.y.powi(2) + lhs.z.powi(2);
        let rhs_len = rhs.x.powi(2) + rhs.y.powi(2) + rhs.z.powi(2);

        let lhs_magnitude = lhs_len.sqrt();
        let rhs_magnitude = rhs_len.sqrt();
        let magnitude = lhs_magnitude * rhs_magnitude;

        let cos = if magnitude == 0.0 {
            0.0
        } else {
            Self::dot(lhs, rhs) / magnitude
        };

        cos.max(-1.0).min(1.0).acos()
    }

    pub fn scale(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.x * rhs.x,
            y: lhs.y * rhs.y,
            z: lhs.z * rhs.z,
        }
    }

    // TODO(alex): max
    // TODO(alex): min
    // TODO(alex): move_towards
    // TODO(alex): sqrt_distance
}

impl Vec3 {
    pub fn magnitude(self) -> f32 {
        self.sqrt_magnitude().sqrt()
    }

    pub fn sqrt_magnitude(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalized(self) -> Self {
        let mut len = self.sqrt_magnitude();

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self {
            x: self.x * len,
            y: self.y * len,
            z: self.z * len,
        }
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(self) -> *const f32 {
        &self.x
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3 {{ x: {}, y: {} z: {} }}", self.x, self.y, self.z)
    }
}

impl From<Vec2> for Vec3 {
    fn from(vector: Vec2) -> Self {
        Self::new(vector.x, vector.y, 0.0)
    }
}

impl From<Vec4> for Vec3 {
    fn from(vector: Vec4) -> Self {
        Self::new(vector.x, vector.y, vector.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Vec2> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z,
        }
    }
}

impl ops::Add<Vec4> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::AddAssign<Vec2> for Vec3 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z,
        };
    }
}

impl ops::AddAssign<Vec4> for Vec3 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vec2> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z,
        }
    }
}

impl ops::Sub<Vec4> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::SubAssign<Vec2> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z,
        };
    }
}

impl ops::SubAssign<Vec4> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec4) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / rhs as f32
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self /= rhs as f32
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
