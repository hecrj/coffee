use coffee::graphics::{Color, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    button, renderer, Button, Column, Panel, Root, Row, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <Tour as UserInterface>::run(WindowSettings {
        title: String::from("User Interface - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
}

struct Tour {
    steps: Steps,
    back_button: button::State,
    next_button: button::State,
}

impl Game for Tour {
    type Input = KeyboardAndMouse;
    type State = ();
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Tour> {
        Task::new(|| Tour {
            steps: Steps::new(),
            back_button: button::State::new(),
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

impl UserInterface for Tour {
    type Event = Event;
    type Renderer = renderer::Basic;

    fn layout(
        &mut self,
        _state: &Self::State,
        window: &Window,
    ) -> Root<Event, Self::Renderer> {
        let Tour {
            steps,
            back_button,
            next_button,
        } = self;

        let mut controls = Row::new();

        if steps.has_started() {
            controls = controls.push(
                Button::new(back_button, "Back")
                    .on_click(Event::BackPressed)
                    .r#type(button::Type::Secondary),
            );
        }

        controls = controls.push(Column::new().fill_width());

        if !steps.has_finished() {
            controls = controls.push(
                Button::new(next_button, "Next").on_click(Event::NextPressed),
            );
        }

        let content = steps.current().layout();

        Root::new(
            Column::new()
                .width(window.width())
                .height(window.height())
                .center_children()
                .padding(20)
                .push(content.push(controls)),
        )
    }

    fn update(&mut self, _state: &mut Self::State, event: Event) {
        match event {
            Event::BackPressed => {
                self.steps.go_back();
            }
            Event::NextPressed => {
                self.steps.advance();
            }
        }
    }
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Buttons,
                Step::Text,
                Step::RowsAndColumns,
            ],
            current: 0,
        }
    }

    fn current(&mut self) -> &mut Step {
        &mut self.steps[self.current as usize]
    }

    fn advance(&mut self) {
        if !self.has_finished() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_started() {
            self.current -= 1;
        }
    }

    fn has_started(&self) -> bool {
        self.current > 0
    }

    fn has_finished(&self) -> bool {
        self.current + 1 >= self.steps.len()
    }
}

enum Step {
    Welcome,
    Buttons,
    Text,
    RowsAndColumns,
}

impl Step {
    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome!",
            Step::Buttons => "Buttons",
            Step::Text => "Text",
            Step::RowsAndColumns => "Rows and Columns",
        }
    }

    fn layout(&mut self) -> Column<Event, <Tour as UserInterface>::Renderer> {
        match self {
            Step::Welcome => self.welcome(),
            Step::Buttons => self.buttons(),
            Step::Text => self.text(),
            Step::RowsAndColumns => self.rows_and_columns(),
        }
    }

    fn container(&self) -> Column<Event, <Tour as UserInterface>::Renderer> {
        Column::new()
            .max_width(500.0)
            .spacing(20)
            .push(Text::new(self.title()).size(50.0))
    }

    fn welcome(&self) -> Column<Event, <Tour as UserInterface>::Renderer> {
        self.container()
            .push(Text::new(
                "This example introduces some of the different UI \
                 widgets that are built into Coffee.",
            ))
            .push(Text::new(
                "Press the \"Next\" button whenever you are ready!",
            ))
    }

    fn buttons(&self) -> Column<Event, <Tour as UserInterface>::Renderer> {
        self.container()
    }

    fn text(&self) -> Column<Event, <Tour as UserInterface>::Renderer> {
        self.container()
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
            )
    }

    fn rows_and_columns(
        &self,
    ) -> Column<Event, <Tour as UserInterface>::Renderer> {
        self.container()
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
                    .push(Text::new("This text will be shown on the left side"))
                    .push(Text::new("This is the right side")),
            )
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    BackPressed,
    NextPressed,
}
