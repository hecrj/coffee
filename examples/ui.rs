use coffee::graphics::{
    Color, Frame, HorizontalAlignment, Window, WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{
    button, slider, Align, Button, Checkbox, Column, Element, Justify, Radio,
    Renderer, Row, Slider, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <Tour as UserInterface>::run(WindowSettings {
        title: String::from("User Interface - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Tour {
    steps: Steps,
    back_button: button::State,
    next_button: button::State,
}

impl Game for Tour {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Tour> {
        Task::succeed(|| Tour {
            steps: Steps::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
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

impl UserInterface for Tour {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, event: Message, _window: &mut Window) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        let Tour {
            steps,
            back_button,
            next_button,
        } = self;

        let mut controls = Row::new();

        if steps.has_previous() {
            controls = controls.push(
                Button::new(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .class(button::Class::Secondary),
            );
        }

        controls = controls.push(Column::new());

        if steps.can_continue() {
            controls = controls.push(
                Button::new(next_button, "Next").on_press(Message::NextPressed),
            );
        }

        let content = Column::new()
            .max_width(500)
            .spacing(20)
            .push(steps.layout().map(Message::StepMessage))
            .push(controls);

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

#[derive(Debug, Clone, Copy)]
enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
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
                Step::Slider {
                    state: slider::State::new(),
                    value: 50,
                },
                Step::Text {
                    size_slider: slider::State::new(),
                    size: 30,
                    color_sliders: [slider::State::new(); 3],
                    color: Color::BLACK,
                },
                Step::RowsAndColumns {
                    layout: Layout::Row,
                    spacing_slider: slider::State::new(),
                    spacing: 20,
                },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn layout(&mut self) -> Element<StepMessage> {
        self.steps[self.current].layout()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
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
    Slider {
        state: slider::State,
        value: u16,
    },
    Text {
        size_slider: slider::State,
        size: u16,
        color_sliders: [slider::State; 3],
        color: Color,
    },
    RowsAndColumns {
        layout: Layout,
        spacing_slider: slider::State,
        spacing: u16,
    },
    End,
}

#[derive(Debug, Clone, Copy)]
enum StepMessage {
    CheckboxToggled(bool),
    LanguageSelected(Language),
    SliderChanged(f32),
    TextSizeChanged(f32),
    TextColorChanged(Color),
    LayoutChanged(Layout),
    SpacingChanged(f32),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::CheckboxToggled(value) => {
                if let Step::Checkbox { is_checked } = self {
                    *is_checked = value;
                }
            }
            StepMessage::LanguageSelected(language) => {
                if let Step::Radio { selection } = self {
                    *selection = Some(language);
                }
            }
            StepMessage::SliderChanged(new_value) => {
                if let Step::Slider { value, .. } = self {
                    *value = new_value.round() as u16;
                }
            }
            StepMessage::TextSizeChanged(new_size) => {
                if let Step::Text { size, .. } = self {
                    *size = new_size.round() as u16;
                }
            }
            StepMessage::TextColorChanged(new_color) => {
                if let Step::Text { color, .. } = self {
                    *color = new_color;
                }
            }
            StepMessage::LayoutChanged(new_layout) => {
                if let Step::RowsAndColumns { layout, .. } = self {
                    *layout = new_layout;
                }
            }
            StepMessage::SpacingChanged(new_spacing) => {
                if let Step::RowsAndColumns { spacing, .. } = self {
                    *spacing = new_spacing.round() as u16;
                }
            }
        };
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Buttons { .. } => true,
            Step::Checkbox { is_checked } => *is_checked,
            Step::Radio { selection } => *selection == Some(Language::Rust),
            Step::Slider { .. } => true,
            Step::Text { .. } => true,
            Step::RowsAndColumns { .. } => true,
            Step::End => false,
        }
    }

    fn layout(&mut self) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome().into(),
            Step::Buttons {
                primary,
                secondary,
                positive,
            } => Self::buttons(primary, secondary, positive).into(),
            Step::Checkbox { is_checked } => Self::checkbox(*is_checked).into(),
            Step::Radio { selection } => Self::radio(*selection).into(),
            Step::Slider { state, value } => Self::slider(state, *value).into(),
            Step::Text {
                size_slider,
                size,
                color_sliders,
                color,
            } => Self::text(size_slider, *size, color_sliders, *color).into(),
            Step::RowsAndColumns {
                layout,
                spacing_slider,
                spacing,
            } => {
                Self::rows_and_columns(*layout, spacing_slider, *spacing).into()
            }
            Step::End => Self::end().into(),
        }
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new()
            .spacing(20)
            .align_items(Align::Stretch)
            .push(Text::new(title).size(50))
    }

    fn welcome() -> Column<'a, StepMessage> {
        Self::container("Welcome!")
            .push(Text::new(
                "This is a tour that introduces some of the features and \
                 concepts related with UI development in Coffee.",
            ))
            .push(Text::new(
                "You will need to interact with the UI in order to reach the \
                 end!",
            ))
    }

    fn buttons(
        primary: &'a mut button::State,
        secondary: &'a mut button::State,
        positive: &'a mut button::State,
    ) -> Column<'a, StepMessage> {
        Self::container("Button")
            .push(Text::new("A button can fire actions when clicked."))
            .push(Text::new(
                "As of now, there are 3 different types of buttons: \
                 primary, secondary, and positive.",
            ))
            .push(Button::new(primary, "Primary"))
            .push(
                Button::new(secondary, "Secondary")
                    .class(button::Class::Secondary),
            )
            .push(
                Button::new(positive, "Positive")
                    .class(button::Class::Positive),
            )
            .push(Text::new(
                "Additional types will be added in the near future! Choose \
                 each type smartly depending on the situation.",
            ))
    }

    fn checkbox(is_checked: bool) -> Column<'a, StepMessage> {
        Self::container("Checkbox")
            .push(Text::new(
                "A box that can be checked. Useful to build toggle controls.",
            ))
            .push(Checkbox::new(
                is_checked,
                "Show \"Next\" button",
                StepMessage::CheckboxToggled,
            ))
            .push(Text::new(
                "A checkbox always has a label, and both the checkbox and its \
                 label can be clicked to toggle it.",
            ))
    }

    fn radio(selection: Option<Language>) -> Column<'a, StepMessage> {
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Coffee is written in..."))
            .push(Language::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, language| {
                    choices.push(Radio::new(
                        language,
                        language.into(),
                        selection,
                        StepMessage::LanguageSelected,
                    ))
                },
            ));

        Self::container("Radio button")
            .push(Text::new(
                "A radio button is normally used to represent a choice. Like \
                 a checkbox, it always has a label.",
            ))
            .push(question)
    }

    fn slider(
        state: &'a mut slider::State,
        value: u16,
    ) -> Column<'a, StepMessage> {
        Self::container("Slider")
            .push(Text::new(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            ))
            .push(Text::new(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            ))
            .push(Slider::new(
                state,
                0.0..=100.0,
                value as f32,
                StepMessage::SliderChanged,
            ))
            .push(
                Text::new(&value.to_string())
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn text(
        size_slider: &'a mut slider::State,
        size: u16,
        color_sliders: &'a mut [slider::State; 3],
        color: Color,
    ) -> Column<'a, StepMessage> {
        let size_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("You can change its size:"))
            .push(
                Text::new(&format!("This text is {} points", size)).size(size),
            )
            .push(Slider::new(
                size_slider,
                10.0..=50.0,
                size as f32,
                StepMessage::TextSizeChanged,
            ));

        let [red, green, blue] = color_sliders;
        let color_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("And its color:"))
            .push(Text::new(&format!("{:?}", color)).color(color))
            .push(
                Row::new()
                    .spacing(10)
                    .push(Slider::new(red, 0.0..=1.0, color.r, move |r| {
                        StepMessage::TextColorChanged(Color { r, ..color })
                    }))
                    .push(Slider::new(green, 0.0..=1.0, color.g, move |g| {
                        StepMessage::TextColorChanged(Color { g, ..color })
                    }))
                    .push(Slider::new(blue, 0.0..=1.0, color.b, move |b| {
                        StepMessage::TextColorChanged(Color { b, ..color })
                    })),
            );

        Self::container("Text")
            .push(Text::new(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            ))
            .push(size_section)
            .push(color_section)
    }

    fn rows_and_columns(
        layout: Layout,
        spacing_slider: &'a mut slider::State,
        spacing: u16,
    ) -> Column<'a, StepMessage> {
        let row_radio = Radio::new(
            Layout::Row,
            "Row",
            Some(layout),
            StepMessage::LayoutChanged,
        );

        let column_radio = Radio::new(
            Layout::Column,
            "Column",
            Some(layout),
            StepMessage::LayoutChanged,
        );

        let layout_section: Element<_> = match layout {
            Layout::Row => Row::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
            Layout::Column => Column::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
        };

        let spacing_section = Column::new()
            .spacing(10)
            .push(Slider::new(
                spacing_slider,
                0.0..=100.0,
                spacing as f32,
                StepMessage::SpacingChanged,
            ))
            .push(
                Text::new(&format!("{} px", spacing))
                    .horizontal_alignment(HorizontalAlignment::Center),
            );

        Self::container("Rows and columns")
            .spacing(spacing)
            .push(Text::new(
                "Coffee uses a layout model based on flexbox to position UI \
                 elements.",
            ))
            .push(Text::new(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            ))
            .push(layout_section)
            .push(Text::new(
                "You can also easily change the spacing between elements:",
            ))
            .push(spacing_section)
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push(Text::new(
                "This tour will be extended as more features are added.",
            ))
            .push(Text::new("Make sure to keep an eye on it!"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Other,
}

impl Language {
    fn all() -> [Language; 6] {
        [
            Language::C,
            Language::Elm,
            Language::Ruby,
            Language::Haskell,
            Language::Rust,
            Language::Other,
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
            Language::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    Row,
    Column,
}
