extern crate context;
extern crate graphic;
extern crate legion;
extern crate math;

use legion::filter::ChunksetFilterData;
use legion::filter::Filter;
use legion::prelude::*;
use legion::systems::resource::Resource;
use legion::world::IntoComponentSource;
use legion::world::TagLayout;
use legion::world::TagSet;

use std::collections::HashMap;

use context::canvas::Canvas;

use crate::resource;
use crate::scene::Scene;
use crate::system;

pub struct Engine {
    universe: Universe,
    scenes: HashMap<i16, Scene>,
    render_system: Schedule,
    systems: Vec<Schedule>,
    resources: Resources,
}

impl Engine {
    pub fn new() -> Self {
        let mut resources = Resources::default();
        resources.insert(resource::SceneState::new());

        Engine {
            universe: Universe::new(),
            scenes: HashMap::new(),
            render_system: system::renderer::create(),
            systems: Vec::new(),
            resources,
        }
    }

    pub fn create_scene(&mut self, key: u8) {
        let world = self.universe.create_world();
        let scene = Scene::new(world);

        self.scenes.insert(key as i16, scene);

        let scene_state = &mut self.resources.get_mut::<resource::SceneState>().unwrap();
        if scene_state.current_scene == -1 {
            scene_state.current_scene = 0;
        }
    }

    pub fn add_system(&mut self, system: Schedule) {
        self.systems.push(system);
    }

    pub fn add_scene_system(&mut self, scene: u8, system: Schedule) {
        if let Some(scene) = self.scenes.get_mut(&(scene as i16)) {
            scene.add_system(system);
        }
    }

    pub fn add_entities<T, C>(&mut self, scene: u8, tags: T, components: C)
    where
        T: TagSet + TagLayout + for<'a> Filter<ChunksetFilterData<'a>>,
        C: IntoComponentSource,
    {
        if let Some(scene) = self.scenes.get_mut(&(scene as i16)) {
            scene.world_mut().insert(tags, components);
        }
    }

    pub fn add_resource<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        self.resources.insert(resource);
    }

    // TODO: return Error
    pub fn run(&mut self) {
        let mut canvas = Canvas::new("pandix engine", 400, 400).unwrap();

        graphic::api::load_gpu_function_pointers(|proc_address| {
            canvas.get_graphic_specs(proc_address)
        });

        let resource = &mut self.resources;
        while !canvas.should_close() {
            canvas.on_update_begin();

            graphic::api::clear();
            graphic::api::clear_color(0.0, 0.0, 1.0, 1.0);

            let current_scene = resource
                .get::<resource::SceneState>()
                .unwrap()
                .current_scene;
            // TODO: remove unwrap
            let scene = self.scenes.get_mut(&current_scene).unwrap();

            // execute global systems
            self.systems
                .iter_mut()
                .for_each(|schedule| schedule.execute(scene.world_mut(), resource));

            // execute scene systems
            scene.execute_systems(resource);

            // execute engine render system
            self.render_system.execute(scene.world_mut(), resource);

            canvas.on_update_end();
        }
    }
}
