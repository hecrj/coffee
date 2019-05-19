use coffee::graphics::{Color, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    button, renderer, Button, Column, Panel, Root, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <Menu as UserInterface>::run(WindowSettings {
        title: String::from("Examples menu - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
}

struct Menu {
    state: State,
}

impl Game for Menu {
    type Input = KeyboardAndMouse;
    type State = ();
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Menu> {
        Task::new(|| Menu {
            state: State::new(),
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
            r: 0.5,
            g: 0.5,
            b: 0.6,
            a: 1.0,
        });
    }
}

impl UserInterface for Menu {
    type Event = Event;
    type Renderer = renderer::Basic;

    fn layout(
        &mut self,
        _state: &Self::State,
        window: &Window,
    ) -> Root<Event, Self::Renderer> {
        let content = match &mut self.state {
            State::Selection(selection) => {
                selection.layout().map(Event::SelectionEvent)
            }
            State::Particles => Panel::new(
                Column::new()
                    .fill_width()
                    .spacing(10)
                    .push(Text::new("Particles"))
                    .push(Text::new(
                        "A particle gravity simulator that showcases a \
                         loading screen, input handling, and graphics \
                         interpolation with batched drawing and font \
                         rendering.",
                    ))
                    .push(Text::new(
                        "The mouse cursor will attract the particles!",
                    )),
            )
            .max_width(500)
            .map(Event::SelectionEvent),
        };

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
            Event::SelectionEvent(SelectionEvent::ParticlesPressed) => {
                self.state = State::Particles;
            }
            _ => {}
        }
    }
}

struct Selection {
    particles_button: button::State,
    input_button: button::State,
    color_button: button::State,
}

impl Selection {
    fn new() -> Selection {
        Selection {
            particles_button: button::State::new(),
            input_button: button::State::new(),
            color_button: button::State::new(),
        }
    }

    fn layout(&mut self) -> Column<SelectionEvent, renderer::Basic> {
        Column::new()
            .width(300.0)
            .spacing(30)
            .push(
                Button::new(&mut self.particles_button, "Particles")
                    .on_click(SelectionEvent::ParticlesPressed),
            )
            .push(
                Button::new(&mut self.input_button, "Input")
                    .on_click(SelectionEvent::InputPressed),
            )
            .push(
                Button::new(&mut self.color_button, "Color")
                    .on_click(SelectionEvent::ColorPressed),
            )
    }
}

enum State {
    Selection(Selection),
    Particles,
}

impl State {
    fn new() -> State {
        State::Selection(Selection::new())
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    SelectionEvent(SelectionEvent),
}

#[derive(Debug, Clone, Copy)]
enum SelectionEvent {
    ParticlesPressed,
    InputPressed,
    ColorPressed,
}
