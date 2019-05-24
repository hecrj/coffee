use coffee::graphics::{Color, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    button, renderer, Button, Checkbox, Column, Element, Radio, Root, Row,
    Text, UserInterface,
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

        let content = Column::new()
            .max_width(500.0)
            .spacing(20)
            .push(steps.current().layout().map(Event::StepEvent))
            .push(controls);

        Root::new(
            Column::new()
                .width(window.width())
                .height(window.height())
                .center_children()
                .padding(20)
                .push(content),
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
            Event::StepEvent(step_event) => {
                self.steps.update(step_event);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    BackPressed,
    NextPressed,
    StepEvent(StepEvent),
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
                Step::Buttons {
                    primary: button::State::new(),
                    secondary: button::State::new(),
                    positive: button::State::new(),
                },
                Step::Checkbox { is_checked: false },
                Step::Radio { selection: None },
                Step::Text,
                Step::RowsAndColumns,
            ],
            current: 0,
        }
    }

    fn update(&mut self, event: StepEvent) {
        match event {
            StepEvent::CheckboxToggled(value) => {
                if let Step::Checkbox { is_checked } = self.current() {
                    *is_checked = value;
                }
            }
            StepEvent::LanguageSelected(language) => {
                if let Step::Radio { selection } = self.current() {
                    *selection = Some(language);
                }
            }
        };
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

    fn current(&mut self) -> &mut Step {
        &mut self.steps[self.current as usize]
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
    Buttons {
        primary: button::State,
        secondary: button::State,
        positive: button::State,
    },
    Checkbox {
        is_checked: bool,
    },
    Radio {
        selection: Option<Language>,
    },
    Text,
    RowsAndColumns,
}

#[derive(Debug, Clone, Copy)]
enum StepEvent {
    CheckboxToggled(bool),
    LanguageSelected(Language),
}

impl<'a> Step {
    fn layout(
        &mut self,
    ) -> Element<StepEvent, <Tour as UserInterface>::Renderer> {
        match self {
            Step::Welcome => Self::welcome().into(),
            Step::Buttons {
                primary,
                secondary,
                positive,
            } => Self::buttons(primary, secondary, positive).into(),
            Step::Checkbox { is_checked } => Self::checkbox(*is_checked).into(),
            Step::Radio { selection } => Self::radio(*selection).into(),
            Step::Text => Self::text().into(),
            Step::RowsAndColumns => Self::rows_and_columns().into(),
        }
    }

    fn container(
        title: &str,
    ) -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn welcome() -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        Self::container("Welcome!")
            .push(Text::new(
                "This is a tour that introduces some of the features and \
                 concepts related with UI development in Coffee.",
            ))
            .push(Text::new(
                "We will start by taking a look at the interactive widgets \
                 that are built into Coffee.",
            ))
            .push(Text::new(
                "Then, we will learn about the layout engine and how to build \
                 responsive UIs.",
            ))
            .push(Text::new(
                "Press the \"Next\" button whenever you are ready!",
            ))
    }

    fn buttons(
        primary: &'a mut button::State,
        secondary: &'a mut button::State,
        positive: &'a mut button::State,
    ) -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        Self::container("Buttons")
            .push(Text::new("Buttons can fire actions when clicked."))
            .push(Text::new(
                "As of now, there are 3 different types of buttons: \
                 primary, secondary, and positive.",
            ))
            .push(Button::new(primary, "Primary"))
            .push(
                Button::new(secondary, "Secondary")
                    .r#type(button::Type::Secondary),
            )
            .push(
                Button::new(positive, "Positive")
                    .r#type(button::Type::Positive),
            )
            .push(Text::new(
                "More types of buttons will probably be supported in the near \
                 future! Choose each type smartly depending on the situation.",
            ))
    }

    fn checkbox(
        is_checked: bool,
    ) -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        Self::container("Checkbox").push(Checkbox::new(
            is_checked,
            "Some checkbox",
            StepEvent::CheckboxToggled,
        ))
    }

    fn radio(
        selection: Option<Language>,
    ) -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        let container = Self::container("Radio")
            .push(Text::new("Which is your favorite programming language?"));

        Language::all().iter().cloned().fold(
            container,
            |container, language| {
                container.push(Radio::new(
                    language,
                    language.into(),
                    selection,
                    StepEvent::LanguageSelected,
                ))
            },
        )
    }

    fn text() -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        Self::container("Text")
            .push(Text::new(
                "Text is probably the most essential widget for your UI. \
                 It will automatically adapt to the width of its \
                 container.",
            ))
            .push(Text::new("You can change its size and color:"))
            .push(Text::new("This text is 30 points").size(30))
            .push(Text::new("This text is 40 points and cyan").size(40).color(
                Color {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            ))
    }

    fn rows_and_columns(
    ) -> Column<'a, StepEvent, <Tour as UserInterface>::Renderer> {
        Self::container("Rows and Columns")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
}

impl Language {
    fn all() -> [Language; 5] {
        [
            Language::Rust,
            Language::Elm,
            Language::Ruby,
            Language::Haskell,
            Language::C,
        ]
    }
}

impl From<Language> for &str {
    fn from(language: Language) -> &'static str {
        match language {
            Language::Rust => "Rust",
            Language::Elm => "Elm",
            Language::Ruby => "Ruby",
            Language::Haskell => "Haskell",
            Language::C => "C",
        }
    }
}
