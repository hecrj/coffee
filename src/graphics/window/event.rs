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
                    winit::WindowEvent::MouseInput {
                        state, button, ..
                    } => f(Event::Input(input::Event::MouseInput {
                        state,
                        button,
                    })),
                    winit::WindowEvent::MouseWheel { delta, .. } => match delta
                    {
                        winit::MouseScrollDelta::LineDelta(x, y) => {
                            f(Event::Input(input::Event::MouseWheel {
                                delta_x: x,
                                delta_y: y,
                            }))
                        }
                        _ => {}
                    },

                    winit::WindowEvent::ReceivedCharacter(codepoint) => {
                        f(Event::Input(input::Event::TextInput {
                            character: codepoint,
                        }))
                    }
                    winit::WindowEvent::CursorMoved { position, .. } => {
                        f(Event::CursorMoved(position))
                    }
                    winit::WindowEvent::CursorEntered { .. } => {
                        f(Event::Input(input::Event::CursorEntered))
                    }
                    winit::WindowEvent::CursorLeft { .. } => {
                        f(Event::Input(input::Event::CursorLeft))
                    }
                    winit::WindowEvent::CloseRequested { .. } => {
                        f(Event::CloseRequested)
                    }
                    winit::WindowEvent::Resized(logical_size) => {
                        f(Event::Resized(logical_size))
                    }
                    winit::WindowEvent::Focused(focus) => {
                        f(Event::Input(if focus == true {
                            input::Event::WindowFocused
                        } else {
                            input::Event::WindowUnfocused
                        }))
                    }
                    winit::WindowEvent::Moved(
                        winit::dpi::LogicalPosition { x, y },
                    ) => f(Event::Input(input::Event::WindowMoved {
                        x: x as f32,
                        y: y as f32,
                    })),
                    _ => {}
                },
                _ => (),
            };
        });
    }
}
