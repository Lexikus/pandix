extern crate math;

use math::Mat3;
use math::Vec2;
use math::Vec3;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat3_new() {
        let x = Vec3::one();
        let y = Vec3::one();
        let z = Vec3::one();
        let mat = Mat3 {
            cols: [x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z],
        };
        assert_eq!(Mat3::new(x, y, z), mat);
    }

    #[test]
    fn mat3_identity() {
        let mat = Mat3 {
            cols: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        };
        assert_eq!(Mat3::identity(), mat);
    }

    #[test]
    fn mat3_zero() {
        let mat = Mat3 {
            cols: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        };
        assert_eq!(Mat3::zero(), mat);
    }

    #[test]
    fn mat3_trs() {
        let mut mat_result = Mat3::identity();
        mat_result.translate(Vec2::new(5.0, 6.0));
        mat_result.rotate(0.2);
        mat_result.scale(Vec2::new(8.0, 9.0));

        assert_eq!(
            Mat3::trs(Vec2::new(5.0, 6.0), 0.2, Vec2::new(8.0, 9.0)),
            mat_result
        );
    }

    #[test]
    fn mat3_tr() {
        let mut mat_result = Mat3::identity();
        mat_result.translate(Vec2::new(5.0, 6.0));
        mat_result.rotate(0.2);

        assert_eq!(Mat3::tr(Vec2::new(5.0, 6.0), 0.2), mat_result);
    }

    #[test]
    fn mat3_from_translate() {
        let pos = Vec2::new(3.0, 4.0);
        let mat = Mat3 {
            cols: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, pos.x, pos.y, 1.0],
        };

        assert_eq!(Mat3::from_translate(pos), mat);
    }

    #[test]
    fn mat3_from_scale() {
        let scale = Vec2::new(3.0, 4.0);
        let mat = Mat3 {
            cols: [scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 1.0],
        };

        assert_eq!(Mat3::from_scale(scale), mat);
    }

    #[test]
    fn mat3_from_rotation() {
        let quat = 0.15;
        let mat = Mat3 {
            cols: [
                0.9887711,
                0.14943814,
                0.0,
                -0.14943814,
                0.9887711,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        };

        assert_eq!(Mat3::from_rotation(quat), mat);
    }

    #[test]
    fn mat3_determinant() {
        let determinant = -0.018000003;
        let mat = Mat3 {
            cols: [0.5, 0.1, 0.2, 0.1, 0.2, 0.4, 0.2, 0.4, 0.6],
        };

        assert_eq!(mat.determinant(), determinant);

        let determinant = 1.0;
        let mat = Mat3::identity();

        assert_eq!(mat.determinant(), determinant);

        let determinant = 0.0;
        let mat = Mat3::zero();

        assert_eq!(mat.determinant(), determinant);
    }

    #[test]
    fn mat3_inverse() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat3 {
            cols: [0.5, 0.1, 0.2, 0.1, 0.2, 0.4, 0.2, 0.4, 0.6],
        };

        let mat_inverse = Mat3::new(
            Vec3::new(2.222222, -1.111111, -0.0),
            Vec3::new(-1.111111, -14.444443, 9.999999),
            Vec3::new(-0.0, 9.999999, -4.9999995),
        );

        mat.inverse();
        assert_eq!(mat, mat_inverse);
    }

    #[test]
    fn mat3_is_invertible() {
        let mat_zero = Mat3::zero();
        assert_eq!(mat_zero.is_invertible(), false);
        let mat_identity = Mat3::identity();
        assert_eq!(mat_identity.is_invertible(), true);
    }

    #[test]
    fn mat3_is_transpose() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat3 {
            cols: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        };

        let mat_transpose = Mat3::new(
            Vec3::new(1.0, 4.0, 7.0),
            Vec3::new(2.0, 5.0, 8.0),
            Vec3::new(3.0, 6.0, 9.0),
        );
        mat.transpose();
        assert_eq!(mat, mat_transpose);
    }

    #[test]
    fn mat3_translate() {
        let mut mat = Mat3::identity();
        let mat_translate = Mat3::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(2.0, 3.0, 1.0),
        );
        mat.translate(Vec2::new(2.0, 3.0));
        assert_eq!(mat, mat_translate);
    }

    #[test]
    fn mat3_rotate() {
        let mut mat = Mat3::identity();
        let mat_rotate = Mat3 {
            cols: [
                0.96377087,
                0.26673144,
                0.0,
                -0.26673144,
                0.96377087,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        };
        mat.rotate(0.27);
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat3_scale() {
        let mut mat = Mat3::identity();
        let mat_scale = Mat3::new(
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        mat.scale(Vec2::new(2.0, 3.0));
        assert_eq!(mat, mat_scale);
    }

    #[test]
    fn mat3_mul() {
        let mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_right = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_result = Mat3 {
            cols: [38.0, 44.0, 50.0, 98.0, 116.0, 134.0, 158.0, 188.0, 218.0],
        };
        assert_eq!(mat_left * mat_right, mat_result);

        let mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let vec_right = Vec3::new(1.0, 2.0, 3.0);
        let vec_result = Vec3::new(38.0, 44.0, 50.0);
        assert_eq!(mat_left * vec_right, vec_result);

        let mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_result = Mat3::new(
            Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(10.0, 12.0, 14.0),
            Vec3::new(18.0, 20.0, 22.0),
        );
        assert_eq!(mat_left * 2f32, mat_result);
        assert_eq!(2f32 * mat_left, mat_result);

        let mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_result = Mat3::new(
            Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(10.0, 12.0, 14.0),
            Vec3::new(18.0, 20.0, 22.0),
        );
        assert_eq!(mat_left * 2f64, mat_result);
        assert_eq!(2f64 * mat_left, mat_result);

        let mut mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_right = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_result = Mat3::new(
            Vec3::new(38.0, 44.0, 50.0),
            Vec3::new(98.0, 116.0, 134.0),
            Vec3::new(158.0, 188.0, 218.0),
        );
        mat_left *= mat_right;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_result = Mat3::new(
            Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(10.0, 12.0, 14.0),
            Vec3::new(18.0, 20.0, 22.0),
        );
        mat_left *= 2f32;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(9.0, 10.0, 11.0),
        );
        let mat_result = Mat3::new(
            Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(10.0, 12.0, 14.0),
            Vec3::new(18.0, 20.0, 22.0),
        );
        mat_left *= 2f64;
        assert_eq!(mat_left, mat_result);
    }
}
