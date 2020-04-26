extern crate pandix;

use pandix::Engine;
use pandix::ecs::prelude::*;

enum Scene {
    Main = 0
}

#[derive(Debug)]
struct Test(i32);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Static;

fn main() {
    let mut engine = Engine::new();
    engine.create_scene(Scene::Main as u8);
    engine.add_entities(Scene::Main as u8, (Static,), vec![(Test(1),), (Test(2),)]);
    engine.add_entities(Scene::Main as u8, (Static,), vec![(Test(3),)]);
    engine.add_entities(Scene::Main as u8, (Static,), vec![(Test(4),), (Test(5),), (Test(6),)]);

    let log_test_schedulable = SystemBuilder::new("LogTest")
        .with_query(<(Read<Test>)>::query())
        .build(|_, world, _, queries| {
            queries
                .iter_entities(world)
                .for_each(|(_, value)| println!("{:?}", value.0))
        });
    let log_test_system = Schedule::builder()
        .add_system(log_test_schedulable)
        .build();

    engine.add_system(log_test_system);
    engine.run();
}
