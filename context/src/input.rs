#![allow(dead_code)]

use super::keyboard;
use super::keyboard::Action;
use super::keyboard::Button;
use super::keyboard::Key;

// TODO(alex):
// we should save the original key state aswell.
// this will be relevant when we actually want to have the correct behaviour.
// For instance, we want to write into a input field.
pub struct Input {
    current: [Button; keyboard::KEYS_COUNT],
    before: [Button; keyboard::KEYS_COUNT],
}

impl Input {
    pub fn new() -> Self {
        Input {
            current: keyboard::keys_collection(),
            before: keyboard::keys_collection(),
        }
    }

    pub fn update(&mut self, key: Key, button: Button) {
        let current = self.current[key as usize].clone();
        self.before[key as usize] = current;
        self.current[key as usize] = button;
    }

    pub fn on_update_end(&mut self) {
        self.current
            .iter_mut()
            .zip(self.before.iter_mut())
            .filter(|(current, _)| {
                *current.action() == Action::Press || *current.action() == Action::Release
            })
            .for_each(|(current, before)| {
                if let Action::Press = *current.action() {
                    current.set_action(Action::Repeat);
                    before.set_action(Action::Press);
                }

                if let Action::Release = *current.action() {
                    before.set_action(Action::Release);
                }
            });
    }

    pub fn is_key_hold(&self, key: Key) -> bool {
        *self.current[key as usize].action() == Action::Press
            || *self.current[key as usize].action() == Action::Repeat
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let current = *self.current[key as usize].action() == Action::Press;
        let before = *self.before[key as usize].action() == Action::Release;

        current && before
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        let current = *self.current[key as usize].action() == Action::Release;
        let before = *self.before[key as usize].action() == Action::Press
            || *self.before[key as usize].action() == Action::Repeat;

        current && before
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
