//! A simple example that demonstrates capturing window and input events.
use std::collections::HashSet;

use coffee::graphics::{
    Color, Frame, Image, Point, Rectangle, Sprite, Vector, Window,
    WindowSettings,
};
use coffee::input::{self, keyboard, mouse, Input};
use coffee::load::Task;
use coffee::ui::{
    Align, Column, Element, Justify, Renderer, Row, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <InputExample as UserInterface>::run(WindowSettings {
        title: String::from("Input - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct CustomInput {
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<keyboard::KeyCode>,
    mouse_buttons_pressed: HashSet<mouse::Button>,
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
            input::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { x, y } => {
                    self.cursor_position = Point::new(x, y);
                }
                mouse::Event::Input { state, button } => match state {
                    input::ButtonState::Pressed => {
                        self.mouse_buttons_pressed.insert(button);
                    }
                    input::ButtonState::Released => {
                        self.mouse_buttons_pressed.remove(&button);
                    }
                },
                mouse::Event::WheelScrolled { delta_x, delta_y } => {
                    self.mouse_wheel = Point::new(delta_x, delta_y);
                }
                _ => {}
            },
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::TextEntered { character } => {
                    self.text_buffer.push(character);
                }
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.insert(key_code);
                    }
                    input::ButtonState::Released => {
                        self.keys_pressed.remove(&key_code);
                    }
                },
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
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<keyboard::KeyCode>,
    mouse_buttons_pressed: HashSet<mouse::Button>,
    text_buffer: String,
}

impl InputExample {
    const MAX_TEXTSIZE: usize = 40;
}

impl Game for InputExample {
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<InputExample> {
        Task::using_gpu(|gpu| Image::from_colors(gpu, &[Color::BLACK])).map(
            |palette| InputExample {
                palette,
                cursor_position: Point::new(0.0, 0.0),
                mouse_wheel: Point::new(0.0, 0.0),
                keys_pressed: HashSet::new(),
                mouse_buttons_pressed: HashSet::new(),
                text_buffer: String::with_capacity(Self::MAX_TEXTSIZE),
            },
        )
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
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });

        // Draw a small square at the mouse cursor's position.
        self.palette.draw(
            Sprite {
                source: Rectangle {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
                position: self.cursor_position - Vector::new(3.0, 3.0),
                scale: (6.0, 6.0),
            },
            &mut frame.as_target(),
        );
    }
}

impl UserInterface for InputExample {
    type Message = ();
    type Renderer = Renderer;

    fn react(&mut self, _msg: (), _window: &mut Window) {}

    fn layout(&mut self, window: &Window) -> Element<()> {
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

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(label_and_value("Pressed keys:", &keys))
            .push(label_and_value("Text buffer (type):", &self.text_buffer))
            .push(label_and_value("Pressed mouse buttons:", &mouse_buttons))
            .push(label_and_value(
                "Last mouse wheel scroll:",
                &format!("{}, {}", self.mouse_wheel.x, self.mouse_wheel.y),
            ));

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .push(content)
            .into()
    }
}

fn label_and_value(label: &str, value: &str) -> Element<'static, ()> {
    Row::new()
        .spacing(20)
        .align_items(Align::Stretch)
        .push(Text::new(label))
        .push(Text::new(value))
        .into()
}
