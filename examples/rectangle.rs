use coffee::graphics::{
    Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings,
};
use coffee::load::Task;
use coffee::{Game, Timer};

fn main() -> coffee::Result<()> {
    Example::run(WindowSettings {
        title: String::from("Rectangle - Coffee"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Example;

impl Game for Example {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Example> {
        Task::succeed(|| Example)
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 100.0,
            }),
            Color::WHITE,
        );
        mesh.draw(&mut frame.as_target());
    }
}
