use crate::graphics;
use crate::loader::{Loader, Progress};

pub trait LoadingScreen {
    fn on_progress(
        &mut self,
        progress: f32,
        window: &mut graphics::Window,
    ) -> graphics::Result<()>;
}

pub fn run<T>(
    screen: &mut LoadingScreen,
    loader: &mut Loader<T>,
    window: &mut graphics::Window,
) -> T {
    screen.on_progress(0.0, window).unwrap();
    window.swap_buffers();

    loop {
        match loader.load(window.gpu()) {
            Progress::Loading { work_completed } => {
                screen
                    .on_progress(
                        work_completed as f32 / loader.total_work() as f32
                            * 100.0,
                        window,
                    )
                    .unwrap();
                window.swap_buffers();
            }
            Progress::Done(result) => return result,
        }
    }
}

pub struct ProgressBar {}

impl ProgressBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadingScreen for ProgressBar {
    fn on_progress(
        &mut self,
        progress: f32,
        _window: &mut graphics::Window,
    ) -> graphics::Result<()> {
        println!("{}", progress);
        Ok(())
    }
}
