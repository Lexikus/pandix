extern crate pandix;

use pandix::ecs::*;
use pandix::*;

enum Scene {
    Main = 0,
    Test = 1,
}

#[derive(Debug)]
struct Test(i32);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Static;

fn main() {
    let mut engine = Engine::new();
    engine.create_scene(Scene::Main as u8);
    engine.create_scene(Scene::Test as u8);

    engine.add_sprite("whatever", "whatever");
    engine.add_sprite("whatever2", "whatever2");

    engine.add_entities(
        Scene::Main as u8,
        (Static,),
        vec![
            (Test(0), component::Sprite::new("whatever")),
            (Test(2), component::Sprite::new("whatever2")),
        ],
    );
    engine.add_entities(Scene::Main as u8, (Static,), vec![(Test(1),), (Test(2),)]);
    engine.add_entities(Scene::Main as u8, (Static,), vec![(Test(3),)]);
    engine.add_entities(
        Scene::Main as u8,
        (Static,),
        vec![(Test(4),), (Test(5),), (Test(6),)],
    );

    let key_system_test_scheduleable = SystemBuilder::new("door")
        .read_resource::<resource::Input>()
        .read_resource::<resource::Tick>()
        .build(|_, _, (input, tick), _| {
            if input.is_key_pressed(keyboard::Key::K) {
                println!("{}", tick.delta_time() as f64);
            }
        });

    let key_system_test = Schedule::builder()
        .add_system(key_system_test_scheduleable)
        .build();

    engine.add_system(key_system_test);
    engine.run();
}
