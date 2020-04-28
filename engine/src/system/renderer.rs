extern crate ecs;

use ecs::prelude::*;

pub fn create() -> Schedule {
    let system = SystemBuilder::new("RendererSystem").build(|_, _, _, _| ());

    Schedule::builder().add_system(system).build()
}
