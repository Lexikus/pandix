extern crate common;
extern crate context;
extern crate graphic;
extern crate math;

fn main() {
    math::health();
    common::health();
    graphic::health();
    context::health();

    let mut input = context::input::Input::new();

    input.update(
        context::keyboard::Key::A,
        context::keyboard::Button::new(
            context::keyboard::Key::A,
            context::keyboard::Action::Press,
            context::keyboard::Modifier::Unknown,
        ),
    );

    input.on_update_end();
}
