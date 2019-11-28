extern crate common;
extern crate context;
extern crate graphic;
extern crate math;

use context::canvas::Canvas;
use context::input::Input;
use context::keyboard::Key;

fn main() {
    math::health();
    common::health();
    graphic::health();
    context::health();

    let mut input = Input::new();
    let mut canvas = Canvas::new("fsdf", 400, 400).unwrap();

    while !canvas.should_close() {
        canvas.on_update_begin();
        canvas.process_events(&mut input);

        if input.is_key_pressed(Key::W) {
            println!("==> W is pressed");
        }

        if input.is_key_released(Key::W) {
            println!("==> W is released");
        }

        if input.is_key_hold(Key::S) {
            println!("==> S is hold");
        }

        input.on_update_end();
        canvas.on_update_end();
    }
}
