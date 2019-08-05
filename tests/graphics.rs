use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

mod _graphics;

use _graphics::{test, Test};

#[test]
#[ignore]
fn graphics() -> Result<()> {
    Runner::run(WindowSettings {
        title: String::from("Graphics test suite - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

pub enum Runner {
    Pending(Vec<Task<Test>>),
    Drawing {
        remaining: Vec<Task<Test>>,
        current: Test,
        done: Vec<test::Drawing>,
    },
    Diffing {
        tests: Vec<test::Drawing>,
    },
    Reporting {
        results: Vec<test::Result>,
    },
    Finished,
}

impl Game for Runner {
    type LoadingScreen = ();
    type Input = ();

    fn load(_window: &Window) -> Task<Runner> {
        Task::succeed(|| Runner::Pending(Test::all()))
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        // TODO: Update logic
    }

    fn is_finished(&self) -> bool {
        match self {
            Runner::Finished => true,
            _ => false,
        }
    }
}
