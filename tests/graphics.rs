use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

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
        tests: Vec<Task<Test>>,
    },
    Drawing {
        remaining: Vec<Task<Test>>,
        current: Test,
        done: Vec<test::Drawing>,
    },
    Diffing {
        tests: Vec<test::Drawing>,
    },
    Reporting {
        results: Vec<test::Result>,
    },
    Finished,
}

impl Game for Runner {
    type LoadingScreen = ();
    type Input = ();

    fn load(_window: &Window) -> Task<Runner> {
        Task::succeed(|| Runner {
            state: State::Pending { tests: Test::all() },
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        // We need to own the current state to avoid awkward copies.
        // TODO: Not sure if there is a better way to do this.
        // Something like `replace` but taking a closure would be nice.
        let state = std::mem::replace(&mut self.state, State::Finished);

        self.state = match state {
            State::Pending { mut tests } => {
                if let Some(load_first) = tests.pop() {
                    State::Drawing {
                        remaining: tests,
                        current: load_first
                            .run(frame.gpu())
                            .expect("Load test"),
                        done: Vec::new(),
                    }
                } else {
                    State::Finished
                }
            }
            State::Drawing {
                mut remaining,
                current,
                mut done,
            } => {
                let drawing = current.draw(frame.gpu());
                done.push(drawing);

                if let Some(load_next) = remaining.pop() {
                    State::Drawing {
                        remaining,
                        current: load_next.run(frame.gpu()).expect("Load test"),
                        done,
                    }
                } else {
                    State::Diffing { tests: done }
                }
            }
            State::Diffing { mut tests } => State::Reporting {
                results: tests.drain(..).map(|t| t.diff()).collect(),
            },
            State::Reporting { .. } => State::Finished,
            State::Finished => State::Finished,
        }
    }

    fn is_finished(&self) -> bool {
        match self.state {
            State::Finished => true,
            _ => false,
        }
    }
}
