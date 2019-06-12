use coffee::graphics::{
    Color, Frame, Mesh, Point, Shape, Window, WindowSettings,
};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    Example::run(WindowSettings {
        title: String::from("Mesh - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

struct Example {
    shape: Shape,
    mode: Mode,
}

enum Mode {
    Fill { color: Color },
    Stroke { color: Color, width: u16 },
}

impl Game for Example {
    type Input = ();
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Example> {
        let width = window.width();
        let height = window.height();

        Task::new(move || Example {
            shape: Shape::Circle {
                center: Point::new(width / 2.0, height / 2.0),
                radius: 100.0,
            },
            mode: Mode::Fill {
                color: Color::WHITE,
            },
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let mut mesh = Mesh::new();

        match self.mode {
            Mode::Fill { color } => {
                mesh.fill(self.shape, color);
            }
            Mode::Stroke { color, width } => {
                mesh.stroke(self.shape, color, width);
            }
        }

        mesh.draw(&mut frame.as_target());
    }
}
