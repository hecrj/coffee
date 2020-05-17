use coffee::graphics::{
    self, Color, Frame, HorizontalAlignment, VerticalAlignment, Window,
    WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{
    Align, Column, Element, Image, Justify, Renderer, Text, UserInterface,
};
use coffee::{Game, Result, Timer};

pub fn main() -> Result<()> {
    <ImageScreen as UserInterface>::run(WindowSettings {
        title: String::from("ImageScreen - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct ImageScreen {
    image: graphics::Image,
}

impl Game for ImageScreen {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ImageScreen> {
        graphics::Image::load("resources/ui.png")
            .map(|image| ImageScreen { image })
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

impl UserInterface for ImageScreen {
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
                Text::new("This is an image")
                    .size(50)
                    .height(60)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(Image::new(&self.image).height(250))
            .into()
    }
}
