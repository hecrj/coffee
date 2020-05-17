//! An example that showcases gamepad events
use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::input::{self, gamepad, Input};
use coffee::load::Task;
use coffee::ui::{
    Align, Column, Element, Justify, Renderer, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <GamepadExample as UserInterface>::run(WindowSettings {
        title: String::from("Gamepad - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Gamepad {
    last_event: Option<gamepad::Event>,
}

impl Input for Gamepad {
    fn new() -> Gamepad {
        Gamepad { last_event: None }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Gamepad { event, .. } => {
                self.last_event = Some(event);
            }
            _ => {}
        }
    }

    fn clear(&mut self) {}
}

struct GamepadExample {
    last_event: String,
}

impl Game for GamepadExample {
    type Input = Gamepad;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<GamepadExample> {
        Task::succeed(|| GamepadExample {
            last_event: "None".to_string(),
        })
    }

    fn interact(&mut self, gamepad: &mut Gamepad, _window: &mut Window) {
        if let Some(event) = gamepad.last_event {
            self.last_event = format!("{:#?}", event);
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
    }
}

impl UserInterface for GamepadExample {
    type Message = ();
    type Renderer = Renderer;

    fn react(&mut self, _msg: (), _window: &mut Window) {}

    fn layout(&mut self, window: &Window) -> Element<()> {
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .push(
                Column::new()
                    .max_width(500)
                    .spacing(20)
                    .push(Text::new("Last gamepad event:").size(30))
                    .push(Text::new(&self.last_event)),
            )
            .into()
    }
}
