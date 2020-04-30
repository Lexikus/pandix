#![allow(dead_code)]

extern crate glutin;

use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::Window;
use glutin::window::WindowBuilder;
use glutin::{ContextWrapper, ContextBuilder, PossiblyCurrent};

#[derive(Debug)]
pub enum CanvasError {
    CreatingWindowFailed,
}

pub struct Canvas {
    title: String,
    width: u32,
    height: u32,
    context: ContextWrapper<PossiblyCurrent, Window>
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Result<(Canvas, CanvasLoop), CanvasError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new();
        let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
        let context = unsafe { context.make_current().unwrap() };

        Ok((
            Canvas {
                title: title.to_owned(),
                width,
                height,
                context,
            },
            CanvasLoop(event_loop),
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

    pub fn context(&self) -> &ContextWrapper<PossiblyCurrent, Window> {
        &self.context
    }

    pub fn get_graphic_specs(&mut self, proc_address: &'static str) -> *const std::ffi::c_void {
        self.context.get_proc_address(proc_address)
    }
}

pub struct CanvasLoop(EventLoop<()>);

impl CanvasLoop {
    pub fn run<F>(self, mut function: F)
    where
        F: 'static + FnMut(Event<'_, ()>),
    {
        self.0.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            }

            function(event);
        });
    }
}
