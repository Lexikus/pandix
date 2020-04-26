extern crate ecs;

use ecs::World;

pub struct Scene {
    world: World
}

impl Scene {
    pub fn new(world: World) -> Self {
        Scene {
            world
        }
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}