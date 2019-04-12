use super::Window;

use crate::graphics::{gpu, Color, Target};

pub struct Frame<'a> {
    window: &'a mut Window,
}

impl<'a> Frame<'a> {
    pub fn new(window: &mut Window) -> Frame {
        Frame { window }
    }

    pub fn width(&self) -> f32 {
        self.window.width
    }

    pub fn height(&self) -> f32 {
        self.window.height
    }

    pub fn as_target(&mut self) -> Target {
        let view = self.window.context.target().clone();
        let width = self.window.width;
        let height = self.window.height;

        Target::new(self.window.gpu(), view, width, height)
    }

    pub fn clear(&mut self, color: Color) {
        self.as_target().clear(color);
    }

    pub(in crate::graphics) fn draw_font(&mut self, font: &mut gpu::Font) {
        self.window.gpu.draw_font(
            font,
            &self.window.context.target(),
            &self.window.context.depth(),
        );
    }
}
