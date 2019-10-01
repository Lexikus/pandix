#![allow(dead_code)]

use std::collections::HashMap;

use super::keyboard::Button;
use super::keyboard::Action;
use super::keyboard::Key;

// TODO: is_modifier_bla functions
// use super::keyboard::Modifier;

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

    // TODO: reset the state of pressed keys. This is needed because we want to have a key Press state only for one frame
    pub fn on_update_end(&mut self) {
        self.current
            .iter()
            .zip(&self.before)
            .filter(|input| (input.0).1.action == Action::Press)
            .for_each(|input| println!("{:?}, {:?}", input.0, input.1));
    }

    pub fn is_key_hold(&self, key: Key) -> bool {
        self.current
            .get(&key)
            .unwrap_or(&Button::new())
            .action == Action::Press
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let current_action = self.current
            .get(&key)
            .unwrap_or(&Button::new())
            .action;

        let before_action = self.before
            .get(&key)
            .unwrap_or(&Button::new())
            .action;

        current_action == Action::Press &&
        before_action == Action::Release
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        let current_action = self.current
            .get(&key)
            .unwrap_or(&Button::new())
            .action;

        let before_action = self.before
            .get(&key)
            .unwrap_or(&Button::new())
            .action;

        current_action == Action::Release &&
        (before_action == Action::Press ||
        before_action == Action::Repeat)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}