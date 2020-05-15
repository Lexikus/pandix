extern crate legion;

use legion::systems::resource::Resources;
use legion::systems::schedule::Schedule;
use legion::world::World;

pub(crate) struct Scene {
    world: World,
    systems: Vec<Schedule>,
}

impl Scene {
    pub(crate) fn new(world: World) -> Self {
        Scene {
            world,
            systems: Vec::new(),
        }
    }

    pub(crate) fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub(crate) fn add_system(&mut self, system: Schedule) {
        self.systems.push(system);
    }

    pub(crate) fn execute_systems(&mut self, resources: &mut Resources) {
        let world = &mut self.world;
        self.systems
            .iter_mut()
            .for_each(|schedule| schedule.execute(world, resources));
    }
}
