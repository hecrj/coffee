use coffee::graphics::{
    Color, Frame, HorizontalAlignment, VerticalAlignment, Window,
    WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{
    Align, Column, Element, Justify, ProgressBar, Renderer, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

pub fn main() -> Result<()> {
    <Progress as UserInterface>::run(WindowSettings {
        title: String::from("Progress - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Progress {
    value: f32,
}

impl Game for Progress {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Progress> {
        Task::succeed(|| Progress { value: 0.0 })
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });

        if timer.has_ticked() {
            if self.value >= 1.0 {
                self.value = 0.0;
            }
            self.value += 0.002;
        }
    }
}

impl UserInterface for Progress {
    type Message = ();
    type Renderer = Renderer;

    fn react(&mut self, _message: (), _window: &mut Window) {}

    fn layout(&mut self, window: &Window) -> Element<()> {
        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .spacing(20)
            .push(
                Text::new(&format!("{:.0}%", self.value * 100.0))
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(ProgressBar::new(self.value).width(400))
            .into()
    }
}
