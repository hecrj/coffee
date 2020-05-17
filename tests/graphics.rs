#![cfg(not(target_os = "windows"))]
use coffee::graphics::{
    Color, Frame, Gpu, Point, Quad, Window, WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{
    button, Button, Checkbox, Column, Element, Justify, Panel, Renderer, Row,
    Text, UserInterface,
};
use coffee::{Game, Result, Timer};

mod _graphics;
use _graphics::{test, Test};

#[test]
#[ignore]
fn graphics() -> Result<()> {
    env_logger::init();

    <Runner as UserInterface>::run(WindowSettings {
        title: String::from("Graphics integration tests - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Runner {
    state: State,
}

pub enum State {
    Pending {
        tests: Vec<Test>,
    },
    Running {
        remaining: Vec<Test>,
        current: test::Drawing,
    },
    AskingToSaveModelImage {
        remaining: Vec<Test>,
        current: test::Drawing,
        save_button: button::State,
        fail_button: button::State,
        saved: bool,
    },
    ReportingDifferences {
        remaining: Vec<Test>,
        current: test::Differences,
        show: bool,
        quit_button: button::State,
    },
    Finished,
}

struct Progress {
    remaining: usize,
    current: Option<Test>,
}

impl State {
    fn progress(&self) -> Option<Progress> {
        match self {
            State::Pending { tests } => Some(Progress {
                remaining: tests.len(),
                current: None,
            }),
            State::Running { current, remaining } => Some(Progress {
                remaining: remaining.len(),
                current: Some(current.test()),
            }),
            State::ReportingDifferences {
                current, remaining, ..
            } => Some(Progress {
                remaining: remaining.len(),
                current: Some(current.test()),
            }),
            State::AskingToSaveModelImage {
                current, remaining, ..
            } => Some(Progress {
                remaining: remaining.len(),
                current: Some(current.test()),
            }),
            State::Finished { .. } => None,
        }
    }
}

impl Runner {
    fn run_next(&mut self, gpu: &mut Gpu) {
        // We need to own the current state to avoid awkward copies.
        // TODO: Not sure if there is a better way to do this.
        // Something like `replace` but taking a closure would be nice.
        let state = std::mem::replace(&mut self.state, State::Finished);

        let next = |mut remaining: Vec<Test>, gpu: &mut Gpu| {
            if let Some(next) = remaining.pop() {
                State::Running {
                    remaining,
                    current: next.run(gpu),
                }
            } else {
                State::Finished
            }
        };

        self.state = match state {
            State::Pending { mut tests } => {
                if let Some(first) = tests.pop() {
                    State::Running {
                        remaining: tests,
                        current: first.run(gpu),
                    }
                } else {
                    State::Finished
                }
            }
            State::Running { remaining, current } => {
                let differences = current.differences(gpu);

                match differences {
                    Ok(None) => next(remaining, gpu),
                    Ok(Some(differences)) => State::ReportingDifferences {
                        remaining,
                        current: differences,
                        show: false,
                        quit_button: button::State::new(),
                    },
                    Err(test::Error::ModelImageNotFound(_)) => {
                        State::AskingToSaveModelImage {
                            remaining,
                            current,
                            save_button: button::State::new(),
                            fail_button: button::State::new(),
                            saved: false,
                        }
                    }
                    Err(error) => panic!("Something went wrong: {:?}", error),
                }
            }
            State::ReportingDifferences { remaining, .. } => {
                next(remaining, gpu)
            }
            State::AskingToSaveModelImage { remaining, .. } => {
                next(remaining, gpu)
            }
            State::Finished { .. } => state,
        }
    }
}

impl Game for Runner {
    type LoadingScreen = ();
    type Input = ();

    fn load(_window: &Window) -> Task<Runner> {
        Task::succeed(|| Runner {
            state: State::Pending { tests: Test::all() },
        })
    }

    fn interact(&mut self, _input: &mut (), window: &mut Window) {
        match self.state {
            State::Pending { .. } => self.run_next(window.gpu()),
            State::Running { .. } => self.run_next(window.gpu()),
            State::AskingToSaveModelImage { saved, .. } if saved => {
                self.run_next(window.gpu())
            }
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });

        let canvas = match &self.state {
            State::Running { current, .. } => Some(current.canvas()),
            State::ReportingDifferences { current, .. } => {
                Some(current.canvas())
            }
            State::AskingToSaveModelImage { current, .. } => {
                Some(current.canvas())
            }
            _ => None,
        };

        if let Some(canvas) = canvas {
            canvas.draw(
                Quad {
                    position: Point::new(
                        frame.width() * 0.5 - canvas.width() as f32 * 0.5,
                        frame.height() * 0.5 - canvas.height() as f32 * 0.5,
                    ),
                    size: (canvas.width() as f32, canvas.height() as f32),
                    ..Quad::default()
                },
                &mut frame.as_target(),
            );
        }

        match &self.state {
            State::ReportingDifferences { current, show, .. } if *show => {
                let image = current.image();

                image.draw(
                    Quad {
                        position: Point::new(
                            frame.width() * 0.5 - image.width() as f32 * 0.5,
                            frame.height() * 0.5 - image.height() as f32 * 0.5,
                        ),
                        size: (image.width() as f32, image.height() as f32),
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
            State::Finished => true,
            _ => false,
        }
    }

    fn on_close_request(&mut self) -> bool {
        panic!("Exited before completion")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CreateModelImage,
    FailTest,
    ToggleDifferences(bool),
}

impl UserInterface for Runner {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, msg: Message, window: &mut Window) {
        match msg {
            Message::CreateModelImage => match &mut self.state {
                State::AskingToSaveModelImage { saved, current, .. }
                    if !*saved =>
                {
                    current.save_as_model(window.gpu());

                    *saved = true;
                }
                _ => {}
            },
            Message::FailTest => {
                if let Some(progress) = self.state.progress() {
                    match progress.current {
                        Some(test) => panic!("\"{:?}\" test failed", test),
                        None => {}
                    }
                }
            }
            Message::ToggleDifferences(value) => {
                if let State::ReportingDifferences { show, .. } =
                    &mut self.state
                {
                    *show = value;
                }
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        let progress: Element<_> = match self.state.progress() {
            Some(progress) => status_line(progress),
            None => Column::new().into(),
        };

        let dialog: Element<Message> = match &mut self.state {
            State::AskingToSaveModelImage {
                current,
                save_button,
                fail_button,
                ..
            } => Row::new()
                .justify_content(Justify::Center)
                .push(save_model_image_dialog(
                    current.test(),
                    save_button,
                    fail_button,
                ))
                .into(),
            State::ReportingDifferences {
                current,
                show,
                quit_button,
                ..
            } => Row::new()
                .justify_content(Justify::Center)
                .push(differences_dialog(current.test(), *show, quit_button))
                .into(),
            _ => Column::new().into(),
        };

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .spacing(20)
            .justify_content(Justify::SpaceBetween)
            .push(progress)
            .push(dialog)
            .into()
    }
}

// UI elements
fn status_line<'a>(progress: Progress) -> Element<'a, Message> {
    Row::new()
        .justify_content(Justify::SpaceBetween)
        .push(Text::new(&match progress.current {
            Some(test) => format!("Testing {:?}...", test),
            None => String::from("Pending..."),
        }))
        .push(Text::new(&format!(
            "{} tests remaining",
            progress.remaining
        )))
        .into()
}

fn save_model_image_dialog<'a>(
    test: Test,
    save_button: &'a mut button::State,
    fail_button: &'a mut button::State,
) -> Element<'a, Message> {
    let message = Text::new(&format!(
        "No model image exists for the \"{:?}\" test. \
         Create one from the current drawing?",
        test
    ));

    let options = Row::new()
        .spacing(10)
        .push(
            Button::new(fail_button, "No, fail the test.")
                .class(button::Class::Secondary)
                .fill_width()
                .on_press(Message::FailTest),
        )
        .push(
            Button::new(save_button, "Yes, create it.")
                .class(button::Class::Positive)
                .fill_width()
                .on_press(Message::CreateModelImage),
        );

    Panel::new(Column::new().spacing(20).push(message).push(options)).into()
}

fn differences_dialog<'a>(
    test: Test,
    show: bool,
    quit_button: &'a mut button::State,
) -> Element<'a, Message> {
    let message =
        Text::new(&format!("Differences found for the \"{:?}\" test.", test));

    let show_checkbox =
        Checkbox::new(show, "Overlay differences", Message::ToggleDifferences);

    let options = Row::new().spacing(10).push(
        Button::new(quit_button, "Quit")
            .class(button::Class::Secondary)
            .fill_width()
            .on_press(Message::FailTest),
    );

    Panel::new(
        Column::new()
            .spacing(20)
            .push(message)
            .push(show_checkbox)
            .push(options),
    )
    .into()
}
