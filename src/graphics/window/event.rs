use super::winit;
use crate::input;

pub(crate) enum Event {
    CloseRequested,
    Resized(NewSize),
    Input(input::Event),
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
                                virtual_keycode: Some(virtual_keycode),
                                ..
                            },
                        ..
                    } => {
                        f(Event::Input(input::Event::KeyboardInput {
                            state: match state {
                                winit::ElementState::Pressed => {
                                    input::KeyState::Pressed
                                }
                                winit::ElementState::Released => {
                                    input::KeyState::Released
                                }
                            },
                            key_code: virtual_keycode,
                        }));
                    }
                    winit::WindowEvent::CloseRequested => {
                        f(Event::CloseRequested)
                    }
                    winit::WindowEvent::Resized(logical_size) => {
                        f(Event::Resized(NewSize(logical_size)))
                    }
                    _ => {}
                },
                _ => (),
            };
        });
    }
}

pub(crate) struct NewSize(winit::dpi::LogicalSize);

impl NewSize {
    pub fn to_physical(&self, dpi: f64) -> winit::dpi::PhysicalSize {
        self.0.to_physical(dpi)
    }
}
