extern crate glutin;

pub mod canvas;
pub mod input;
pub mod keyboard;

pub use glutin::event::Event;

pub fn health() {
    println!("context reachable");
}
