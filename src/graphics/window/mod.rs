mod event;
mod frame;
mod settings;

pub(crate) use event::{Event, EventLoop};
pub use frame::Frame;
pub use settings::Settings;

pub(crate) use crate::graphics::gpu::winit;
use crate::graphics::gpu::{self, Gpu};

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
}

impl Window {
    pub(crate) fn new(
        mut settings: Settings,
        event_loop: &EventLoop,
    ) -> Window {
        let (mut width, mut height) = settings.size;

        // Try to revert DPI
        let dpi = event_loop.raw().get_primary_monitor().get_hidpi_factor();

        width = (width as f64 / dpi).round() as u32;
        height = (height as f64 / dpi).round() as u32;

        settings.size = (width, height);

        let (gpu, surface) =
            Gpu::for_window(settings.into_builder(), event_loop.raw());

        let window = surface.window();

        let (width, height) = window
            .get_inner_size()
            .map(|inner_size| {
                let dpi = window.get_hidpi_factor();
                (
                    (inner_size.width * dpi) as f32,
                    (inner_size.height * dpi) as f32,
                )
            })
            .unwrap_or((width as f32, height as f32));

        Window {
            gpu,
            surface,
            width,
            height,
        }
    }

    /// Get the [`Gpu`] linked to the [`Window`].
    ///
    /// [`Gpu`]: struct.Gpu.html
    /// [`Window`]: struct.Window.html
    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }

    /// Get the next [`Frame`] that will be shown on the [`Window`].
    ///
    /// [`Frame`]: struct.Frame.html
    /// [`Window`]: struct.Window.html
    pub fn frame(&mut self) -> Frame {
        Frame::new(self)
    }

    /// Get the width of the [`Window`].
    ///
    /// [`Window`]: struct.Window.html
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the height of the [`Window`].
    ///
    /// [`Window`]: struct.Window.html
    pub fn height(&self) -> f32 {
        self.height
    }

    pub(crate) fn dpi(&self) -> f64 {
        self.surface.window().get_hidpi_factor()
    }

    pub(crate) fn swap_buffers(&mut self) {
        self.surface.swap_buffers(&mut self.gpu);
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::LogicalSize) {
        self.surface.update_viewport(&mut self.gpu);

        let dpi = self.surface.window().get_hidpi_factor();
        let physical_size = new_size.to_physical(dpi);

        self.width = physical_size.width as f32;
        self.height = physical_size.height as f32;
    }
}
