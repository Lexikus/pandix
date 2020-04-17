extern crate math;

use math::Mat2;
use math::Vec2;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat2_new() {
        let x = Vec2::one();
        let y = Vec2::one();
        let mat = Mat2 {
            cols: [x.x, x.y, y.x, y.y],
        };
        assert_eq!(Mat2::new(x, y), mat);
    }

    #[test]
    fn mat2_identity() {
        let mat = Mat2 {
            cols: [1.0, 0.0, 0.0, 1.0],
        };
        assert_eq!(Mat2::identity(), mat);
    }

    #[test]
    fn mat2_zero() {
        let mat = Mat2 {
            cols: [0.0, 0.0, 0.0, 0.0],
        };
        assert_eq!(Mat2::zero(), mat);
    }

    #[test]
    fn mat2_from_scale() {
        let scale = Vec2::new(3.0, 4.0);
        let mat = Mat2 {
            cols: [scale.x, 0.0, 0.0, scale.y],
        };

        assert_eq!(Mat2::from_scale(scale), mat);
    }

    #[test]
    fn mat2_from_rotation() {
        let quat = 0.15;
        let mat = Mat2 {
            cols: [0.9887711, 0.14943814, -0.14943814, 0.9887711],
        };

        assert_eq!(Mat2::from_rotation(quat), mat);
    }

    #[test]
    fn mat2_determinant() {
        let determinant = -2.0;
        let mat = Mat2 {
            cols: [1.0, 2.0, 3.0, 4.0],
        };

        assert_eq!(mat.determinant(), determinant);

        let determinant = 1.0;
        let mat = Mat2::identity();

        assert_eq!(mat.determinant(), determinant);

        let determinant = 0.0;
        let mat = Mat2::zero();

        assert_eq!(mat.determinant(), determinant);
    }

    #[test]
    fn mat2_inverse() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat2 {
            cols: [0.5, 0.1, 0.1, 0.2],
        };

        let mat_inverse = Mat2::new(
            Vec2::new(2.222222, -1.111111),
            Vec2::new(-1.111111, 5.5555553),
        );

        mat.inverse();
        assert_eq!(mat, mat_inverse);
    }

    #[test]
    fn mat2_is_invertible() {
        let mat_zero = Mat2::zero();
        assert_eq!(mat_zero.is_invertible(), false);
        let mat_identity = Mat2::identity();
        assert_eq!(mat_identity.is_invertible(), true);
    }

    #[test]
    fn mat2_is_transpose() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat2 {
            cols: [1.0, 2.0, 4.0, 5.0],
        };

        let mat_transpose = Mat2::new(Vec2::new(1.0, 4.0), Vec2::new(2.0, 5.0));
        mat.transpose();
        assert_eq!(mat, mat_transpose);
    }

    #[test]
    fn mat2_rotate() {
        let mut mat = Mat2::identity();
        let mat_rotate = Mat2 {
            cols: [0.96377087, 0.26673144, -0.26673144, 0.96377087],
        };
        mat.rotate(0.27);
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat2_scale() {
        let mut mat = Mat2::identity();
        let mat_scale = Mat2::new(Vec2::new(2.0, 0.0), Vec2::new(0.0, 3.0));
        mat.scale(Vec2::new(2.0, 3.0));
        assert_eq!(mat, mat_scale);
    }

    #[test]
    fn mat2_mul() {
        let mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_right = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_result = Mat2 {
            cols: [11.0, 14.0, 35.0, 46.0],
        };
        assert_eq!(mat_left * mat_right, mat_result);

        let mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let vec_right = Vec2::new(1.0, 2.0);
        let vec_result = Vec2::new(11.0, 14.0);
        assert_eq!(mat_left * vec_right, vec_result);

        let mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_result = Mat2::new(Vec2::new(2.0, 4.0), Vec2::new(10.0, 12.0));
        assert_eq!(mat_left * 2f32, mat_result);
        assert_eq!(2f32 * mat_left, mat_result);

        let mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_result = Mat2::new(Vec2::new(2.0, 4.0), Vec2::new(10.0, 12.0));
        assert_eq!(mat_left * 2f64, mat_result);
        assert_eq!(2f64 * mat_left, mat_result);

        let mut mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_right = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_result = Mat2::new(Vec2::new(11.0, 14.0), Vec2::new(35.0, 46.0));
        mat_left *= mat_right;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_result = Mat2::new(Vec2::new(2.0, 4.0), Vec2::new(10.0, 12.0));
        mat_left *= 2f32;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(5.0, 6.0));
        let mat_result = Mat2::new(Vec2::new(2.0, 4.0), Vec2::new(10.0, 12.0));
        mat_left *= 2f64;
        assert_eq!(mat_left, mat_result);
    }
}
