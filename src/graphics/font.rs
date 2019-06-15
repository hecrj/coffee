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

    /// Loads a [`Font`] from raw data.
    ///
    /// [`Font`]: struct.Font.html
    pub fn from_bytes(gpu: &mut Gpu, bytes: &'static [u8]) -> Result<Font> {
        Ok(Font(gpu.upload_font(bytes)))
    }

    /// Creates a [`Task`] that loads a [`Font`] from raw data.
    ///
    /// [`Task`]: ../load/struct.Task.html
    /// [`Font`]: struct.Font.html
    pub fn load_from_bytes(bytes: &'static [u8]) -> Task<Font> {
        Task::using_gpu(move |gpu| Font::from_bytes(gpu, bytes))
    }

    /// Adds [`Text`] to this [`Font`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`Font`]: struct.Font.html
    pub fn add(&mut self, text: Text<'_>) {
        self.0.add(text)
    }

    /// Computes the layout bounds of the given [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    pub fn measure(&mut self, text: Text<'_>) -> (f32, f32) {
        self.0.measure(text)
    }

    /// Renders and flushes all the text added to this [`Font`].
    ///
    /// [`Font`]: struct.Font.html
    #[inline]
    pub fn draw(&mut self, target: &mut Target<'_>) {
        target.draw_font(&mut self.0)
    }
}
