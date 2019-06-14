//! An example that showcases gamepad events
use coffee::graphics::{
    Color, Font, Frame, Point, Text, Window, WindowSettings,
};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    GamepadExample::run(WindowSettings {
        title: String::from("Gamepad - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

struct GamepadExample {
    font: Font,
    last_event: String,
}

impl Game for GamepadExample {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<GamepadExample> {
        Font::load(include_bytes!("../resources/font/Inconsolata-Regular.ttf"))
            .map(|font| GamepadExample {
                font,
                last_event: "None".to_string(),
            })
    }

    fn interact(&mut self, _input: &mut (), _window: &mut Window) {
        //if let Some(Event::GamepadEvent { event, .. }) = input.last_event {
        //    self.last_event = format!("{:#?}", event);
        //}
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        // Draw simple text UI
        self.font.add(Text {
            content: "Last Gamepad Event:",
            position: Point::new(10.0, frame.height() - 250.0),
            size: 20.0,
            color: Color::WHITE,
            ..Text::default()
        });

        self.font.add(Text {
            content: &self.last_event.clone(),
            position: Point::new(10.0, frame.height() - 225.0),
            size: 16.0,
            color: Color::WHITE,
            ..Text::default()
        });

        self.font.draw(&mut frame.as_target());
    }
}
