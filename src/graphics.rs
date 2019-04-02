use crate::window::Window;

pub struct Gpu {
    window: Window,
}

impl Gpu {
    pub fn new(window: Window) -> Gpu {
        Gpu { window }
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn current_frame(&mut self) -> Frame {
        Frame { gpu: self }
    }
}

pub struct Frame<'a> {
    gpu: &'a mut Gpu,
}

impl<'a> Frame<'a> {
    pub fn clear(&mut self) {}
    pub fn present(&mut self) {}
}

pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

pub enum Error {}
pub type Result<T> = std::result::Result<T, Error>;
