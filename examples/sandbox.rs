extern crate engine;
extern crate common;
extern crate context;
extern crate graphic;
extern crate math;

use engine::Engine;

fn main() {
    math::health();
    common::health();
    graphic::health();
    context::health();

    let engine = Engine::new();
    engine.run();
}