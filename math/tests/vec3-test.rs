extern crate math;

use math::Vec2;
use math::Vec3;
use math::Vec4;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec3_new() {
        let new_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), new_vector);
    }

    #[test]
    fn vec3_new_normalized() {
        let new_vector = Vec3 {
            x: 0.21821788,
            y: 0.43643576,
            z: 0.8728715,
        };
        assert_eq!(Vec3::new_normalized(1.0, 2.0, 4.0), new_vector);

        let new_vector = Vec3 {
            x: -0.21821788,
            y: -0.43643576,
            z: -0.8728715,
        };
        assert_eq!(Vec3::new_normalized(-1.0, -2.0, -4.0), new_vector);
    }

    #[test]
    fn vec3_zero() {
        let new_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::zero(), new_vector);
    }

    #[test]
    fn vec3_one() {
        let new_vector = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(Vec3::one(), new_vector);
    }

    #[test]
    fn vec3_left() {
        let new_vector = Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::left(), new_vector);
    }

    #[test]
    fn vec3_right() {
        let new_vector = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::right(), new_vector);
    }

    #[test]
    fn vec3_up() {
        let new_vector = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(Vec3::up(), new_vector);
    }

    #[test]
    fn vec3_down() {
        let new_vector = Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };
        assert_eq!(Vec3::down(), new_vector);
    }

    #[test]
    fn vec3_back() {
        let new_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(Vec3::back(), new_vector);
    }

    #[test]
    fn vec3_forward() {
        let new_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        assert_eq!(Vec3::forward(), new_vector);
    }

    #[test]
    fn vec3_distance() {
        let distance = 7.0;
        assert_eq!(
            Vec3::distance(Vec3::one(), Vec3::new(3.0, 4.0, 7.0)),
            distance
        );

        let distance = 0.0;
        assert_eq!(Vec3::distance(Vec3::one(), Vec3::one()), distance);

        let distance = 1.4142135;
        assert_eq!(Vec3::distance(Vec3::up(), Vec3::one()), distance);

        let distance = 2.4494898;
        assert_eq!(Vec3::distance(Vec3::down(), Vec3::one()), distance);
    }

    #[test]
    fn vec3_dot() {
        let dot = 3.0;
        assert_eq!(Vec3::dot(Vec3::one(), Vec3::one()), dot);

        let dot = 95.0;
        assert_eq!(
            Vec3::dot(Vec3::new(3.0, 5.0, 21.0), Vec3::new(6.0, 7.0, 2.0)),
            dot
        );

        let dot = 290.0;
        assert_eq!(
            Vec3::dot(Vec3::new(-3.0, 5.0, 3.0), Vec3::new(6.0, 7.0, 91.0)),
            dot
        );
    }

    #[test]
    fn vec3_cross() {
        let cross_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::cross(Vec3::zero(), Vec3::zero()), cross_vector);

        let cross_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::cross(Vec3::one(), Vec3::one()), cross_vector);

        let cross_vector = Vec3 {
            x: 1466.0,
            y: -44.9,
            z: -49.5,
        };
        assert_eq!(
            Vec3::cross(Vec3::new(0.3, 12.0, -2.0), Vec3::new(4.0, -5.0, 123.0)),
            cross_vector
        );
    }

    #[test]
    fn vec3_lerp() {
        let lerp_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::lerp(Vec3::zero(), Vec3::one(), 0.0), lerp_vector);

        let lerp_vector = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(Vec3::lerp(Vec3::zero(), Vec3::one(), 1.0), lerp_vector);

        let lerp_vector = Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        };
        assert_eq!(Vec3::lerp(Vec3::zero(), Vec3::one(), 0.5), lerp_vector);
    }

    #[test]
    fn vec3_angle_rad() {
        let angle_rad = 0.0;
        assert_eq!(
            Vec3::angle_rad(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0)),
            angle_rad
        );

        let angle_rad = 1.0183495;
        assert_eq!(
            Vec3::angle_rad(Vec3::new(21.0, 4.0, 3.0), Vec3::new(5.0, 2.0, 12.0)),
            angle_rad
        );

        let angle_rad = 0.68471915;
        assert_eq!(
            Vec3::angle_rad(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(0.0, 0.1, 0.2)),
            angle_rad
        );

        let angle_rad = 1.5707964;
        assert_eq!(Vec3::angle_rad(Vec3::right(), Vec3::up()), angle_rad);

        // This calculation is invalid. To prevent an option wrapper
        // it returns 90 degree or 1.5707964 rad
        let angle_rad = 1.5707964;
        assert_eq!(Vec3::angle_rad(Vec3::zero(), Vec3::zero()), angle_rad);
    }

    #[test]
    fn vec3_scale() {
        let scaled_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::scale(Vec3::up(), Vec3::zero()), scaled_vector);

        let scaled_vector = Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(Vec3::scale(Vec3::left(), Vec3::one()), scaled_vector);

        let scaled_vector = Vec3 {
            x: -2.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(
            Vec3::scale(Vec3::left(), Vec3::new(2.0, 3.0, 0.0)),
            scaled_vector
        );
    }

    #[test]
    fn vec3_magnitude() {
        let vec = Vec3::zero();
        let magnitude = 0.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec3::up();
        let magnitude = 1.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec3::one();
        let magnitude = 1.7320508;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec3::new(32.0, 4.0, 2.0);
        let magnitude = 32.31099;
        assert_eq!(vec.magnitude(), magnitude);
    }

    #[test]
    fn vec3_sqrt_magnitude() {
        let vec = Vec3::zero();
        let magnitude = 0.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec3::up();
        let magnitude = 1.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec3::one();
        let magnitude = 3.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec3::new(32.0, 4.0, 2.0);
        let magnitude = 1044.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);
    }

    #[test]
    fn vec3_into_normalize() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let new_vector = vec.normalized();
        let normalized_vector = Vec3 {
            x: 0.26726124,
            y: 0.5345225,
            z: 0.8017837,
        };
        assert_eq!(new_vector, normalized_vector);

        let vec = Vec3::new(-1.0, -2.0, -3.0);
        let new_vector = vec.normalized();
        let normalized_vector = Vec3 {
            x: -0.26726124,
            y: -0.5345225,
            z: -0.8017837,
        };
        assert_eq!(new_vector, normalized_vector);
    }

    #[test]
    fn vec3_into() {
        let vec3 = Vec3::zero();
        let vec2: Vec2 = vec3.into();
        assert_eq!(vec2, Vec2::new(0.0, 0.0));

        let vec3 = Vec3::zero();
        let vec4: Vec4 = vec3.into();
        assert_eq!(vec4, Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_from() {
        let vec2 = Vec2::zero();
        let vec3 = Vec3::from(vec2);
        assert_eq!(vec3, Vec3::new(0.0, 0.0, 0.0));

        let vec4 = Vec4::zero();
        let vec3: Vec3 = Vec3::from(vec4);
        assert_eq!(vec3, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_neg() {
        let vec3 = -Vec3::one();
        assert_eq!(vec3, Vec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn vec3_eq() {
        assert!(Vec3::one() == Vec3::one());
        assert!(Vec3::one() != Vec3::zero());
    }

    #[test]
    fn vec3_add() {
        let lhs = Vec3::one();
        let rhs = Vec3::one();
        assert_eq!(lhs + rhs, Vec3::new(2.0, 2.0, 2.0));

        let lhs = Vec3::one();
        let rhs = Vec3::one();
        assert_eq!(lhs + rhs, Vec3::new(2.0, 2.0, 2.0));

        let lhs = Vec3::one();
        let rhs = Vec4::one();
        assert_eq!(lhs + rhs, Vec3::new(2.0, 2.0, 2.0));

        let mut lhs = Vec3::one();
        lhs += Vec3::one();
        assert_eq!(lhs, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn vec3_sub() {
        let lhs = Vec3::one();
        let rhs = Vec3::one();
        assert_eq!(lhs - rhs, Vec3::new(0.0, 0.0, 0.0));

        let lhs = Vec3::one();
        let rhs = Vec3::one();
        assert_eq!(lhs - rhs, Vec3::new(0.0, 0.0, 0.0));

        let lhs = Vec3::one();
        let rhs = Vec4::one();
        assert_eq!(lhs - rhs, Vec3::new(0.0, 0.0, 0.0));

        let mut lhs = Vec3::one();
        lhs -= Vec3::one();
        assert_eq!(lhs, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_mul() {
        let lhs = Vec3::one();
        assert_eq!(lhs * 3f32, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3f64, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3i8, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3i16, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3i32, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3i64, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3u8, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3u16, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3u32, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(lhs * 3u64, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3.0f32 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3.0f64 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3i8 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3i16 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3i32 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3i64 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3u8 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3u16 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3u32 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let lhs = Vec3::one();
        assert_eq!(3u64 * lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3f32;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3f64;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3i8;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3i16;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3i32;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3i64;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3u8;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3u16;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3u32;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));

        let mut lhs = Vec3::one();
        lhs *= 3u64;
        assert_eq!(lhs, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn vec3_div() {
        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3f32, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3f64, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i8, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i16, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i32, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i64, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u8, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u16, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u32, Vec3::new(1.0, 1.0, 1.0));

        let lhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u64, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3f32;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3f64;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3i8;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3i16;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3i32;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3i64;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3u8;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3u16;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3u32;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));

        let mut lhs = Vec3::new(3.0, 3.0, 3.0);
        lhs /= 3u64;
        assert_eq!(lhs, Vec3::new(1.0, 1.0, 1.0));
    }
}
