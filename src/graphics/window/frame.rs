use super::Window;

use crate::graphics::{Color, Gpu, Target};

/// The next frame of your game.
///
/// You can only get a [`Frame`] by using [`Window::frame`].
///
/// This type is useful to define explicit rendering function signatures. If
/// a function should never render off-screen, consider taking a `Frame` as an
/// argument instead of a generic [`Target`].
///
/// [`Frame`]: struct.Frame.html
/// [`Window::frame`]: struct.Window.html#method.frame
/// [`Target`]: struct.Target.html
#[derive(Debug)]
pub struct Frame<'a> {
    window: &'a mut Window,
}

impl<'a> Frame<'a> {
    pub(crate) fn new(window: &mut Window) -> Frame<'_> {
        Frame { window }
    }

    /// Get the [`Gpu`] linked to the [`Window`] of this [`Frame`].
    ///
    /// [`Gpu`]: struct.Gpu.html
    /// [`Window`]: struct.Window.html
    /// [`Frame`]: struct.Frame.html
    pub fn gpu(&mut self) -> &mut Gpu {
        self.window.gpu()
    }

    /// Get the width of the frame.
    pub fn width(&self) -> f32 {
        self.window.width
    }

    /// Get the height of the frame.
    pub fn height(&self) -> f32 {
        self.window.height
    }

    /// See the frame as a [`Target`].
    ///
    /// You will need to use this in order to render some resources to it.
    ///
    /// [`Target`]: struct.Target.html
    pub fn as_target(&mut self) -> Target<'_> {
        let Window {
            surface,
            gpu,
            width,
            height,
            ..
        } = &mut self.window;

        let view = surface.target();

        Target::new(gpu, view, *width, *height)
    }

    /// Clear the frame with the given [`Color`].
    ///
    /// [`Color`]: struct.Color.html
    pub fn clear(&mut self, color: Color) {
        self.as_target().clear(color);
    }
}
