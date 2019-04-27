//! A simple example that demonstrates capturing window and input events.
use coffee::graphics::{
    Batch, Color, Font, Gpu, Image, Point, Rectangle, Sprite, Text, Window,
    WindowSettings,
};
use coffee::input;
use coffee::load::{loading_screen, Join, LoadingScreen, Task};
use coffee::{Game, Result, Timer};

use std::collections::HashMap;

fn main() -> Result<()> {
    InputExample::run(WindowSettings {
        title: String::from("Input Example - Coffee"),
        size: (720, 240),
        resizable: false,
    })
}

struct Input {
    cursor_position: Point,
    mouse_wheel: Point,
    key_state: HashMap<input::KeyCode, bool>,
    mouse_state: HashMap<input::MouseButton, bool>,
    text_buffer: String,
}

impl Input {
    fn new() -> Input {
        Input {
            cursor_position: Point::new(0.0, 0.0),
            mouse_wheel: Point::new(0.0, 0.0),
            key_state: HashMap::new(),
            mouse_state: HashMap::new(),
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
    mouse_x: f32,
    mouse_y: f32,
    wheel_x: f32,
    wheel_y: f32,
    text_buffer: String,
    keys_down: String,
    mousebuttons_down: String,
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
                mouse_x: 0.0,
                mouse_y: 0.0,
                wheel_x: 0.0,
                wheel_y: 0.0,
                text_buffer: String::with_capacity(Self::MAX_TEXTSIZE),
                keys_down: String::new(),
                mousebuttons_down: String::new(),
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
            input::Event::KeyboardInput { key_code, state } => {
                input.key_state.insert(
                    key_code,
                    match state {
                        input::ButtonState::Pressed => true,
                        input::ButtonState::Released => false,
                    },
                );
            }
            input::Event::MouseInput { state, button } => {
                input.mouse_state.insert(
                    button,
                    match state {
                        input::ButtonState::Pressed => true,
                        input::ButtonState::Released => false,
                    },
                );
            }
            _ => {}
        }
    }

    fn update(&mut self, _view: &Self::View, _window: &Window) {}

    fn interact(
        &mut self,
        input: &mut Input,
        _view: &mut View,
        _gpu: &mut Gpu,
    ) {
        self.mouse_x = input.cursor_position.x;
        self.mouse_y = input.cursor_position.y;

        self.wheel_x = input.mouse_wheel.x;
        self.wheel_y = input.mouse_wheel.y;

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

        self.keys_down = input
            .key_state
            .iter()
            .filter(|(_, &v)| v == true)
            .map(|(k, _)| format!("{:?}", k))
            .collect::<Vec<_>>()
            .join(", ");

        self.mousebuttons_down = input
            .mouse_state
            .iter()
            .filter(|(_, &v)| v == true)
            .map(|(k, _)| format!("{:?}", k))
            .collect::<Vec<_>>()
            .join(", ");
    }

    fn draw(&self, view: &mut Self::View, window: &mut Window, _timer: &Timer) {
        let mut frame = window.frame();
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

        add_aligned_text(
            String::from("Pressed keys:"),
            self.keys_down.clone(),
            20.0,
            20.0,
        );

        add_aligned_text(
            String::from("Text Buffer (type):"),
            self.text_buffer.clone(),
            20.0,
            50.0,
        );

        add_aligned_text(
            String::from("Pressed mouse buttons:"),
            self.mousebuttons_down.clone(),
            20.0,
            80.0,
        );

        add_aligned_text(
            String::from("Last mouse wheel scroll:"),
            format!("{}, {}", self.wheel_x, self.wheel_y),
            20.0,
            110.0,
        );

        view.font.draw(&mut frame);

        let mut batch = Batch::new(view.palette.clone());
        // Draw a small square at the mouse cursor's position.
        batch.add(Sprite {
            source: Rectangle {
                x: 0,
                y: 0,
                width: 6,
                height: 6,
            },
            position: Point::new(self.mouse_x - 3.0, self.mouse_y - 3.0),
        });
        batch.draw(Point::new(0.0, 0.0), &mut frame.as_target());
    }
}
