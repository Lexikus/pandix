extern crate core;
extern crate context;
pub extern crate ecs;
pub extern crate math;

pub use core::resource;
pub use core::Engine;

pub mod keyboard {
    pub use super::context::keyboard::Action;
    pub use super::context::keyboard::Key;
    pub use super::context::keyboard::Modifier;
}
