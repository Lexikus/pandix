#![allow(dead_code)]
extern crate context;
extern crate ecs;
extern crate graphic;
extern crate math;

mod scene;
mod system;

use std::collections::HashMap;

use ecs::prelude::*;

use context::canvas::Canvas;

use scene::Scene;

struct Test;

pub struct Engine {
    universe: Universe,
    active_scene: i16,
    scenes: HashMap<i16, Scene>,
    render_system: Schedule,
    systems: Vec<Schedule>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            universe: Universe::new(),
            active_scene: -1,
            scenes: HashMap::new(),
            render_system: system::renderer::create(),
            systems: Vec::new(),
        }
    }

    pub fn create_scene(&mut self, key: u8) {
        let world = self.universe.create_world();
        let scene = Scene::new(world);

        self.scenes.insert(key as i16, scene);

        if self.active_scene == -1 {
            self.active_scene = 0;
        }
    }

    pub fn add_system(&mut self, system: Schedule) {
        self.systems.push(system);
    }

    pub fn add_entities<T, C>(&mut self, scene: u8, tags: T, components: C)
    where
        T: ecs::TagSet + ecs::TagLayout + for<'a> ecs::Filter<ecs::ChunksetFilterData<'a>>,
        C: ecs::IntoComponentSource,
    {
        if let Some(scene) = self.scenes.get_mut(&(scene as i16)) {
            scene.world_mut().insert(tags, components);
        }
    }

    // TODO: return Error
    pub fn run(&mut self) {
        let mut canvas = Canvas::new("fsdf", 400, 400).unwrap();

        graphic::api::load_gpu_function_pointers(|proc_address| {
            canvas.get_graphic_specs(proc_address)
        });

        while !canvas.should_close() {
            canvas.on_update_begin();

            graphic::api::clear();
            graphic::api::clear_color(0.0, 0.0, 1.0, 1.0);

            let world = self.scenes.get_mut(&self.active_scene).unwrap().world_mut();

            // Systems
            // Game systems
            self.systems
                .iter_mut()
                .for_each(|schedule| schedule.execute(world));
            // Engine render system
            self.render_system.execute(world);

            canvas.on_update_end();
        }
    }
}
