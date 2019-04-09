use crate::graphics::gpu;
use crate::graphics::{Frame, Gpu, Text};

pub struct Font(gpu::Font);

impl Font {
    pub fn from_bytes(gpu: &mut Gpu, bytes: &'static [u8]) -> Font {
        Font(gpu.upload_font(bytes))
    }

    pub fn add(&mut self, text: Text) {
        self.0.add(text)
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.draw_font(&mut self.0)
    }
}
