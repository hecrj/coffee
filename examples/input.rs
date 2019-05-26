//! A simple example that demonstrates capturing window and input events.
use std::collections::HashSet;

use coffee::graphics::{
    Color, Font, Frame, Image, Point, Quad, Rectangle, Text, Vector, Window,
    WindowSettings,
};
use coffee::input;
use coffee::load::{loading_screen, Join, LoadingScreen, Task};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    InputExample::run(WindowSettings {
        title: String::from("Input - Coffee"),
        size: (720, 240),
        resizable: false,
        fullscreen: false,
    })
}

struct Input {
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<input::KeyCode>,
    mouse_buttons_pressed: HashSet<input::MouseButton>,
    text_buffer: String,
}

impl Input {
    fn new() -> Input {
        Input {
            cursor_position: Point::new(0.0, 0.0),
            mouse_wheel: Point::new(0.0, 0.0),
            keys_pressed: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            text_buffer: String::new(),
        }
    }
}

struct View {
    palette: Image,
    font: Font,
}

impl View {
    const COLORS: [Color; 1] = [Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    }];

    fn load() -> Task<View> {
        (
            Task::using_gpu(|gpu| Image::from_colors(gpu, &Self::COLORS)),
            Font::load(include_bytes!(
                "../resources/font/Inconsolata-Regular.ttf"
            )),
        )
            .join()
            .map(|(palette, font)| View { palette, font })
    }
}

struct InputExample {
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<input::KeyCode>,
    mouse_buttons_pressed: HashSet<input::MouseButton>,
    text_buffer: String,
}

impl InputExample {
    const MAX_TEXTSIZE: usize = 40;
}

impl Game for InputExample {
    type View = View;
    type Input = Input;

    const TICKS_PER_SECOND: u16 = 10;

    fn new(
        window: &mut Window,
    ) -> Result<(InputExample, Self::View, Self::Input)> {
        let task = Task::stage("Loading font...", View::load());

        let mut loading_screen = loading_screen::ProgressBar::new(window.gpu());
        let view = loading_screen.run(task, window)?;

        Ok((
            InputExample {
                cursor_position: Point::new(0.0, 0.0),
                mouse_wheel: Point::new(0.0, 0.0),
                keys_pressed: HashSet::new(),
                mouse_buttons_pressed: HashSet::new(),
                text_buffer: String::with_capacity(Self::MAX_TEXTSIZE),
            },
            view,
            Input::new(),
        ))
    }

    fn on_input(&self, input: &mut Input, event: input::Event) {
        match event {
            input::Event::CursorMoved { x, y } => {
                input.cursor_position = Point::new(x, y);
            }
            input::Event::TextInput { character } => {
                input.text_buffer.push(character);
            }
            input::Event::MouseWheel { delta_x, delta_y } => {
                input.mouse_wheel = Point::new(delta_x, delta_y);
            }
            input::Event::KeyboardInput { key_code, state } => match state {
                input::ButtonState::Pressed => {
                    input.keys_pressed.insert(key_code);
                }
                input::ButtonState::Released => {
                    input.keys_pressed.remove(&key_code);
                }
            },
            input::Event::MouseInput { state, button } => match state {
                input::ButtonState::Pressed => {
                    input.mouse_buttons_pressed.insert(button);
                }
                input::ButtonState::Released => {
                    input.mouse_buttons_pressed.remove(&button);
                }
            },
            _ => {}
        }
    }

    fn update(&mut self, _view: &Self::View, _window: &Window) {}

    fn interact(
        &mut self,
        input: &mut Input,
        _view: &mut View,
        _window: &mut Window,
    ) {
        self.cursor_position = input.cursor_position;
        self.mouse_wheel = input.mouse_wheel;
        self.keys_pressed = input.keys_pressed.clone();
        self.mouse_buttons_pressed = input.mouse_buttons_pressed.clone();

        if !input.text_buffer.is_empty() {
            for c in input.text_buffer.chars() {
                match c {
                    // Match ASCII backspace and delete from the text buffer
                    '\u{0008}' => {
                        self.text_buffer.pop();
                    }
                    _ => {
                        if self.text_buffer.chars().count() < Self::MAX_TEXTSIZE
                        {
                            self.text_buffer.push_str(&input.text_buffer);
                        }
                    }
                }
            }
            input.text_buffer.clear();
        }
    }

    fn draw(&self, view: &mut Self::View, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        // This closure simplifies some of the boilerplate.
        let mut add_aligned_text =
            |label: String, content: String, x: f32, y: f32| {
                view.font.add(Text {
                    content: label,
                    position: Point::new(x, y),
                    bounds: (frame.width(), frame.height()),
                    size: 20.0,
                    color: Color::WHITE,
                });
                view.font.add(Text {
                    content: content,
                    position: Point::new(x + 260.0, y),
                    bounds: (frame.width(), frame.height()),
                    size: 20.0,
                    color: Color::WHITE,
                });
            };

        let keys = self
            .keys_pressed
            .iter()
            .map(|key| format!("{:?}", key))
            .collect::<Vec<_>>()
            .join(", ");

        let mouse_buttons = self
            .mouse_buttons_pressed
            .iter()
            .map(|button| format!("{:?}", button))
            .collect::<Vec<_>>()
            .join(", ");

        add_aligned_text(String::from("Pressed keys:"), keys, 20.0, 20.0);

        add_aligned_text(
            String::from("Text Buffer (type):"),
            self.text_buffer.clone(),
            20.0,
            50.0,
        );

        add_aligned_text(
            String::from("Pressed mouse buttons:"),
            mouse_buttons,
            20.0,
            80.0,
        );

        add_aligned_text(
            String::from("Last mouse wheel scroll:"),
            format!("{}, {}", self.mouse_wheel.x, self.mouse_wheel.y),
            20.0,
            110.0,
        );

        view.font.draw(&mut frame.as_target());

        // Draw a small square at the mouse cursor's position.
        view.palette.draw(
            Quad {
                source: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                },
                position: self.cursor_position - Vector::new(3.0, 3.0),
                size: (6.0, 6.0),
            },
            &mut frame.as_target(),
        );
    }
}
