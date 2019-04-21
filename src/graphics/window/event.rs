use super::winit;
use crate::input;

pub(crate) enum Event {
    CloseRequested,
    Resized(winit::dpi::LogicalSize),
    Input(input::Event),
    CursorMoved(winit::dpi::LogicalPosition),
}

pub struct EventLoop(winit::EventsLoop);

impl EventLoop {
    pub fn new() -> Self {
        Self(winit::EventsLoop::new())
    }

    pub(super) fn raw(&self) -> &winit::EventsLoop {
        &self.0
    }

    pub(crate) fn poll<F>(&mut self, mut f: F)
    where
        F: FnMut(Event),
    {
        self.0.poll_events(|event| {
            match event {
                winit::Event::WindowEvent { event, .. } => match event {
                    winit::WindowEvent::KeyboardInput {
                        input:
                            winit::KeyboardInput {
                                state,
                                virtual_keycode: Some(key_code),
                                ..
                            },
                        ..
                    } => {
                        f(Event::Input(input::Event::KeyboardInput {
                            state,
                            key_code,
                        }));
                    }
                    winit::WindowEvent::CursorMoved { position, .. } => {
                        f(Event::CursorMoved(position))
                    }
                    winit::WindowEvent::CloseRequested => {
                        f(Event::CloseRequested)
                    }
                    winit::WindowEvent::Resized(logical_size) => {
                        f(Event::Resized(logical_size))
                    }
                    _ => {}
                },
                _ => (),
            };
        });
    }
}
