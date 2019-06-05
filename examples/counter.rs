use coffee::graphics::{
    Color, Frame, HorizontalAlignment, VerticalAlignment, Window,
    WindowSettings,
};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    button, Align, Button, Column, Element, Justify, Renderer, Text,
};
use coffee::{Game, Result, Timer, UserInterface};

pub fn main() -> Result<()> {
    <Counter as UserInterface>::run(WindowSettings {
        title: String::from("Counter - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl Game for Counter {
    type State = ();
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Counter> {
        Task::new(|| Counter {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        })
    }

    fn draw(&mut self, _state: &(), frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
    }
}

#[derive(Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl UserInterface for Counter {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, _state: &mut (), message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn layout(&mut self, _state: &(), window: &Window) -> Element<Message> {
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .spacing(20)
            .push(
                Button::new(&mut self.increment_button, "+")
                    .on_click(Message::IncrementPressed),
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
                    .on_click(Message::DecrementPressed),
            )
            .into()
    }
}
