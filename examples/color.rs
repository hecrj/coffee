use coffee::graphics::{
    Color, Font, Frame, Image, Point, Quad, Rectangle, Text, Window,
    WindowSettings,
};
use coffee::load::{loading_screen, Join, LoadingScreen, Task};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    Colors::run(WindowSettings {
        title: String::from("Color - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

struct Colors;

impl Game for Colors {
    type View = View;
    type Input = ();

    const TICKS_PER_SECOND: u16 = 10;

    fn new(window: &mut Window) -> Result<(Self, Self::View, Self::Input)> {
        let load = Task::stage("Loading view...", View::load());

        let mut loading_screen = loading_screen::ProgressBar::new(window.gpu());
        let view = loading_screen.run(load, window)?;

        Ok((Colors, view, ()))
    }

    fn update(&mut self, _view: &Self::View, _window: &Window) {}

    fn draw(&self, view: &mut Self::View, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::new(0.5, 0.5, 0.5, 1.0));

        let target = &mut frame.as_target();

        view.palette.draw(
            Quad {
                source: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                },
                position: Point::new(0.0, 0.0),
                size: (500.0, 500.0),
            },
            target,
        );

        view.font.add(Text {
            content: String::from("Prussian blue"),
            position: Point::new(20.0, 500.0),
            size: 50.0,
            color: View::PRUSSIAN_BLUE,
            ..Text::default()
        });

        view.font.draw(target);
    }
}

struct View {
    palette: Image,
    font: Font,
}

impl View {
    const PRUSSIAN_BLUE: Color = Color {
        r: 0.0,
        g: 0.1922,
        b: 0.3255,
        a: 1.0,
    };

    fn load() -> Task<View> {
        (
            Task::using_gpu(|gpu| {
                Image::from_colors(gpu, &[Self::PRUSSIAN_BLUE])
            }),
            Font::load(include_bytes!(
                "../resources/font/Inconsolata-Regular.ttf"
            )),
        )
            .join()
            .map(|(palette, font)| View { palette, font })
    }
}
