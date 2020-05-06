extern crate legion;

use legion::systems::resource::Resources;
use legion::systems::schedule::Schedule;
use legion::world::World;

pub struct Scene {
    world: World,
    systems: Vec<Schedule>,
}

impl Scene {
    pub fn new(world: World) -> Self {
        Scene {
            world,
            systems: Vec::new(),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn add_system(&mut self, system: Schedule) {
        self.systems.push(system);
    }

    pub fn execute_systems(&mut self, resources: &mut Resources) {
        let world = &mut self.world;
        self.systems
            .iter_mut()
            .for_each(|schedule| schedule.execute(world, resources));
    }
}
