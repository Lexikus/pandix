#![allow(dead_code)]

use std::collections::HashMap;

use super::keyboard::Action;
use super::keyboard::Button;
use super::keyboard::Key;

const KEYS_ON_KEYBOARD: usize = 121;

pub struct Input {
    current: HashMap<Key, Button>,
    before: HashMap<Key, Button>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            current: HashMap::with_capacity(KEYS_ON_KEYBOARD),
            before: HashMap::with_capacity(KEYS_ON_KEYBOARD),
        }
    }

    pub fn update(&mut self, key: Key, button: Button) {
        let current = self.current.entry(key).or_default();
        *self.before.entry(key).or_default() = current.clone();
        *current = button;
    }

    pub fn on_update_end(&mut self) {
        self.current
            .iter_mut()
            .zip(&mut self.before)
            .map(|((_, current), (_, before))| (current, before))
            .filter(|(current, _)| {
                *current.action() == Action::Press || *current.action() == Action::Release
            })
            .for_each(|(current, before)| {
                match *current.action() {
                    Action::Press => {
                        current.set_action(Action::Repeat);
                        before.set_action(Action::Press);
                    }
                    Action::Release => {
                        before.set_action(Action::Release);
                    }
                    _ => (),
                };
            });
    }

    pub fn is_key_hold(&self, key: Key) -> bool {
        let &action = self
            .current
            .get(&key)
            .unwrap_or(&Button::default())
            .action();

        action == Action::Press || action == Action::Repeat
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let &current_action = self
            .current
            .get(&key)
            .unwrap_or(&Button::default())
            .action();

        let &before_action = self.before.get(&key).unwrap_or(&Button::default()).action();

        current_action == Action::Press && before_action == Action::Release
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        let &current_action = self
            .current
            .get(&key)
            .unwrap_or(&Button::default())
            .action();

        let &before_action = self.before.get(&key).unwrap_or(&Button::default()).action();

        current_action == Action::Release
            && (before_action == Action::Press || before_action == Action::Repeat)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
