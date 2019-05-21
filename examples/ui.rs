use coffee::graphics::{Color, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    button, renderer, Button, Column, Panel, Root, Row, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <Example as UserInterface>::run(WindowSettings {
        title: String::from("User Interface - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
}

struct Example {
    steps: Vec<Step>,
    current_step: u16,
    next_button: button::State,
}

impl Game for Example {
    type Input = KeyboardAndMouse;
    type State = ();
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Example> {
        Task::new(|| Example {
            steps: Step::all(),
            current_step: 0,
            next_button: button::State::new(),
        })
    }

    fn draw(
        &mut self,
        _state: &Self::State,
        window: &mut Window,
        _timer: &Timer,
    ) {
        let mut frame = window.frame();
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
    }
}

impl UserInterface for Example {
    type Event = Event;
    type Renderer = renderer::Basic;

    fn layout(
        &mut self,
        _state: &Self::State,
        window: &Window,
    ) -> Root<Event, Self::Renderer> {
        let content = match self.step() {
            Step::Welcome => Column::new()
                .max_width(500.0)
                .spacing(20)
                .push(Text::new("Welcome!").size(50.0))
                .push(Text::new(
                    "This example introduces some of the different UI \
                     widgets that are built into Coffee.",
                ))
                .push(Text::new(
                    "Press the \"Next\" button whenever you are ready!",
                )),
            Step::Text => Column::new()
                .max_width(500.0)
                .spacing(20)
                .push(Text::new("Text").size(50.0))
                .push(Text::new(
                    "Text is probably the most essential widget for your UI. \
                     It will automatically adapt to the width of its \
                     container.",
                ))
                .push(Text::new("You can change its size and color:"))
                .push(Text::new("This text is 30.0 points").size(30.0))
                .push(
                    Text::new("This text is 40.0 points and cyan")
                        .size(40.0)
                        .color(Color {
                            r: 0.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                ),
            Step::RowsAndColumns => Column::new()
                .max_width(500.0)
                .spacing(20)
                .push(Text::new("Rows and Columns").size(50.0))
                .push(Text::new(
                    "Rows and columns can be used to distribute content \
                     horizontally or vertically, respectively.",
                ))
                .push(Text::new(
                    "All the text you have been reading until now was inside \
                     a column. Here is a row:",
                ))
                .push(
                    Row::new()
                        .spacing(20)
                        .push(Text::new(
                            "This text will be shown on the left side",
                        ))
                        .push(Text::new("This is the right side")),
                ),
        };

        Root::new(
            Column::new()
                .width(window.width())
                .height(window.height())
                .center_children()
                .padding(20)
                .push(if self.is_end_reached() {
                    content
                } else {
                    content.push(
                        Button::new(&mut self.next_button, "Next")
                            .align_right()
                            .on_click(Event::NextPressed),
                    )
                }),
        )
    }

    fn update(&mut self, _state: &mut Self::State, event: Event) {
        match event {
            Event::NextPressed => {
                self.advance();
            }
        }
    }
}

impl Example {
    fn step(&mut self) -> &mut Step {
        &mut self.steps[self.current_step as usize]
    }

    fn is_end_reached(&self) -> bool {
        self.current_step + 1 >= self.steps.len() as u16
    }

    fn advance(&mut self) {
        if !self.is_end_reached() {
            self.current_step += 1;
        }
    }
}

enum Step {
    Welcome,
    Text,
    RowsAndColumns,
}

impl Step {
    fn all() -> Vec<Step> {
        vec![Step::Welcome, Step::Text, Step::RowsAndColumns]
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    NextPressed,
}
