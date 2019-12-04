#![allow(dead_code)]

extern crate common;
extern crate context;
extern crate graphic;
extern crate math;

use context::canvas::Canvas;
use context::input::Input;
use context::keyboard::Key;

// TODO(alex): abstract the setup in this struct. Basically, we need to give the window a name.
//             Also, we have to specify the size of the window or is it fullscreen etc.
//             all the crates settings needs to be modifed via the engine before it can be run.
pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub fn run(&self) {
        math::health();
        common::health();
        graphic::health();
        context::health();

        let v: math::Vec2 = math::Vec3::new(-5.0, -2.5, 1.0).into();
        let x = v.x;

        println!("{}", x);
        println!("{}", v);
        println!("{}", v.normalized());


        let mut input = Input::new();
        let mut canvas = Canvas::new("fsdf", 400, 400).unwrap();

        graphic::api::load_gpu_function_pointers(|proc_address| {
            canvas.get_graphic_specs(proc_address)
        });

        while !canvas.should_close() {
            canvas.on_update_begin();
            canvas.process_events(&mut input);

            graphic::api::clear();
            graphic::api::clear_color(0.0, 0.0, 1.0, 1.0);

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
}
