extern crate context;
extern crate graphic;
extern crate legion;
extern crate math;

use std::collections::HashMap;

use legion::filter::ChunksetFilterData;
use legion::filter::Filter;
use legion::systems::resource::Resource;
use legion::systems::resource::Resources;
use legion::systems::schedule::Schedule;
use legion::world::IntoComponentSource;
use legion::world::TagLayout;
use legion::world::TagSet;
use legion::world::Universe;

use context::canvas::Canvas;
use context::input;
use context::input::Input;
use context::keyboard::Action;
use context::keyboard::Button;
use context::keyboard::Key;
use context::keyboard::Modifier;
use context::Event;

use crate::scene::Scene;
use crate::scene_management::SceneManagement;
use crate::system;
use crate::tick;
use crate::tick::Tick;

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
        resources.insert(SceneManagement::new());
        resources.insert(Input::new());
        resources.insert(Tick::new());

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

        if let Some(ref mut scene_management) = self.resources.get_mut::<SceneManagement>() {
            if scene_management.current() == -1 {
                scene_management.change(0);
            }
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
    pub fn run(mut self) {
        let (mut canvas, canvas_loop) = Canvas::new("pandix engine", 400, 400).unwrap();

        graphic::api::load_graphic_functions_from_context(|proc_address| {
            canvas.get_context_proc_address(proc_address)
        });

        canvas_loop.run(&canvas, move |events| {
            let resources = &mut self.resources;

            for event in events {
                match event {
                    Event::KeyDown {
                        keycode,
                        keymod,
                        repeat,
                        ..
                    } => {
                        if let Some(ref mut input) = resources.get_mut::<Input>() {
                            let key: Key = if keycode.is_some() {
                                keycode.unwrap().into()
                            } else {
                                Key::Unknown
                            };
                            let action = if repeat {
                                Action::Repeat
                            } else {
                                Action::Press
                            };
                            let modifier: Modifier = keymod.into();
                            let button = Button::new(key, action, modifier);

                            input::update(input, key, button);
                        }
                    }
                    context::Event::KeyUp {
                        keycode, keymod, ..
                    } => {
                        if let Some(ref mut input) = resources.get_mut::<Input>() {
                            let key: Key = if keycode.is_some() {
                                keycode.unwrap().into()
                            } else {
                                Key::Unknown
                            };
                            let action = Action::Release;
                            let modifier: Modifier = keymod.into();
                            let button = Button::new(key, action, modifier);

                            input::update(input, key, button);
                        }
                    }
                    _ => (),
                }
            }

            if let Some(tick) = &mut resources.get_mut::<Tick>() {
                tick::update(tick);
            }

            graphic::api::clear_color(0.0, 0.0, 1.0, 1.0);

            let current_scene = resources.get::<SceneManagement>().unwrap().current();
            // TODO: remove unwrap
            let scene = self.scenes.get_mut(&current_scene).unwrap();

            // execute global systems
            self.systems
                .iter_mut()
                .for_each(|schedule| schedule.execute(scene.world_mut(), resources));

            // execute scene systems
            scene.execute_systems(resources);

            // execute engine render system
            self.render_system.execute(scene.world_mut(), resources);

            let mut input = resources.get_mut::<Input>().unwrap();

            input::clean_up(&mut input);
        });
    }
}
