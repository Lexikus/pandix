extern crate engine;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();

    // NOTE(alex):
    // do stuff with the engine like appending the game state to it or whatever.
    // Need to think this through.

    engine.run();
}
