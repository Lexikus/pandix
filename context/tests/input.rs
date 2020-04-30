extern crate context;

use context::input;
use context::input::Input;
use context::keyboard::Action;
use context::keyboard::Button;
use context::keyboard::Key;
use context::keyboard::Modifier;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_is_key_hold() {
        let mut input = Input::new();

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(input.is_key_hold(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Repeat, Modifier::Unknown),
        );

        assert!(input.is_key_hold(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(!input.is_key_hold(Key::A));
    }

    #[test]
    fn input_is_key_pressed() {
        let mut input = Input::new();

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(input.is_key_pressed(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Repeat, Modifier::Unknown),
        );

        assert!(!input.is_key_pressed(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(!input.is_key_pressed(Key::A));
    }

    #[test]
    fn input_is_key_released() {
        let mut input = Input::new();

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(!input.is_key_released(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Repeat, Modifier::Unknown),
        );

        assert!(!input.is_key_released(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(input.is_key_released(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        assert!(!input.is_key_released(Key::A));

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        assert!(input.is_key_released(Key::A));
    }

    #[test]
    fn input_is_key_pressed_only_one_frame() {
        let mut input = Input::new();

        // Key should not be pressed
        assert!(!input.is_key_pressed(Key::A));

        // one frame later
        input::clean_up(&mut input);

        // context updates input system
        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        // key should be pressed in this frame
        assert!(input.is_key_pressed(Key::A));

        // one frame later
        input::clean_up(&mut input);

        // key should be not pressed anymore. its state should be repeat
        assert!(!input.is_key_pressed(Key::A));
    }

    #[test]
    fn input_is_key_released_only_one_frame() {
        let mut input = Input::new();

        // Key should not be released
        assert!(!input.is_key_released(Key::A));

        // one frame later
        input::clean_up(&mut input);

        // context updates input system
        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Press, Modifier::Unknown),
        );

        // Key should still not be released
        assert!(!input.is_key_released(Key::A));

        input::clean_up(&mut input);

        // Key should still not be released
        assert!(!input.is_key_released(Key::A));

        input::clean_up(&mut input);

        input::update(
            &mut input,
            Key::A,
            Button::new(Key::A, Action::Release, Modifier::Unknown),
        );

        // Key should be released
        assert!(input.is_key_released(Key::A));

        input::clean_up(&mut input);

        // Key should not be released
        assert!(!input.is_key_released(Key::A));
    }
}
