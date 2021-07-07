use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::input::keyboard;
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result <()> {
    PongGame::run(WindowSettings {
        title: String::from("Pong"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct PongGame {

}

impl Game for PongGame {
    type Input = (keyboard);
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<PongAssets> {
        Task::succeed(|| PongAssets {
            // Insert pong assets here
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the frame
        frame.clear(Color::BLACK);
    }
}