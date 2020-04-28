use legion::prelude::*;

// Define our entity data types
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Model(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Static;

pub fn run() {
    // Create a world to store our entities
    let universe = Universe::new();
    let mut world = universe.create_world();

    // Create entities with `Position` and `Velocity` data
    world.insert(
        (),
        (0..999).map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 })),
    );

    // Create entities with `Position` data and a shared `Model` data, tagged as `Static`
    // Shared data values are shared across many entities,
    // and enable further batch processing and filtering use cases
    let _entities: &[Entity] = world.insert(
        (Model(5), Static),
        (0..999).map(|_| (Position { x: 0.0, y: 0.0 },)),
    );

    // Create a query which finds all `Position` and `Velocity` components
    let query = <(Write<Position>, Read<Velocity>)>::query();

    // Iterate through all entities that match the query in the world
    for (mut pos, vel) in query.iter(&mut world) {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }

    // -----------------------------------------------------------------
    let mut systems: Vec<Box<dyn Fn(&mut World) -> ()>> = Vec::new();
    systems.push(Box::new(|m| {
        let query = <Read<Velocity>>::query();
        query.iter(m).for_each(|entity| println!("{:?}", entity));
    }));
    systems.iter().for_each(|f| f(&mut world));
    // -----------------------------------------------------------------

    // -----------------------------------------------------------------
    let mut systems: Vec<Schedule> = Vec::new();
    let read_velocity_system = SystemBuilder::new("ReadVelocitySystem")
        .with_query(<Read<Velocity>>::query())
        .build(|_, world, _, queries| {
            queries
                .iter(world)
                .for_each(|entity| println!("{:?}", entity));
        });

    let schedule = Schedule::builder().add_system(read_velocity_system).build();

    systems.push(schedule);
    systems.iter_mut().for_each(|s| s.execute(&mut world));
    // -----------------------------------------------------------------

    // -----------------------------------------------------------------
    let query = <Read<Velocity>>::query();
    query
        .iter(&mut world)
        .for_each(|entity| println!("{:?}", entity));
    // -----------------------------------------------------------------
}
