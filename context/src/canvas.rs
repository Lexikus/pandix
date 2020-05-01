#![allow(dead_code)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::video::GLContext;
use sdl2::video::GLProfile;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

const OPENGL_MAJOR_VERSION: u8 = 4;
const OPENGL_MINOR_VERSION: u8 = 0;

#[derive(Debug)]
pub enum CanvasError {
    CreatingWindowFailed,
    CreatingContextFailed,
    CreatingEventHandlerFailed,
}

pub struct Canvas {
    title: String,
    width: u32,
    height: u32,
    sdl: Sdl,
    window: Window,
    subsystem: VideoSubsystem,
    context: GLContext,
}

impl Canvas {
    // TODO: more options like resizable, fullscreen | fullscreen borderless, vsync etc.
    //       maybe create a CanvasBuilder?
    pub fn new(title: &str, width: u32, height: u32) -> Result<(Canvas, CanvasLoop), CanvasError> {
        let sdl = sdl2::init().map_err(|_| CanvasError::CreatingWindowFailed)?;
        let subsystem = sdl.video().map_err(|_| CanvasError::CreatingWindowFailed)?;

        let gl_attr = subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(OPENGL_MAJOR_VERSION, OPENGL_MINOR_VERSION);

        let window = subsystem
            .window(title, width, height)
            .opengl()
            .build()
            .map_err(|_| CanvasError::CreatingWindowFailed)?;
        let context = window
            .gl_create_context()
            .map_err(|_| CanvasError::CreatingContextFailed)?;
        let event_pump = sdl
            .event_pump()
            .map_err(|_| CanvasError::CreatingEventHandlerFailed)?;
        let cavas_loop = CanvasLoop(event_pump);

        Ok((
            Canvas {
                title: title.to_owned(),
                width,
                height,
                sdl,
                window,
                subsystem,
                context,
            },
            cavas_loop,
        ))
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

    pub fn subsystem(&self) -> &VideoSubsystem {
        &self.subsystem
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn context(&self) -> &GLContext {
        &self.context
    }

    pub fn get_context_proc_address(
        &mut self,
        proc_address: &'static str,
    ) -> *const std::ffi::c_void {
        self.subsystem.gl_get_proc_address(proc_address) as *const _
    }
}

pub struct CanvasLoop(EventPump);

impl CanvasLoop {
    pub fn run<F>(mut self, canvas: &Canvas, mut function: F)
    where
        F: FnMut(Vec<Event>),
    {
        'running: loop {
            // TODO try to get it to work without allocation on the heap
            let iter: Vec<_> = self.0.poll_iter().collect();
            for event in &iter {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            function(iter);

            canvas.window().gl_swap_window();
        }
    }
}
