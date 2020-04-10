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
        let w = Vec4::one();
        let z = Vec4::one();
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
}
