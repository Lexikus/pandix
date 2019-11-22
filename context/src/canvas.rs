#![allow(dead_code)]

extern crate glfw;
extern crate graphic;

const OPENGL_MAJOR_VERSION: u32 = 4;
const OPENGL_MINOR_VERSION: u32 = 0;

use super::input::Input;
use super::keyboard::Button;

use std::sync::mpsc::Receiver;

use glfw::Context;
use glfw::FlushedMessages;
use glfw::Glfw;
use glfw::OpenGlProfileHint;
use glfw::SwapInterval;
use glfw::Window;
use glfw::WindowEvent;
use glfw::WindowHint;
use glfw::WindowMode;
use glfw::FAIL_ON_ERRORS;

#[derive(Debug)]
pub enum CanvasError {
    CanvasInitFailed,
    CreatingWindowFailed,
}

pub struct Canvas {
    title: String,
    width: u32,
    height: u32,
    window: Window,
    glfw: Glfw,
    events: Receiver<(f64, WindowEvent)>,
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Canvas, CanvasError> {
        let mut glfw = glfw::init(FAIL_ON_ERRORS).map_err(|_| CanvasError::CanvasInitFailed)?;

        glfw.window_hint(WindowHint::ContextVersion(
            OPENGL_MAJOR_VERSION,
            OPENGL_MINOR_VERSION,
        ));

        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut window, receiver) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .ok_or(CanvasError::CreatingWindowFailed)?;

        glfw.make_context_current(Some(&window));

        graphic::api::load_with(|s| window.get_proc_address(s));

        window.set_key_polling(true);

        Ok(Canvas {
            title: title.to_owned(),
            width,
            height,
            window,
            glfw,
            events: receiver,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_vsync(&mut self, enable: bool) {
        if enable {
            self.glfw.set_swap_interval(SwapInterval::Adaptive);
        } else {
            self.glfw.set_swap_interval(SwapInterval::None);
        }
    }

    pub fn on_update_begin(&mut self) {
        self.glfw.poll_events();
    }

    pub fn poll_events(&self) -> FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.events)
    }

    pub fn on_update_end(&mut self) {
        self.window.swap_buffers();
    }

    pub fn terminate() {
        glfw::terminate();
    }

    pub fn on_update_external_events(&self, input: &mut Input) {
        for (_, message) in self.poll_events() {
            match message {
                WindowEvent::Key(key, _, action, modifiers) => {
                    let action = action.into();
                    let key = key.into();
                    let modifier = modifiers.into();

                    input.update(key, Button::new(key, action, modifier));
                }
                WindowEvent::Size(x, y) => {
                    println!("{} {}", x, y);
                }
                _ => (),
            };
        }
    }
}
