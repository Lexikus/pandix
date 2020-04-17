extern crate math;

use math::Mat4;
use math::Quat;
use math::Vec3;
use math::Vec4;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat4_new() {
        let x = Vec4::one();
        let y = Vec4::one();
        let z = Vec4::one();
        let w = Vec4::one();
        let mat = Mat4 {
            cols: [
                x.x, x.y, x.z, x.w, y.x, y.y, y.z, y.w, z.x, z.y, z.z, z.w, w.x, w.y, w.z, w.w,
            ],
        };
        assert_eq!(Mat4::new(x, y, z, w), mat);
    }

    #[test]
    fn mat4_identity() {
        let mat = Mat4 {
            cols: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        };
        assert_eq!(Mat4::identity(), mat);
    }

    #[test]
    fn mat4_zero() {
        let mat = Mat4 {
            cols: [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        };
        assert_eq!(Mat4::zero(), mat);
    }

    #[test]
    fn mat4_perspective() {
        let mat = Mat4 {
            cols: [
                -0.074007675,
                0.0,
                0.0,
                0.0,
                0.0,
                -0.9398974,
                0.0,
                0.0,
                0.0,
                0.0,
                -1.1799539,
                -1.0,
                0.0,
                0.0,
                -17.657627,
                0.0,
            ],
        };
        assert_eq!(Mat4::perspective(23.5, 12.7, 8.1, 98.123), mat);
    }

    #[test]
    fn mat4_ortho() {
        let mat = Mat4 {
            cols: [
                2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, -3.0, -7.0, -11.0, 1.0,
            ],
        };
        assert_eq!(Mat4::ortho(1.0, 2.0, 3.0, 4.0, 5.0, 6.0), mat);
    }

    #[test]
    fn mat4_frustum() {
        let mat = Mat4 {
            cols: [
                10.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 3.0, 7.0, -11.0, -1.0, 0.0, 0.0, -60.0,
                0.0,
            ],
        };
        assert_eq!(Mat4::frustum(1.0, 2.0, 3.0, 4.0, 5.0, 6.0), mat);
    }

    #[test]
    fn mat4_look_at() {
        let mat = Mat4 {
            cols: [
                -0.70710677,
                -0.40824834,
                -0.57735026,
                0.0,
                0.0,
                0.8164967,
                -0.57735026,
                0.0,
                0.70710677,
                -0.40824834,
                -0.57735026,
                0.0,
                -0.0,
                -0.0,
                0.0,
                1.0,
            ],
        };
        assert_eq!(Mat4::look_at(Vec3::zero(), Vec3::one(), Vec3::up()), mat);

        let mat = Mat4 {
            cols: [
                0.10283788,
                0.09277505,
                -0.9903621,
                0.0,
                -0.0,
                0.99564093,
                0.09326954,
                0.0,
                0.9946981,
                -0.009591643,
                0.102389604,
                0.0,
                -785.2488,
                -69.58577,
                -83.81696,
                1.0,
            ],
        };
        assert_eq!(
            Mat4::look_at(
                Vec3::new(4.2, 77.1, 789.0),
                Vec3::new(320.2, 47.34, 756.33),
                Vec3::up()
            ),
            mat
        );
    }

    #[test]
    fn mat4_trs() {
        let mat = Mat4 {
            cols: [
                -200.0, 224.0, -80.0, 0.0, -180.0, -171.0, 180.0, 0.0, 220.0, 40.0, -90.0, 0.0,
                5.0, 6.0, 7.0, 1.0,
            ],
        };
        assert_eq!(
            Mat4::trs(
                Vec3::new(5.0, 6.0, 7.0),
                Quat::new(1.0, 2.0, 3.0, 4.0),
                Vec3::new(8.0, 9.0, 10.0)
            ),
            mat
        );
    }

    #[test]
    fn mat4_tr() {
        let mat = Mat4 {
            cols: [
                -25.0, 28.0, -10.0, 0.0, -20.0, -19.0, 20.0, 0.0, 22.0, 4.0, -9.0, 0.0, 5.0, 6.0,
                7.0, 1.0,
            ],
        };
        assert_eq!(
            Mat4::tr(Vec3::new(5.0, 6.0, 7.0), Quat::new(1.0, 2.0, 3.0, 4.0)),
            mat
        );
    }

    #[test]
    fn mat4_from_translate() {
        let pos = Vec3::new(3.0, 4.0, 21.0);
        let mat = Mat4 {
            cols: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, pos.x, pos.y, pos.z,
                1.0,
            ],
        };

        assert_eq!(Mat4::from_translate(pos), mat);
    }

    #[test]
    fn mat4_from_scale() {
        let scale = Vec3::new(3.0, 4.0, 21.0);
        let mat = Mat4 {
            cols: [
                scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ],
        };

        assert_eq!(Mat4::from_scale(scale), mat);
    }

    #[test]
    fn mat4_from_rotation() {
        let quat = Quat::new(3.0, 4.0, 21.0, 55.0);
        let mat = Mat4 {
            cols: [
                -913.0, 2334.0, -314.0, 0.0, -2286.0, -899.0, 498.0, 0.0, 566.0, -162.0, -49.0,
                0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        };

        assert_eq!(Mat4::from_rotation(quat), mat);
    }

    #[test]
    fn mat4_from_rotation_unnormalized_axis() {
        let axis = Vec3::new(0.0, 5.0, 0.0);
        let rad = 0.63;
        let mat = Mat4 {
            cols: [
                0.8080275,
                0.0,
                -0.58914477,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.58914477,
                0.0,
                0.8080275,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        };

        assert_eq!(Mat4::from_rotation_unnormalized_axis(rad, axis), mat);
    }

    #[test]
    fn mat4_from_rotation_axis() {
        let axis = Vec3::up();
        let rad = 0.63;
        let mat = Mat4 {
            cols: [
                0.8080275,
                0.0,
                -0.58914477,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.58914477,
                0.0,
                0.8080275,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        };

        assert_eq!(Mat4::from_rotation_axis(rad, axis), mat);
    }

    #[test]
    fn mat4_from_rotation_x() {
        let rad = 0.63;
        let mat = Mat4 {
            cols: [
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.8080275,
                0.58914477,
                0.0,
                0.0,
                -0.58914477,
                0.8080275,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        };

        assert_eq!(Mat4::from_rotation_x(rad), mat);
    }

    #[test]
    fn mat4_from_rotation_y() {
        let rad = 0.43;
        let mat = Mat4 {
            cols: [
                0.90896577, 0.0, -0.4168708, 0.0, 0.0, 1.0, 0.0, 0.0, 0.4168708, 0.0, 0.90896577,
                0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        };

        assert_eq!(Mat4::from_rotation_y(rad), mat);
    }

    #[test]
    fn mat4_from_rotation_z() {
        let rad = 0.33;
        let mat = Mat4 {
            cols: [
                0.94604236,
                0.32404304,
                0.0,
                0.0,
                -0.32404304,
                0.94604236,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        };

        assert_eq!(Mat4::from_rotation_z(rad), mat);
    }

    #[test]
    fn mat4_determinant() {
        let determinant = 1625.0;
        let mat = Mat4::tr(Vec3::up(), Quat::new(1.0, 2.0, 3.0, 4.0));

        assert_eq!(mat.determinant(), determinant);

        let determinant = 1.0;
        let mat = Mat4::identity();

        assert_eq!(mat.determinant(), determinant);

        let determinant = 0.0;
        let mat = Mat4::zero();

        assert_eq!(mat.determinant(), determinant);
    }

    #[test]
    fn mat4_inverse() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat4::tr(Vec3::up(), Quat::new(1.0, 2.0, 3.0, 4.0));

        let mat_inverse = Mat4::new(
            Vec4::new(0.055999998, 0.13046153, 0.2276923, 0.0),
            Vec4::new(0.16, 0.27384615, 0.43076923, 0.0),
            Vec4::new(0.20799999, 0.4406154, 0.6369231, 0.0),
            Vec4::new(-0.16, -0.27384615, -0.43076923, 1.0),
        );

        mat.inverse();
        assert_eq!(mat, mat_inverse);
    }

    #[test]
    fn mat4_is_invertible() {
        let mat_zero = Mat4::zero();
        assert_eq!(mat_zero.is_invertible(), false);
        let mat_identity = Mat4::identity();
        assert_eq!(mat_identity.is_invertible(), true);
    }

    #[test]
    fn mat4_is_transpose() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat4::tr(Vec3::up(), Quat::new(1.0, 2.0, 3.0, 4.0));

        let mat_transpose = Mat4::new(
            Vec4::new(-25.0, -20.0, 22.0, 0.0),
            Vec4::new(28.0, -19.0, 4.0, 1.0),
            Vec4::new(-10.0, 20.0, -9.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.transpose();
        assert_eq!(mat, mat_transpose);
    }

    #[test]
    fn mat4_translate() {
        let mut mat = Mat4::identity();
        let mat_translate = Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(2.0, 3.0, 7.0, 1.0),
        );
        mat.translate(Vec3::new(2.0, 3.0, 7.0));
        assert_eq!(mat, mat_translate);
    }

    #[test]
    fn mat4_rotate() {
        let mut mat = Mat4::identity();
        let mat_rotate = Mat4::new(
            Vec4::new(0.96377087, 0.0, -0.26673144, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.26673144, 0.0, 0.96377087, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.rotate_axis(0.27, Vec3::up());
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat4_rotate_non_normalized() {
        let mut mat = Mat4::identity();
        let mat_rotate = Mat4::new(
            Vec4::new(0.96377087, 0.0, -0.26673144, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.26673144, 0.0, 0.96377087, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.rotate_unnormalized_axis(0.27, Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat4_rotate_x() {
        let mut mat = Mat4::identity();
        let mat_rotate = Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.96377087, 0.26673144, 0.0),
            Vec4::new(0.0, -0.26673144, 0.96377087, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.rotate_x(0.27);
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat4_rotate_y() {
        let mut mat = Mat4::identity();
        let mat_rotate = Mat4::new(
            Vec4::new(0.96377087, 0.0, -0.26673144, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.26673144, 0.0, 0.96377087, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.rotate_y(0.27);
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat4_rotate_z() {
        let mut mat = Mat4::identity();
        let mat_rotate = Mat4::new(
            Vec4::new(0.96377087, 0.26673144, 0.0, 0.0),
            Vec4::new(-0.26673144, 0.96377087, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.rotate_z(0.27);
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat4_scale() {
        let mut mat = Mat4::identity();
        let mat_scale = Mat4::new(
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 7.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        mat.scale(Vec3::new(2.0, 3.0, 7.0));
        assert_eq!(mat, mat_scale);
    }

    #[test]
    fn mat4_mul() {
        let mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_right = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_result = Mat4::new(
            Vec4::new(90.0, 100.0, 110.0, 120.0),
            Vec4::new(202.0, 228.0, 254.0, 280.0),
            Vec4::new(314.0, 356.0, 398.0, 440.0),
            Vec4::new(426.0, 484.0, 542.0, 600.0),
        );
        assert_eq!(mat_left * mat_right, mat_result);

        let mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let vec_right = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let vec_result = Vec4::new(90.0, 100.0, 110.0, 120.0);
        assert_eq!(mat_left * vec_right, vec_result);

        let mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_result = Mat4::new(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );
        assert_eq!(mat_left * 2f32, mat_result);
        assert_eq!(2f32 * mat_left, mat_result);

        let mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_result = Mat4::new(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );
        assert_eq!(mat_left * 2f64, mat_result);
        assert_eq!(2f64 * mat_left, mat_result);

        let mut mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_right = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_result = Mat4::new(
            Vec4::new(90.0, 100.0, 110.0, 120.0),
            Vec4::new(202.0, 228.0, 254.0, 280.0),
            Vec4::new(314.0, 356.0, 398.0, 440.0),
            Vec4::new(426.0, 484.0, 542.0, 600.0),
        );
        mat_left *= mat_right;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_result = Mat4::new(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );
        mat_left *= 2f32;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let mat_result = Mat4::new(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );
        mat_left *= 2f64;
        assert_eq!(mat_left, mat_result);
    }
}
