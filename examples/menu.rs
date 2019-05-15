use coffee::graphics::{Color, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{button, renderer, Button, Column, Root, UserInterface};
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
        frame.clear(Color::BLACK);
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
        Root::new(
            Column::new()
                .width(window.width())
                .height(window.height())
                .center_children()
                .push(match &mut self.state {
                    State::Selection(selection) => selection.layout(),
                    State::Particles => Column::new(),
                }),
        )
    }

    fn update(&mut self, _state: &mut Self::State, event: Event) {
        match event {
            Event::ParticlesPressed => {
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

    fn layout(&mut self) -> Column<Event, renderer::Basic> {
        Column::new()
            .width(300.0)
            .spacing(30)
            .push(
                Button::new(&mut self.particles_button, "Particles")
                    .on_click(Event::ParticlesPressed),
            )
            .push(
                Button::new(&mut self.input_button, "Input")
                    .on_click(Event::InputPressed),
            )
            .push(
                Button::new(&mut self.color_button, "Color")
                    .on_click(Event::ColorPressed),
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
    ParticlesPressed,
    InputPressed,
    ColorPressed,
}
