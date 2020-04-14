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

    pub fn normalize(x: f32, y: f32, z: f32, w: f32) -> Self {
        let mut len = x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2);

        if len > 0.0 {
            len = 1.0 / len.sqrt();
        }

        Self {
            x: x * len,
            y: y * len,
            z: z * len,
            w: w * len,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn distance(from: Self, to: Self) -> f32 {
        (from - to).magnitude()
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    pub fn lerp(lhs: Self, rhs: Self, t: f32) -> Self {
        Self {
            x: lhs.x + t * (rhs.x - lhs.x),
            y: lhs.y + t * (rhs.y - lhs.y),
            z: lhs.z + t * (rhs.z - lhs.z),
            w: lhs.w + t * (rhs.w - lhs.w),
        }
    }

    pub fn scale(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.x * rhs.x,
            y: lhs.y * rhs.y,
            z: lhs.z * rhs.z,
            w: lhs.w * rhs.w,
        }
    }

    // TODO(alex): max
    // TODO(alex): min
    // TODO(alex): move_towards
    // TODO(alex): sqrt_distance
}

impl Vec4 {
    pub fn magnitude(self) -> f32 {
        self.sqrt_magnitude().sqrt()
    }

    pub fn sqrt_magnitude(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)
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
            w: self.w * len,
        }
    }

    // TODO(alex): Check if we have to provide the ptr address of the first element
    //             or the struct object
    pub fn as_ptr(self) -> *const f32 {
        &self.x
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
        Self::new(vector.x, vector.y, 0.0, 0.0)
    }
}

impl From<Vec3> for Vec4 {
    fn from(vector: Vec3) -> Self {
        Self::new(vector.x, vector.y, vector.z, 0.0)
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
        self * rhs as f32
    }
}

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self as f32
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

impl ops::MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= rhs as f32
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

impl ops::Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / rhs as f32
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

impl ops::DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        *self /= rhs as f32
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
