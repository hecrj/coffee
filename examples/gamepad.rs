//! An example that showcases gamepad events
use coffee::graphics::{Color, Font, Gpu, Point, Text, Window, WindowSettings};
use coffee::load::{loading_screen, Join, LoadingScreen, Task};
use coffee::{input, input::Event};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    GamepadExample::run(WindowSettings {
        title: String::from("Gamepad - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
}

struct GamepadExample {}

impl GamepadExample {
    fn load() -> Task<GamepadExample> {
        Task::new(move || GamepadExample {})
    }
}

impl Game for GamepadExample {
    type View = View;
    type Input = Input;

    const TICKS_PER_SECOND: u16 = 20;

    fn new(window: &mut Window) -> Result<(GamepadExample, View, Input)> {
        let task = (
            Task::stage("Setting up...", GamepadExample::load()),
            Task::stage("Loading assets...", View::load()),
        )
            .join();

        let mut loading_screen = loading_screen::ProgressBar::new(window.gpu());
        let (gamepad, view) = loading_screen.run(task, window)?;

        Ok((gamepad, view, Input::new()))
    }

    fn on_input(&self, input: &mut Input, event: input::Event) {
        match event {
            input::Event::GamepadEvent { id, event, time } => {
                input.last_event = Some(coffee::input::Event::GamepadEvent {
                    id,
                    event,
                    time,
                });
            }
            _ => {}
        }
    }

    fn interact(&mut self, input: &mut Input, view: &mut View, _gpu: &mut Gpu) {
        if let Some(Event::GamepadEvent { event, .. }) = input.last_event {
            view.last_event = format!("{:#?}", event);
        }
    }

    fn update(&mut self, _view: &View, _window: &Window) {}

    fn draw(&self, view: &mut View, window: &mut Window, _timer: &Timer) {
        let mut frame = window.frame();
        frame.clear(Color::BLACK);

        // Draw simple text UI
        view.font.add(Text {
            content: String::from("Last Gamepad Event:"),
            position: Point::new(10.0, frame.height() - 250.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: view.last_event.clone(),
            position: Point::new(10.0, frame.height() - 225.0),
            bounds: (frame.width(), frame.height()),
            size: 16.0,
            color: Color::WHITE,
        });

        view.font.draw(&mut frame);
    }
}

struct Input {
    last_event: Option<coffee::input::Event>,
}

impl Input {
    fn new() -> Input {
        Input { last_event: None }
    }
}

struct View {
    font: Font,
    last_event: String,
}

impl View {
    fn load() -> Task<View> {
        Font::load(include_bytes!("../resources/font/Inconsolata-Regular.ttf"))
            .map(|font| View {
                font,
                last_event: "None".to_string(),
            })
    }
}
