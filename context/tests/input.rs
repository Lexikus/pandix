extern crate context;

use context::input::Input;
use context::keyboard::Action;
use context::keyboard::Button;
use context::keyboard::Key;
use context::keyboard::Modifier;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_key_hold() {
        let mut input = Input::new();

        input.update(
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(input.is_key_hold(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Repeat, Modifier::Unknown),
        );

        assert!(input.is_key_hold(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(!input.is_key_hold(Key::A));
    }

    #[test]
    fn is_key_pressed() {
        let mut input = Input::new();

        input.update(
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(input.is_key_pressed(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Repeat, Modifier::Unknown),
        );

        assert!(!input.is_key_pressed(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(!input.is_key_pressed(Key::A));
    }

    #[test]
    fn is_key_released() {
        let mut input = Input::new();

        input.update(
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(!input.is_key_released(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Repeat, Modifier::Unknown),
        );

        assert!(!input.is_key_released(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(input.is_key_released(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(!input.is_key_released(Key::A));

        input.update(
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(input.is_key_released(Key::A));
    }
}
