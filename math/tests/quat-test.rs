extern crate math;
use math::Quat;
use math::Vec3;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quat_new() {
        let new = Quat {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        assert_eq!(Quat::new(1.0, 2.0, 3.0, 4.0), new);
    }

    #[test]
    fn quat_normalize() {
        let normalized = Quat {
            x: 0.18257418,
            y: 0.36514837,
            z: 0.5477226,
            w: 0.73029673,
        };
        assert_eq!(Quat::normalize(1.0, 2.0, 3.0, 4.0), normalized);
    }

    #[test]
    fn quat_identity() {
        let identity = Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };
        assert_eq!(Quat::identity(), identity);
    }

    #[test]
    fn quat_angle() {
        let angle = 2.5948067;
        assert_eq!(
            Quat::angle(Quat::new(0.1, 0.2, 0.3, 0.4), Quat::new(0.2, 0.3, 0.5, 0.1)),
            angle
        );
    }

    #[test]
    fn quat_dot() {
        let dot = 3427.0;
        assert_eq!(
            Quat::dot(
                Quat::new(3.0, 5.0, 21.0, 34.0),
                Quat::new(6.0, 7.0, 2.0, 98.0)
            ),
            dot
        );

        let dot = 4118.0;
        assert_eq!(
            Quat::dot(
                Quat::new(-3.0, 5.0, 3.0, 87.0),
                Quat::new(6.0, 7.0, 91.0, 44.0)
            ),
            dot
        );
    }

    #[test]
    fn quat_euler() {
        let quat = Quat::new(0.0, 0.008726535, 0.0, 0.9999619);
        assert_eq!(Quat::euler(Vec3::up()), quat);
    }

    #[test]
    fn quat_inverse() {
        let quat = Quat::new(-0.7751938, -0.15503876, -0.23255815, 0.31007752);
        assert_eq!(Quat::inverse(Quat::new(1.0, 0.2, 0.3, 0.4)), quat);
    }

    #[test]
    fn quat_lerp() {
        let lerp_quat = Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(
            Quat::lerp(
                Quat::new(0.0, 0.0, 0.0, 0.0),
                Quat::new(1.0, 1.0, 1.0, 1.0),
                0.0
            ),
            lerp_quat
        );

        let lerp_quat = Quat {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert_eq!(
            Quat::lerp(
                Quat::new(0.0, 0.0, 0.0, 0.0),
                Quat::new(1.0, 1.0, 1.0, 1.0),
                1.0
            ),
            lerp_quat
        );
    }

    #[test]
    fn quat_normalized() {
        let quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        let new_quat = quat.normalized();
        let normalized_quat = Quat {
            x: 0.18257418,
            y: 0.36514837,
            z: 0.5477226,
            w: 0.73029673,
        };
        assert_eq!(new_quat, normalized_quat);
    }

    #[test]
    fn quat_mul() {
        let identity = Quat::identity();
        let one = Quat::new(0.1, 0.2, 0.3, 0.4);
        let result = Quat::new(0.1, 0.2, 0.3, 0.4);
        assert_eq!(identity * one, result);

        let quat = Quat::new(0.1, 0.2, 0.3, 0.4);
        let vec = Vec3::one();
        let result = Vec3::new(0.76000005, 1.12, 1.0);
        assert_eq!(quat * vec, result);

        let quat = Quat::new(0.1, 0.2, 0.3, 0.4);
        let vec = Vec3::one();
        let result = Vec3::new(0.76000005, 1.12, 1.0);
        assert_eq!(vec * quat, result);
    }
}
