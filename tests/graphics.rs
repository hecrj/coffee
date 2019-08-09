use coffee::graphics::{Color, Frame, Quad, Window, WindowSettings};
use coffee::input::{keyboard, Keyboard};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use std::time;

mod _graphics;

use _graphics::{test, Test};

#[test]
#[ignore]
fn graphics() -> Result<()> {
    Runner::run(WindowSettings {
        title: String::from("Graphics test suite - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

struct Runner {
    state: State,
}

pub enum State {
    Pending {
        tests: Vec<Test>,
    },
    Drawing {
        remaining: Vec<Test>,
        current: test::Execution,
        done: Vec<test::Output>,
    },
    Diffing {
        outputs: Vec<test::Output>,
    },
    Reporting {
        results: Vec<test::Result>,
    },
    Finished {
        at: time::Instant,
    },
}

impl Game for Runner {
    type LoadingScreen = ();
    type Input = Keyboard;

    fn load(_window: &Window) -> Task<Runner> {
        Task::succeed(|| Runner {
            state: State::Pending { tests: Test::all() },
        })
    }

    fn interact(&mut self, keyboard: &mut Keyboard, window: &mut Window) {
        let now = time::Instant::now();

        // We need to own the current state to avoid awkward copies.
        // TODO: Not sure if there is a better way to do this.
        // Something like `replace` but taking a closure would be nice.
        let state =
            std::mem::replace(&mut self.state, State::Finished { at: now });

        self.state = match state {
            State::Pending { mut tests } => {
                if let Some(first) = tests.pop() {
                    State::Drawing {
                        remaining: tests,
                        current: first.run(window.gpu()),
                        done: Vec::new(),
                    }
                } else {
                    State::Finished { at: now }
                }
            }
            State::Drawing {
                mut remaining,
                current,
                mut done,
            } => {
                if keyboard.was_key_released(keyboard::KeyCode::Right) {
                    done.push(current.store(window.gpu()));

                    if let Some(next) = remaining.pop() {
                        State::Drawing {
                            remaining,
                            current: next.run(window.gpu()),
                            done,
                        }
                    } else {
                        State::Diffing { outputs: done }
                    }
                } else {
                    State::Drawing {
                        remaining,
                        current,
                        done,
                    }
                }
            }
            State::Diffing { outputs } => State::Reporting {
                results: outputs.iter().map(|t| t.diff()).collect(),
            },
            State::Reporting { .. } => State::Finished { at: now },
            State::Finished { .. } => state,
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        match &self.state {
            State::Drawing { current, .. } => {
                let canvas = current.canvas();

                canvas.draw(
                    Quad {
                        size: (canvas.width() as f32, canvas.height() as f32),
                        ..Quad::default()
                    },
                    &mut frame.as_target(),
                );
            }
            _ => {}
        }
    }

    fn is_finished(&self) -> bool {
        match self.state {
            State::Finished { at } => {
                (time::Instant::now() - at).as_secs() >= 2
            }
            _ => false,
        }
    }
}
