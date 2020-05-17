use coffee::graphics::{
    Color, Font, Frame, Image, Point, Quad, Rectangle, Text, Window,
    WindowSettings,
};
use coffee::load::{loading_screen::ProgressBar, Join, Task};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    Colors::run(WindowSettings {
        title: String::from("Color - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Colors {
    palette: Image,
    font: Font,
}

impl Colors {
    const PRUSSIAN_BLUE: Color = Color {
        r: 0.0,
        g: 0.1922,
        b: 0.3255,
        a: 1.0,
    };

    fn load() -> Task<Colors> {
        (
            Task::using_gpu(|gpu| {
                Image::from_colors(gpu, &[Self::PRUSSIAN_BLUE])
            }),
            Font::load_from_bytes(include_bytes!(
                "../resources/font/Inconsolata-Regular.ttf"
            )),
        )
            .join()
            .map(|(palette, font)| Colors { palette, font })
    }
}

impl Game for Colors {
    type Input = ();
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Self> {
        Task::stage("Loading view...", Colors::load())
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::new(0.5, 0.5, 0.5, 1.0));

        let target = &mut frame.as_target();

        self.palette.draw(
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

        self.font.add(Text {
            content: "Prussian blue",
            position: Point::new(20.0, 500.0),
            size: 50.0,
            color: Self::PRUSSIAN_BLUE,
            ..Text::default()
        });

        self.font.draw(target);
    }
}
