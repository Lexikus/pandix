extern crate math;

use math::Vec2;
use math::Vec3;
use math::Vec4;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec2_new() {
        let new_vector = Vec2 { x: 0.0, y: 0.0 };
        assert_eq!(Vec2::new(0.0, 0.0), new_vector);
    }

    #[test]
    fn vec2_new_normalized() {
        let new_vector = Vec2 {
            x: 0.4472136,
            y: 0.8944272,
        };
        assert_eq!(Vec2::new_normalized(1.0, 2.0), new_vector);

        let new_vector = Vec2 {
            x: -0.4472136,
            y: -0.8944272,
        };
        assert_eq!(Vec2::new_normalized(-1.0, -2.0), new_vector);
    }

    #[test]
    fn vec2_zero() {
        let new_vector = Vec2 { x: 0.0, y: 0.0 };
        assert_eq!(Vec2::zero(), new_vector);
    }

    #[test]
    fn vec2_one() {
        let new_vector = Vec2 { x: 1.0, y: 1.0 };
        assert_eq!(Vec2::one(), new_vector);
    }

    #[test]
    fn vec2_left() {
        let new_vector = Vec2 { x: -1.0, y: 0.0 };
        assert_eq!(Vec2::left(), new_vector);
    }

    #[test]
    fn vec2_right() {
        let new_vector = Vec2 { x: 1.0, y: 0.0 };
        assert_eq!(Vec2::right(), new_vector);
    }

    #[test]
    fn vec2_up() {
        let new_vector = Vec2 { x: 0.0, y: 1.0 };
        assert_eq!(Vec2::up(), new_vector);
    }

    #[test]
    fn vec2_down() {
        let new_vector = Vec2 { x: 0.0, y: -1.0 };
        assert_eq!(Vec2::down(), new_vector);
    }

    #[test]
    fn vec2_distance() {
        let distance = 3.6055512;
        assert_eq!(Vec2::distance(Vec2::one(), Vec2::new(3.0, 4.0)), distance);

        let distance = 0.0;
        assert_eq!(Vec2::distance(Vec2::one(), Vec2::one()), distance);

        let distance = 1.0;
        assert_eq!(Vec2::distance(Vec2::up(), Vec2::one()), distance);

        let distance = 2.236068;
        assert_eq!(Vec2::distance(Vec2::down(), Vec2::one()), distance);
    }

    #[test]
    fn vec2_dot() {
        let dot = 2.0;
        assert_eq!(Vec2::dot(Vec2::one(), Vec2::one()), dot);

        let dot = 53.0;
        assert_eq!(Vec2::dot(Vec2::new(3.0, 5.0), Vec2::new(6.0, 7.0)), dot);

        let dot = 17.0;
        assert_eq!(Vec2::dot(Vec2::new(-3.0, 5.0), Vec2::new(6.0, 7.0)), dot);
    }

    #[test]
    fn vec2_lerp() {
        let lerp_vector = Vec2 { x: 0.0, y: 0.0 };
        assert_eq!(Vec2::lerp(Vec2::zero(), Vec2::one(), 0.0), lerp_vector);

        let lerp_vector = Vec2 { x: 1.0, y: 1.0 };
        assert_eq!(Vec2::lerp(Vec2::zero(), Vec2::one(), 1.0), lerp_vector);

        let lerp_vector = Vec2 { x: 0.5, y: 0.5 };
        assert_eq!(Vec2::lerp(Vec2::zero(), Vec2::one(), 0.5), lerp_vector);
    }

    #[test]
    fn vec2_angle_rad() {
        let angle_rad = 0.0;
        assert_eq!(
            Vec2::angle_rad(Vec2::new(1.0, 1.0), Vec2::new(2.0, 2.0)),
            angle_rad
        );

        let angle_rad = 0.19228472;
        assert_eq!(
            Vec2::angle_rad(Vec2::new(21.0, 4.0), Vec2::new(5.0, 2.0)),
            angle_rad
        );

        let angle_rad = 0.7853982;
        assert_eq!(
            Vec2::angle_rad(Vec2::new(-1.0, 1.0), Vec2::new(0.0, 0.1)),
            angle_rad
        );

        let angle_rad = 1.5707964;
        assert_eq!(Vec2::angle_rad(Vec2::right(), Vec2::up()), angle_rad);

        // This calculation is invalid. To prevent an option wrapper
        // it returns 90 degree or 1.5707964 rad
        let angle_rad = 1.5707964;
        assert_eq!(Vec2::angle_rad(Vec2::zero(), Vec2::zero()), angle_rad);
    }

    #[test]
    fn vec2_scale() {
        let scaled_vector = Vec2 { x: 0.0, y: 0.0 };
        assert_eq!(Vec2::scale(Vec2::up(), Vec2::zero()), scaled_vector);

        let scaled_vector = Vec2 { x: -1.0, y: 0.0 };
        assert_eq!(Vec2::scale(Vec2::left(), Vec2::one()), scaled_vector);

        let scaled_vector = Vec2 { x: -2.0, y: 0.0 };
        assert_eq!(
            Vec2::scale(Vec2::left(), Vec2::new(2.0, 3.0)),
            scaled_vector
        );
    }

    #[test]
    fn vec2_magnitude() {
        let vec = Vec2::zero();
        let magnitude = 0.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec2::up();
        let magnitude = 1.0;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec2::one();
        let magnitude = 1.4142135;
        assert_eq!(vec.magnitude(), magnitude);

        let vec = Vec2::new(32.0, 4.0);
        let magnitude = 32.24903;
        assert_eq!(vec.magnitude(), magnitude);
    }

    #[test]
    fn vec2_sqrt_magnitude() {
        let vec = Vec2::zero();
        let magnitude = 0.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec2::up();
        let magnitude = 1.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec2::one();
        let magnitude = 2.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);

        let vec = Vec2::new(32.0, 4.0);
        let magnitude = 1040.0;
        assert_eq!(vec.sqrt_magnitude(), magnitude);
    }

    #[test]
    fn vec2_into_normalize() {
        let vec = Vec2::new(1.0, 2.0);
        let new_vector = vec.normalized();
        let normalized_vector = Vec2 {
            x: 0.4472136,
            y: 0.8944272,
        };
        assert_eq!(new_vector, normalized_vector);

        let vec = Vec2::new(-1.0, -2.0);
        let new_vector = vec.normalized();
        let normalized_vector = Vec2 {
            x: -0.4472136,
            y: -0.8944272,
        };
        assert_eq!(new_vector, normalized_vector);
    }

    #[test]
    fn vec2_into() {
        let vec2 = Vec2::zero();
        let vec3: Vec3 = vec2.into();
        assert_eq!(vec3, Vec3::new(0.0, 0.0, 0.0));

        let vec2 = Vec2::zero();
        let vec4: Vec4 = vec2.into();
        assert_eq!(vec4, Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn vec2_from() {
        let vec3 = Vec3::zero();
        let vec2 = Vec2::from(vec3);
        assert_eq!(vec2, Vec2::new(0.0, 0.0));

        let vec4 = Vec4::zero();
        let vec2: Vec2 = Vec2::from(vec4);
        assert_eq!(vec2, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn vec2_neg() {
        let vec2 = -Vec2::one();
        assert_eq!(vec2, Vec2::new(-1.0, -1.0));
    }

    #[test]
    fn vec2_eq() {
        assert!(Vec2::one() == Vec2::one());
        assert!(Vec2::one() != Vec2::zero());
    }

    #[test]
    fn vec2_add() {
        let lhs = Vec2::one();
        let rhs = Vec2::one();
        assert_eq!(lhs + rhs, Vec2::new(2.0, 2.0));

        let lhs = Vec2::one();
        let rhs = Vec3::one();
        assert_eq!(lhs + rhs, Vec2::new(2.0, 2.0));

        let lhs = Vec2::one();
        let rhs = Vec4::one();
        assert_eq!(lhs + rhs, Vec2::new(2.0, 2.0));

        let mut lhs = Vec2::one();
        lhs += Vec2::one();
        assert_eq!(lhs, Vec2::new(2.0, 2.0));
    }

    #[test]
    fn vec2_sub() {
        let lhs = Vec2::one();
        let rhs = Vec2::one();
        assert_eq!(lhs - rhs, Vec2::new(0.0, 0.0));

        let lhs = Vec2::one();
        let rhs = Vec3::one();
        assert_eq!(lhs - rhs, Vec2::new(0.0, 0.0));

        let lhs = Vec2::one();
        let rhs = Vec4::one();
        assert_eq!(lhs - rhs, Vec2::new(0.0, 0.0));

        let mut lhs = Vec2::one();
        lhs -= Vec2::one();
        assert_eq!(lhs, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn vec2_mul() {
        let lhs = Vec2::one();
        assert_eq!(lhs * 3f32, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3f64, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3i8, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3i16, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3i32, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3i64, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3u8, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3u16, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3u32, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(lhs * 3u64, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3.0f32 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3.0f64 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3i8 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3i16 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3i32 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3i64 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3u8 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3u16 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3u32 * lhs, Vec2::new(3.0, 3.0));

        let lhs = Vec2::one();
        assert_eq!(3u64 * lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3f32;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3f64;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3i8;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3i16;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3i32;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3i64;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3u8;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3u16;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3u32;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));

        let mut lhs = Vec2::one();
        lhs *= 3u64;
        assert_eq!(lhs, Vec2::new(3.0, 3.0));
    }

    #[test]
    fn vec2_div() {
        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3f32, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3f64, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3i8, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3i16, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3i32, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3i64, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3u8, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3u16, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3u32, Vec2::new(1.0, 1.0));

        let lhs = Vec2::new(3.0, 3.0);
        assert_eq!(lhs / 3u64, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3f32;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3f64;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3i8;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3i16;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3i32;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3i64;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3u8;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3u16;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3u32;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));

        let mut lhs = Vec2::new(3.0, 3.0);
        lhs /= 3u64;
        assert_eq!(lhs, Vec2::new(1.0, 1.0));
    }
}
