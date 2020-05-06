extern crate legion;

use legion::systems::schedule::Schedule;
use legion::systems::SystemBuilder;

pub fn create() -> Schedule {
    let system = SystemBuilder::new("RendererSystem").build(|_, _, _, _| ());

    Schedule::builder().add_system(system).build()
}
