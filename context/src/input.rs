#![allow(dead_code)]

use super::keyboard;
use super::keyboard::Action;
use super::keyboard::Button;
use super::keyboard::Key;

pub fn update(input: &mut Input, key: Key, button: Button) {
    let current = input.current[key as usize].clone();
    input.before[key as usize] = current;
    input.current[key as usize] = button;
}

pub fn clean_up(input: &mut Input) {
    input.current
        .iter_mut()
        .zip(input.before.iter_mut())
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
