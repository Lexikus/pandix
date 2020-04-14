#![allow(dead_code)]

use std::fmt;
use std::ops;

use super::Vec3;
use super::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
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

    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn angle(lhs: Self, rhs: Self) -> f32 {
        let dot = Self::dot(lhs, rhs);
        (2.0 * dot * dot - 1.0).acos()
    }

    pub fn angle_axis(angle_rad: f32, axis: Vec3) -> Self {
        let angle_rad = angle_rad * 0.5;
        let s = angle_rad.sin();

        Self {
            x: s * axis.x,
            y: s * axis.y,
            z: s * axis.z,
            w: angle_rad.cos(),
        }
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    pub fn euler(rotation_angles_in_deg: Vec3) -> Self {
        let half_to_rad = (0.5 * PI) / 180.0;
        let rotation_angles_in_deg = rotation_angles_in_deg * half_to_rad;

        let sx = rotation_angles_in_deg.x.sin();
        let cx = rotation_angles_in_deg.x.cos();
        let sy = rotation_angles_in_deg.y.sin();
        let cy = rotation_angles_in_deg.y.cos();
        let sz = rotation_angles_in_deg.z.sin();
        let cz = rotation_angles_in_deg.z.cos();

        Self {
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            w: cx * cy * cz + sx * sy * sz,
        }
    }

    pub fn inverse(rotation: Self) -> Self {
        let sqrt = rotation.x * rotation.x
            + rotation.y * rotation.y
            + rotation.z * rotation.z
            + rotation.w * rotation.w;
        if sqrt == 0.0 {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            };
        }
        let inv_sqrt = 1.0 / sqrt;

        Self {
            x: -rotation.x * inv_sqrt,
            y: -rotation.y * inv_sqrt,
            z: -rotation.z * inv_sqrt,
            w: rotation.w * inv_sqrt,
        }
    }

    pub fn lerp(lhs: Self, rhs: Self, t: f32) -> Self {
        Self {
            x: lhs.x + t * (rhs.x - lhs.x),
            y: lhs.y + t * (rhs.y - lhs.y),
            z: lhs.z + t * (rhs.z - lhs.z),
            w: lhs.w + t * (rhs.w - lhs.w),
        }
    }
}

impl Quat {
    pub fn normalized(self) -> Self {
        let mut len = self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2);

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

    pub fn rotate_x(&mut self, angle_rad: f32) {}

    pub fn rotate_y(&mut self, angle_rad: f32) {}

    pub fn rotate_z(&mut self, angle_rad: f32) {}
}

impl Default for Quat {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Quat {{ x: {}, y: {} z: {}, w: {} }}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl ops::Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let ax = self.x;
        let ay = self.y;
        let az = self.z;
        let aw = self.w;

        let bx = rhs.x;
        let by = rhs.y;
        let bz = rhs.z;
        let bw = rhs.w;

        Self::Output {
            x: ax * bw + aw * bx + ay * bz - az * by,
            y: ay * bw + aw * by + az * bx - ax * bz,
            z: az * bw + aw * bz + ax * by - ay * bx,
            w: aw * bw - ax * bx - ay * by - az * bz,
        }
    }
}

impl ops::Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let x2 = self.x * 2.0;
        let y2 = self.y * 2.0;
        let z2 = self.z * 2.0;
        let xx = self.x * x2;
        let yy = self.y * y2;
        let zz = self.z * z2;
        let xy = self.x * y2;
        let xz = self.x * z2;
        let yz = self.y * z2;
        let wx = self.w * x2;
        let wy = self.w * y2;
        let wz = self.w * z2;

        Self::Output {
            x: (1.0 - (yy + zz)) * rhs.x + (xy - wz) * rhs.y + (xz + wy) * rhs.z,
            y: (xy + wz) * rhs.x + (1.0 - (xx + zz)) * rhs.y + (yz - wx) * rhs.z,
            z: (xz - wy) * rhs.x + (yz + wx) * rhs.y + (1.0 - (xx + yy)) * rhs.z,
        }
    }
}

impl ops::Mul<Quat> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Quat) -> Self::Output {
        rhs * self
    }
}
