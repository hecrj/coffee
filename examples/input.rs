//! A simple example that demonstrates capturing window and input events.
use coffee::{Game, Result, Timer};
use coffee::graphics::{
    Batch, Color, Font, Gpu, Image, Point, Rectangle, Sprite, 
    Text, Window, WindowSettings
};
use coffee::input;
use coffee::load::{loading_screen, Join, LoadingScreen, Task};

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
    pressed_keys: Vec<input::KeyCode>,
    released_keys: Vec<input::KeyCode>,
    pressed_mouse: Vec<input::MouseButton>,
    released_mouse: Vec<input::MouseButton>,
    text_buffer: String,
}

impl Input {
    fn new() -> Input {
        Input {
            cursor_position: Point::new(0.0, 0.0),
            mouse_wheel: Point::new(0.0, 0.0),
            pressed_keys: Vec::new(),
            released_keys: Vec::new(),
            pressed_mouse: Vec::new(),
            released_mouse: Vec::new(),
            text_buffer: String::new(),
        }
    }
}

struct View {
    palette: Image,
    font: Font,
}

impl View {
    const COLORS: [Color; 1] = [
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ];

    fn load() -> Task<View> {
        (
            Task::using_gpu(|gpu| Image::from_colors(gpu, &Self::COLORS)),
            Font::load(include_bytes!(
                "../resources/font/Inconsolata-Regular.ttf"
            )),
        )
            .join()
            .map(|(palette, font)| View {
                palette,
                font,
            })
    }
}

struct InputExample {
    mouse_x: f32,
    mouse_y: f32,
    wheel_x: f32,
    wheel_y: f32,
    text_buffer: String,
    pressed_keys: String,
    released_keys: String,
    pressed_mousebuttons: String,
    released_mousebuttons: String,
}

impl InputExample {

}

impl Game for InputExample {
    type View = View;
    type Input = Input;

    const TICKS_PER_SECOND: u16 = 10;

    fn new(window: &mut Window) -> Result<(InputExample, Self::View, Self::Input)> {
        let task = Task::stage("Loading font...", View::load());

        let mut loading_screen = loading_screen::ProgressBar::new(window.gpu());
        let view = loading_screen.run(task, window)?;

        Ok((InputExample{
                mouse_x: 0.0, 
                mouse_y: 0.0,
                wheel_x: 0.0,
                wheel_y: 0.0,
                text_buffer: String::with_capacity(256),
                pressed_keys: String::new(),
                released_keys: String::new(),
                pressed_mousebuttons: String::new(),
                released_mousebuttons: String::new(),
            }, view, Input::new()))
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
            input::Event::KeyboardInput {
                key_code,
                state
            } => {
                match state {
                    input::ButtonState::Pressed => input.pressed_keys.push(key_code),
                    input::ButtonState::Released => input.released_keys.push(key_code),
                };
            }
            input::Event::MouseInput {
                state,
                button
            } => {
                match state {
                    input::ButtonState::Pressed => input.pressed_mouse.push(button),
                    input::ButtonState::Released => input.released_mouse.push(button),
                };
            }
            _ => {}
        }
    }

    fn update(&mut self, _view: &Self::View, _window: &Window) {
    }

    fn interact(&mut self, input: &mut Input, _view: &mut View, _gpu: &mut Gpu) {
        self.mouse_x = input.cursor_position.x;
        self.mouse_y = input.cursor_position.y;

        self.wheel_x = input.mouse_wheel.x;
        self.wheel_y = input.mouse_wheel.y;
        input.mouse_wheel.x = 0.0;
        input.mouse_wheel.y = 0.0;

        if !input.text_buffer.is_empty() {
            for c in input.text_buffer.chars() {
                match c {
                    '\u{0008}' => {
                        self.text_buffer.pop();
                    }
                    _ => {
                        if self.text_buffer.chars().count() < 30 {
                            self.text_buffer.push_str(&input.text_buffer);
                        }
                    }
                }
            }
            input.text_buffer.clear();
        }

        self.pressed_keys = input.pressed_keys.iter()
                                 .map(|k| format!("{:?}", k))
                                 .collect::<Vec<_>>()
                                 .join(", ");

        self.released_keys = input.released_keys.iter()
                                 .map(|k| format!("{:?}", k))
                                 .collect::<Vec<_>>()
                                 .join(", ");

        input.pressed_keys.clear();
        input.released_keys.clear();

        self.pressed_mousebuttons = input.pressed_mouse.iter()
                                 .map(|k| format!("{:?}", k))
                                 .collect::<Vec<_>>()
                                 .join(", ");

        self.released_mousebuttons = input.released_mouse.iter()
                                 .map(|k| format!("{:?}", k))
                                 .collect::<Vec<_>>()
                                 .join(", ");

        input.pressed_mouse.clear();
        input.released_mouse.clear();
    }

    fn draw(&self, view: &mut Self::View, window: &mut Window, _timer: &Timer) {
        let mut frame = window.frame();
        frame.clear(Color::BLACK);

        view.font.add(Text {
            content: String::from("Text Buffer (type):"),
            position: Point::new(20.0, frame.height() - 40.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: self.text_buffer.clone(),
            position: Point::new(280.0, frame.height() - 40.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: String::from("Pressed keys:"),
            position: Point::new(20.0, 20.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: self.pressed_keys.clone(),
            position: Point::new(280.0, 20.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::from_rgb(0, 255, 0),
        });

        view.font.add(Text {
            content: String::from("Released keys:"),
            position: Point::new(20.0, 50.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: self.released_keys.clone(),
            position: Point::new(280.0, 50.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::from_rgb(255, 0, 0),
        });

        view.font.add(Text {
            content: String::from("Mouse wheel scroll:"),
            position: Point::new(20.0, 80.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: format!("{}, {}", self.wheel_x, self.wheel_y),
            position: Point::new(280.0, 80.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: String::from("Pressed mouse buttons:"),
            position: Point::new(20.0, 110.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: self.pressed_mousebuttons.clone(),
            position: Point::new(280.0, 110.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::from_rgb(0, 255, 0),
        });

        view.font.add(Text {
            content: String::from("Released mouse buttons:"),
            position: Point::new(20.0, 140.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: self.released_mousebuttons.clone(),
            position: Point::new(280.0, 140.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::from_rgb(255, 0, 0),
        });

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
            position: Point::new(self.mouse_x - 3.0, self.mouse_y - 3.0)
        });
        batch.draw(Point::new(0.0, 0.0), &mut frame.as_target());
    }
}