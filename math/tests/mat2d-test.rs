extern crate math;

use math::Mat2d;
use math::Vec2;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat2d_new() {
        let x = Vec2::one();
        let y = Vec2::one();
        let z = Vec2::one();
        let mat = Mat2d {
            cols: [x.x, x.y, y.x, y.y, z.x, z.y],
        };
        assert_eq!(Mat2d::new(x, y, z), mat);
    }

    #[test]
    fn mat2d_identity() {
        let mat = Mat2d {
            cols: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        };
        assert_eq!(Mat2d::identity(), mat);
    }

    #[test]
    fn mat2d_zero() {
        let mat = Mat2d {
            cols: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        };
        assert_eq!(Mat2d::zero(), mat);
    }

    #[test]
    fn mat2d_trs() {
        let mut mat_result = Mat2d::identity();
        mat_result.translate(Vec2::new(5.0, 6.0));
        mat_result.rotate(0.2);
        mat_result.scale(Vec2::new(8.0, 9.0));

        assert_eq!(
            Mat2d::trs(Vec2::new(5.0, 6.0), 0.2, Vec2::new(8.0, 9.0)),
            mat_result
        );
    }

    #[test]
    fn mat2d_tr() {
        let mut mat_result = Mat2d::identity();
        mat_result.translate(Vec2::new(5.0, 6.0));
        mat_result.rotate(0.2);

        assert_eq!(Mat2d::tr(Vec2::new(5.0, 6.0), 0.2), mat_result);
    }

    #[test]
    fn mat2d_from_translate() {
        let pos = Vec2::new(3.0, 4.0);
        let mat = Mat2d {
            cols: [1.0, 0.0, 0.0, 1.0, pos.x, pos.y],
        };

        assert_eq!(Mat2d::from_translate(pos), mat);
    }

    #[test]
    fn mat2d_from_scale() {
        let scale = Vec2::new(3.0, 4.0);
        let mat = Mat2d {
            cols: [scale.x, 0.0, 0.0, scale.y, 0.0, 0.0],
        };

        assert_eq!(Mat2d::from_scale(scale), mat);
    }

    #[test]
    fn mat2d_from_rotation() {
        let quat = 0.15;
        let mat = Mat2d {
            cols: [0.9887711, 0.14943814, -0.14943814, 0.9887711, 0.0, 0.0],
        };

        assert_eq!(Mat2d::from_rotation(quat), mat);
    }

    #[test]
    fn mat2d_determinant() {
        let determinant = -2.0;
        let mat = Mat2d {
            cols: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };

        assert_eq!(mat.determinant(), determinant);

        let determinant = 1.0;
        let mat = Mat2d::identity();

        assert_eq!(mat.determinant(), determinant);

        let determinant = 0.0;
        let mat = Mat2d::zero();

        assert_eq!(mat.determinant(), determinant);
    }

    #[test]
    fn mat2d_inverse() {
        // if the determinant of the matrix is zero, you get a zero matrix back.
        let mut mat = Mat2d {
            cols: [-2.0, 1.0, 1.5, -0.5, 1.0, -2.0],
        };

        let mat_inverse = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(3.0, 4.0),
            Vec2::new(5.0, 6.0),
        );

        mat.inverse();
        assert_eq!(mat, mat_inverse);
    }

    #[test]
    fn mat2d_is_invertible() {
        let mat_zero = Mat2d::zero();
        assert_eq!(mat_zero.is_invertible(), false);
        let mat_identity = Mat2d::identity();
        assert_eq!(mat_identity.is_invertible(), true);
    }

    #[test]
    fn mat2d_translate() {
        let mut mat = Mat2d::identity();
        let mat_translate = Mat2d::new(
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(2.0, 3.0),
        );
        mat.translate(Vec2::new(2.0, 3.0));
        assert_eq!(mat, mat_translate);
    }

    #[test]
    fn mat2d_rotate() {
        let mut mat = Mat2d::identity();
        let mat_rotate = Mat2d {
            cols: [0.96377087, 0.26673144, -0.26673144, 0.96377087, 0.0, 0.0],
        };
        mat.rotate(0.27);
        assert_eq!(mat, mat_rotate);
    }

    #[test]
    fn mat2d_scale() {
        let mut mat = Mat2d::identity();
        let mat_scale = Mat2d::new(
            Vec2::new(2.0, 0.0),
            Vec2::new(0.0, 3.0),
            Vec2::new(0.0, 0.0),
        );
        mat.scale(Vec2::new(2.0, 3.0));
        assert_eq!(mat, mat_scale);
    }

    #[test]
    fn mat2d_mul() {
        let mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_right = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_result = Mat2d {
            cols: [11.0, 14.0, 35.0, 46.0, 68.0, 88.0],
        };
        assert_eq!(mat_left * mat_right, mat_result);

        let mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let vec_right = Vec2::new(1.0, 2.0);
        let vec_result = Vec2::new(20.0, 24.0);
        assert_eq!(mat_left * vec_right, vec_result);

        let mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_result = Mat2d::new(
            Vec2::new(2.0, 4.0),
            Vec2::new(10.0, 12.0),
            Vec2::new(18.0, 20.0),
        );
        assert_eq!(mat_left * 2f32, mat_result);
        assert_eq!(2f32 * mat_left, mat_result);

        let mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_result = Mat2d::new(
            Vec2::new(2.0, 4.0),
            Vec2::new(10.0, 12.0),
            Vec2::new(18.0, 20.0),
        );
        assert_eq!(mat_left * 2f64, mat_result);
        assert_eq!(2f64 * mat_left, mat_result);

        let mut mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_right = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_result = Mat2d::new(
            Vec2::new(11.0, 14.0),
            Vec2::new(35.0, 46.0),
            Vec2::new(68.0, 88.0),
        );
        mat_left *= mat_right;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_result = Mat2d::new(
            Vec2::new(2.0, 4.0),
            Vec2::new(10.0, 12.0),
            Vec2::new(18.0, 20.0),
        );
        mat_left *= 2f32;
        assert_eq!(mat_left, mat_result);

        let mut mat_left = Mat2d::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(9.0, 10.0),
        );
        let mat_result = Mat2d::new(
            Vec2::new(2.0, 4.0),
            Vec2::new(10.0, 12.0),
            Vec2::new(18.0, 20.0),
        );
        mat_left *= 2f64;
        assert_eq!(mat_left, mat_result);
    }
}
