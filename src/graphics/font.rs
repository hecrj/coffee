use crate::graphics::gpu;
use crate::graphics::{Gpu, Target, Text};
use crate::load::Task;
use crate::Result;

/// A collection of text with the same font.
#[allow(missing_debug_implementations)]
pub struct Font(gpu::Font);

impl Font {
    pub(crate) const DEFAULT: &'static [u8] =
        include_bytes!("../../resources/font/Inconsolata-Regular.ttf");

    /// Load a font from raw data.
    pub fn from_bytes(gpu: &mut Gpu, bytes: &'static [u8]) -> Result<Font> {
        Ok(Font(gpu.upload_font(bytes)))
    }

    /// Create a task that loads a font from raw data.
    pub fn load(bytes: &'static [u8]) -> Task<Font> {
        Task::using_gpu(move |gpu| Font::from_bytes(gpu, bytes))
    }

    /// Add text to this font.
    pub fn add(&mut self, text: Text) {
        self.0.add(text)
    }

    pub fn measure(&mut self, text: Text) -> (f32, f32) {
        self.0.measure(text)
    }

    /// Render and flush all the text added to this [`Font`].
    ///
    /// [`Font`]: struct.Font.html
    #[inline]
    pub fn draw(&mut self, target: &mut Target) {
        target.draw_font(&mut self.0)
    }
}
