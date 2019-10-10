extern crate common;
extern crate context;
extern crate graphic;
extern crate math;

fn main() {
    math::health();
    common::health();
    graphic::health();
    context::health();

    let mut context = match context::canvas::Canvas::new("test", 400, 400) {
        Ok(c) => c,
        Err(_) => {
            println!("error");
            return;
        }
    };

    let mut input = context::input::Input::new();

    while !context.should_close() {
        context.on_update_begin(&mut input);

        if input.is_key_released(context::keyboard::Key::Q) {
            // println!("Q pressed");
        } else {
            // println!("_");
        }

        input.on_update_end();
        context.on_update_end();
    }
}
