use coffee::graphics::{
    Color, Frame, HorizontalAlignment, VerticalAlignment, Window,
    WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{
    button, Align, Button, Column, Element, Justify, Renderer, Text,
    UserInterface,
};
use coffee::{Game, Result, Timer};

pub fn main() -> Result<()> {
    <Counter as UserInterface>::run(WindowSettings {
        title: String::from("Counter - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl Game for Counter {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Counter> {
        Task::succeed(|| Counter {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        })
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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl UserInterface for Counter {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, message: Message, _window: &mut Window) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .spacing(20)
            .push(
                Button::new(&mut self.increment_button, "+")
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Text::new(&self.value.to_string())
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(
                Button::new(&mut self.decrement_button, "-")
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
