mod cursor_icon;
mod frame;
mod settings;

pub(crate) use winit;

pub use cursor_icon::CursorIcon;
pub use frame::Frame;
pub use settings::Settings;

use crate::graphics::gpu::{self, Gpu};
use crate::Result;

/// An open window.
///
/// It is provided as an argument in some methods in [`Game`].
///
/// [`Game`]: ../trait.Game.html
pub struct Window {
    gpu: Gpu,
    surface: gpu::Surface,
    width: f32,
    height: f32,
    is_fullscreen: bool,
    cursor_icon: Option<winit::window::CursorIcon>,
}

impl Window {
    pub(crate) fn new(
        settings: Settings,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Window> {
        let (width, height) = settings.size;
        let is_fullscreen = settings.fullscreen;

        let (gpu, surface) = Gpu::for_window(settings, event_loop)?;

        Ok(Window {
            is_fullscreen,
            gpu,
            surface,
            width: width as f32,
            height: height as f32,
            cursor_icon: Some(winit::window::CursorIcon::Default),
        })
    }

    /// Returns the [`Gpu`] linked to the [`Window`].
    ///
    /// [`Gpu`]: struct.Gpu.html
    /// [`Window`]: struct.Window.html
    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }

    pub(crate) fn frame(&mut self) -> Frame<'_> {
        Frame::new(self)
    }

    /// Toggles the [`Window`]'s fullscreen state.
    ///
    /// [`Window`]: struct.Window.html
    pub fn toggle_fullscreen(&mut self) {
        let window = self.surface.window();

        let monitor = if self.is_fullscreen {
            None
        } else {
            Some(window.primary_monitor())
        };

        window
            .set_fullscreen(monitor.map(winit::window::Fullscreen::Borderless));

        self.is_fullscreen = !self.is_fullscreen;
    }

    /// Returns the width of the [`Window`].
    ///
    /// [`Window`]: struct.Window.html
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Returns the height of the [`Window`].
    ///
    /// [`Window`]: struct.Window.html
    pub fn height(&self) -> f32 {
        self.height
    }

    pub(crate) fn swap_buffers(&mut self) {
        self.surface.swap_buffers(&mut self.gpu);
    }

    pub(crate) fn request_redraw(&mut self) {
        self.surface.request_redraw();
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface.resize(&mut self.gpu, new_size);

        self.width = new_size.width as f32;
        self.height = new_size.height as f32;
    }

    pub(crate) fn update_cursor(
        &mut self,
        new_cursor: Option<winit::window::CursorIcon>,
    ) {
        if self.cursor_icon != new_cursor {
            if let Some(cursor_icon) = new_cursor {
                self.surface.window().set_cursor_icon(cursor_icon);
            }
            self.surface
                .window()
                .set_cursor_visible(new_cursor.is_some());
            self.cursor_icon = new_cursor;
        }
    }
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Window {{ width: {}, height: {} }}",
            self.width, self.height
        )
    }
}
