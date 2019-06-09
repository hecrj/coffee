//! A simple example that demonstrates capturing window and input events.
use std::collections::HashSet;

use coffee::graphics::{
    Color, Font, Frame, Image, Point, Quad, Rectangle, Text, Vector, Window,
    WindowSettings,
};
use coffee::input::{self, Input};
use coffee::load::{loading_screen::ProgressBar, Join, Task};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    InputExample::run(WindowSettings {
        title: String::from("Input - Coffee"),
        size: (720, 240),
        resizable: false,
        fullscreen: false,
    })
}

struct CustomInput {
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<input::KeyCode>,
    mouse_buttons_pressed: HashSet<input::MouseButton>,
    text_buffer: String,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            cursor_position: Point::new(0.0, 0.0),
            mouse_wheel: Point::new(0.0, 0.0),
            keys_pressed: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            text_buffer: String::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::CursorMoved { x, y } => {
                self.cursor_position = Point::new(x, y);
            }
            input::Event::TextInput { character } => {
                self.text_buffer.push(character);
            }
            input::Event::MouseWheel { delta_x, delta_y } => {
                self.mouse_wheel = Point::new(delta_x, delta_y);
            }
            input::Event::KeyboardInput { key_code, state } => match state {
                input::ButtonState::Pressed => {
                    self.keys_pressed.insert(key_code);
                }
                input::ButtonState::Released => {
                    self.keys_pressed.remove(&key_code);
                }
            },
            input::Event::MouseInput { state, button } => match state {
                input::ButtonState::Pressed => {
                    self.mouse_buttons_pressed.insert(button);
                }
                input::ButtonState::Released => {
                    self.mouse_buttons_pressed.remove(&button);
                }
            },
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.text_buffer.clear();
    }
}

struct InputExample {
    palette: Image,
    font: Font,
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<input::KeyCode>,
    mouse_buttons_pressed: HashSet<input::MouseButton>,
    text_buffer: String,
}

impl InputExample {
    const MAX_TEXTSIZE: usize = 40;

    const COLORS: [Color; 1] = [Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    }];

    fn load() -> Task<InputExample> {
        (
            Task::using_gpu(|gpu| Image::from_colors(gpu, &Self::COLORS)),
            Font::load(include_bytes!(
                "../resources/font/Inconsolata-Regular.ttf"
            )),
        )
            .join()
            .map(|(palette, font)| InputExample {
                palette,
                font,
                cursor_position: Point::new(0.0, 0.0),
                mouse_wheel: Point::new(0.0, 0.0),
                keys_pressed: HashSet::new(),
                mouse_buttons_pressed: HashSet::new(),
                text_buffer: String::with_capacity(Self::MAX_TEXTSIZE),
            })
    }
}

impl Game for InputExample {
    type Input = CustomInput;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<InputExample> {
        Task::stage("Loading...", InputExample::load())
    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
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
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        // This closure simplifies some of the boilerplate.
        let add_aligned_text =
            |font: &mut Font, label: &str, content: &str, x: f32, y: f32| {
                font.add(Text {
                    content: label,
                    position: Point::new(x, y),
                    bounds: (frame.width(), frame.height()),
                    size: 20.0,
                    color: Color::WHITE,
                    ..Text::default()
                });
                font.add(Text {
                    content: content,
                    position: Point::new(x + 260.0, y),
                    bounds: (frame.width(), frame.height()),
                    size: 20.0,
                    color: Color::WHITE,
                    ..Text::default()
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

        add_aligned_text(&mut self.font, "Pressed keys:", &keys, 20.0, 20.0);

        add_aligned_text(
            &mut self.font,
            "Text Buffer (type):",
            &self.text_buffer,
            20.0,
            50.0,
        );

        add_aligned_text(
            &mut self.font,
            "Pressed mouse buttons:",
            &mouse_buttons,
            20.0,
            80.0,
        );

        add_aligned_text(
            &mut self.font,
            "Last mouse wheel scroll:",
            &format!("{}, {}", self.mouse_wheel.x, self.mouse_wheel.y),
            20.0,
            110.0,
        );

        self.font.draw(&mut frame.as_target());

        // Draw a small square at the mouse cursor's position.
        self.palette.draw(
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
