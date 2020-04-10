extern crate math;

use math::Vec2;
use math::Vec3;
use math::Vec4;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec4_new() {
        let new_vector = Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(Vec4::new(0.0, 0.0, 0.0, 0.0), new_vector);
    }

    #[test]
    fn vec4_new_normalized() {
        let new_vector = Vec4 {
            x: 0.14744195,
            y: 0.2948839,
            z: 0.5897678,
            w: 0.7372098,
        };
        assert_eq!(Vec4::new_normalized(1.0, 2.0, 4.0, 5.0), new_vector);

        let new_vector = Vec4 {
            x: -0.14744195,
            y: -0.2948839,
            z: -0.5897678,
            w: -0.7372098,
        };
        assert_eq!(Vec4::new_normalized(-1.0, -2.0, -4.0, -5.0), new_vector);
    }

    #[test]
    fn vec4_zero() {
        let new_vector = Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(Vec4::zero(), new_vector);
    }

    #[test]
    fn vec4_one() {
        let new_vector = Vec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert_eq!(Vec4::one(), new_vector);
    }

    #[test]
    fn vec4_distance() {
        let distance = 10.630146;
        assert_eq!(
            Vec4::distance(Vec4::one(), Vec4::new(3.0, 4.0, 7.0, 9.0)),
            distance
        );

        let distance = 0.0;
        assert_eq!(Vec4::distance(Vec4::one(), Vec4::one()), distance);

        let distance = 1.7320508;
        assert_eq!(
            Vec4::distance(Vec4::new(0.0, 1.0, 0.0, 0.0), Vec4::one()),
            distance
        );

        let distance = 2.6457512;
        assert_eq!(
            Vec4::distance(Vec4::new(0.0, -1.0, 0.0, 0.0), Vec4::one()),
            distance
        );
    }

    #[test]
    fn vec4_dot() {
        let dot = 4.0;
        assert_eq!(Vec4::dot(Vec4::one(), Vec4::one()), dot);

        let dot = 3427.0;
        assert_eq!(
            Vec4::dot(
                Vec4::new(3.0, 5.0, 21.0, 34.0),
                Vec4::new(6.0, 7.0, 2.0, 98.0)
            ),
            dot
        );

        let dot = 4118.0;
        assert_eq!(
            Vec4::dot(
                Vec4::new(-3.0, 5.0, 3.0, 87.0),
                Vec4::new(6.0, 7.0, 91.0, 44.0)
            ),
            dot
        );
    }

    #[test]
    fn vec4_lerp() {
        let lerp_vector = Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(Vec4::lerp(Vec4::zero(), Vec4::one(), 0.0), lerp_vector);

        let lerp_vector = Vec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert_eq!(Vec4::lerp(Vec4::zero(), Vec4::one(), 1.0), lerp_vector);

        let lerp_vector = Vec4 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: 0.5,
        };
        assert_eq!(Vec4::lerp(Vec4::zero(), Vec4::one(), 0.5), lerp_vector);
    }

    #[test]
    fn vec4_scale() {
        let scaled_vector = Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(
            Vec4::scale(Vec4::new(0.0, 1.0, 0.0, 0.0), Vec4::zero()),
            scaled_vector
        );

        let scaled_vector = Vec4 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(
            Vec4::scale(Vec4::new(-1.0, 0.0, 0.0, 0.0), Vec4::one()),
            scaled_vector
        );

        let scaled_vector = Vec4 {
            x: -2.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(
            Vec4::scale(
                Vec4::new(-1.0, 0.0, 0.0, 0.0),
                Vec4::new(2.0, 3.0, 0.0, 0.0)
            ),
            scaled_vector
        );
    }

    #[test]
    fn vec4_magnitude() {
        let vec = Vec4::zero();
        let magnitude = 0.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let magnitude = 1.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec4::one();
        let magnitude = 2.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec4::new(32.0, 4.0, 2.0, 89.0);
        let magnitude = 94.683685;
        assert_eq!(vec.magnitude(), magnitude);
    }

    #[test]
    fn vec4_sqrt_magnitude() {
        let vec = Vec4::zero();
        let magnitude = 0.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let magnitude = 1.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec4::one();
        let magnitude = 4.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec4::new(32.0, 4.0, 2.0, 89.0);
        let magnitude = 8965.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);
    }

    #[test]
    fn vec4_into_normalize() {
        let vec = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let new_vector = vec.normalized();
        let normalized_vector = Vec4 {
            x: 0.18257418,
            y: 0.36514837,
            z: 0.5477226,
            w: 0.73029673,
        };
        assert_eq!(new_vector, normalized_vector);

        let vec = Vec4::new(-1.0, -2.0, -3.0, -4.0);
        let new_vector = vec.normalized();
        let normalized_vector = Vec4 {
            x: -0.18257418,
            y: -0.36514837,
            z: -0.5477226,
            w: -0.73029673,
        };
        assert_eq!(new_vector, normalized_vector);
    }

    #[test]
    fn vec4_into() {
        let vec4 = Vec4::zero();
        let vec2: Vec2 = vec4.into();
        assert_eq!(vec2, Vec2::new(0.0, 0.0));

        let vec4 = Vec4::zero();
        let vec3: Vec3 = vec4.into();
        assert_eq!(vec3, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec4_from() {
        let vec2 = Vec2::zero();
        let vec4 = Vec4::from(vec2);
        assert_eq!(vec4, Vec4::new(0.0, 0.0, 0.0, 0.0));

        let vec3 = Vec3::zero();
        let vec4: Vec4 = Vec4::from(vec3);
        assert_eq!(vec4, Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn vec4_neg() {
        let vec4 = -Vec4::one();
        assert_eq!(vec4, Vec4::new(-1.0, -1.0, -1.0, -1.0));
    }

    #[test]
    fn vec4_eq() {
        assert!(Vec4::one() == Vec4::one());
        assert!(Vec4::one() != Vec4::zero());
    }

    #[test]
    fn vec4_add() {
        let lhs = Vec4::one();
        let rhs = Vec4::one();
        assert_eq!(lhs + rhs, Vec4::new(2.0, 2.0, 2.0, 2.0));

        let lhs = Vec4::one();
        let rhs = Vec4::one();
        assert_eq!(lhs + rhs, Vec4::new(2.0, 2.0, 2.0, 2.0));

        let lhs = Vec4::one();
        let rhs = Vec4::one();
        assert_eq!(lhs + rhs, Vec4::new(2.0, 2.0, 2.0, 2.0));

        let mut lhs = Vec4::one();
        lhs += Vec4::one();
        assert_eq!(lhs, Vec4::new(2.0, 2.0, 2.0, 2.0));
    }

    #[test]
    fn vec4_sub() {
        let lhs = Vec4::one();
        let rhs = Vec4::one();
        assert_eq!(lhs - rhs, Vec4::new(0.0, 0.0, 0.0, 0.0));

        let lhs = Vec4::one();
        let rhs = Vec4::one();
        assert_eq!(lhs - rhs, Vec4::new(0.0, 0.0, 0.0, 0.0));

        let lhs = Vec4::one();
        let rhs = Vec4::one();
        assert_eq!(lhs - rhs, Vec4::new(0.0, 0.0, 0.0, 0.0));

        let mut lhs = Vec4::one();
        lhs -= Vec4::one();
        assert_eq!(lhs, Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn vec4_mul() {
        let lhs = Vec4::one();
        assert_eq!(lhs * 3f32, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3f64, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3i8, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3i16, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3i32, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3i64, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3u8, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3u16, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3u32, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(lhs * 3u64, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3.0f32 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3.0f64 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3i8 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3i16 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3i32 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3i64 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3u8 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3u16 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3u32 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let lhs = Vec4::one();
        assert_eq!(3u64 * lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3f32;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3f64;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3i8;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3i16;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3i32;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3i64;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3u8;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3u16;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3u32;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));

        let mut lhs = Vec4::one();
        lhs *= 3u64;
        assert_eq!(lhs, Vec4::new(3.0, 3.0, 3.0, 3.0));
    }

    #[test]
    fn vec4_div() {
        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3f32, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3f64, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i8, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i16, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i32, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3i64, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u8, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u16, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u32, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        assert_eq!(lhs / 3u64, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3f32;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3f64;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3i8;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3i16;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3i32;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3i64;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3u8;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3u16;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3u32;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));

        let mut lhs = Vec4::new(3.0, 3.0, 3.0, 3.0);
        lhs /= 3u64;
        assert_eq!(lhs, Vec4::new(1.0, 1.0, 1.0, 1.0));
    }
}
