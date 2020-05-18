use crate::debug::Debug;
use crate::graphics::window::winit;
use crate::graphics::{Window, WindowSettings};
use crate::input::{self, gamepad, keyboard, mouse, window, Input};
use crate::load::{Join, LoadingScreen, Task};
use crate::{Result, Timer};
use std::convert::TryInto;

pub trait Loop<Game: super::Game> {
    type Attributes;

    fn new(
        configuration: Self::Attributes,
        game: &mut Game,
        window: &Window,
    ) -> Self;

    fn load(window: &Window) -> Task<Self::Attributes>;

    fn on_input(&mut self, input: &mut Game::Input, event: input::Event) {
        input.update(event);
    }

    fn after_draw(
        &mut self,
        _game: &mut Game,
        _input: &mut Game::Input,
        _window: &mut Window,
        _debug: &mut Debug,
    ) {
    }

    fn run(window_settings: WindowSettings) -> Result<()>
    where
        Self: 'static + Sized,
        Game: 'static,
        Game::Input: 'static,
    {
        // Window creation
        let event_loop = winit::event_loop::EventLoop::new();
        let mut window = Window::new(window_settings, &event_loop)?;
        let mut debug = Debug::new(window.gpu());

        // Loading
        debug.loading_started();
        let (mut game, configuration) = {
            let mut loading_screen = Game::LoadingScreen::new(window.gpu())?;

            loading_screen.run(
                (Game::load(&window), Self::load(&window)).join(),
                &mut window,
            )?
        };

        let mut game_loop = Self::new(configuration, &mut game, &mut window);
        let mut input = Game::Input::new();
        let mut gamepads = gamepad::Tracker::new();
        debug.loading_finished();

        let mut timer = Timer::new(Game::TICKS_PER_SECOND);

        // Initialization
        debug.frame_started();
        timer.update();

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::NewEvents(_) => {
                debug.interact_started();
            }
            winit::event::Event::MainEventsCleared => {
                if let Some(tracker) = &mut gamepads {
                    while let Some((id, event, time)) = tracker.next_event() {
                        game_loop.on_input(
                            &mut input,
                            input::Event::Gamepad { id, event, time },
                        );
                    }
                }

                game.interact(&mut input, &mut window);
                input.clear();
                debug.interact_finished();

                if timer.tick() {
                    debug.update_started();
                    game.update(&window);
                    debug.update_finished();
                }

                window.request_redraw();

                if game.is_finished() {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
            }
            winit::event::Event::RedrawRequested { .. } => {
                if game.should_draw() {
                    debug.draw_started();
                    game.draw(&mut window.frame(), &timer);
                    debug.draw_finished();

                    game_loop.after_draw(
                        &mut game,
                        &mut input,
                        &mut window,
                        &mut debug,
                    );

                    if debug.is_enabled() {
                        debug.debug_started();
                        game.debug(&input, &mut window.frame(), &mut debug);
                        debug.debug_finished();
                    }

                    window.swap_buffers();
                    debug.frame_finished();

                    debug.frame_started();
                    window.request_redraw();
                }

                timer.update();
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    if game.on_close_request() {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                }
                winit::event::WindowEvent::Resized(logical_size) => {
                    window.resize(logical_size);
                }
                _ => {
                    match event {
                        winit::event::WindowEvent::KeyboardInput {
                            input:
                                winit::event::KeyboardInput {
                                    virtual_keycode,
                                    state: winit::event::ElementState::Released,
                                    ..
                                },
                            ..
                        } if Game::DEBUG_KEY.is_some() => {
                            if virtual_keycode == Game::DEBUG_KEY {
                                debug.toggle();
                            }
                        }
                        _ => {}
                    }

                    if let Some(input_event) = try_into_input_event(event) {
                        game_loop.on_input(&mut input, input_event);
                    }
                }
            },
            _ => {}
        });
    }
}

fn try_into_input_event(
    event: winit::event::WindowEvent<'_>,
) -> Option<input::Event> {
    match event {
        winit::event::WindowEvent::KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    state,
                    virtual_keycode: Some(key_code),
                    ..
                },
            ..
        } => Some(input::Event::Keyboard(keyboard::Event::Input {
            state,
            key_code,
        })),
        winit::event::WindowEvent::ReceivedCharacter(codepoint) => {
            Some(input::Event::Keyboard(keyboard::Event::TextEntered {
                character: codepoint,
            }))
        }
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            Some(input::Event::Mouse(mouse::Event::Input { state, button }))
        }
        winit::event::WindowEvent::MouseWheel { delta, .. } => match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                Some(input::Event::Mouse(mouse::Event::WheelScrolled {
                    delta_x: x,
                    delta_y: y,
                }))
            }
            _ => None,
        },
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(input::Event::Mouse(mouse::Event::CursorMoved {
                x: position.x as f32,
                y: position.y as f32,
            }))
        }
        winit::event::WindowEvent::CursorEntered { .. } => {
            Some(input::Event::Mouse(mouse::Event::CursorEntered))
        }
        winit::event::WindowEvent::CursorLeft { .. } => {
            Some(input::Event::Mouse(mouse::Event::CursorLeft))
        }
        winit::event::WindowEvent::Focused(focus) => Some(if focus == true {
            input::Event::Window(window::Event::Focused)
        } else {
            input::Event::Window(window::Event::Unfocused)
        }),
        winit::event::WindowEvent::Moved(position) => {
            Some(input::Event::Window(window::Event::Moved {
                x: position.x as f32,
                y: position.y as f32,
            }))
        }
        _ => None,
    }
}

pub struct Default {}

impl<Game: super::Game> Loop<Game> for Default
where
    Game: 'static,
{
    type Attributes = ();

    fn new(
        _attributes: Self::Attributes,
        _game: &mut Game,
        _window: &Window,
    ) -> Self {
        Self {}
    }

    fn load(_window: &Window) -> Task<Self::Attributes> {
        Task::succeed(|| ())
    }

    fn after_draw(
        &mut self,
        game: &mut Game,
        _input: &mut Game::Input,
        window: &mut Window,
        _debug: &mut Debug,
    ) {
        window.update_cursor(game.cursor_icon().try_into().ok());
    }
}
