mod event;
mod frame;
mod settings;

pub use event::{Event, EventLoop, NewSize};
pub use frame::Frame;
pub use settings::Settings;

pub(crate) use crate::graphics::gpu::winit;
use crate::graphics::gpu::{self, Gpu};

pub struct Window {
    gpu: Gpu,
    context: gpu::WindowedContext,
    width: f32,
    height: f32,
}

impl Window {
    pub fn new(mut settings: Settings, event_loop: &EventLoop) -> Window {
        let (mut width, mut height) = settings.size;

        // Try to revert DPI
        let dpi = event_loop.raw().get_primary_monitor().get_hidpi_factor();

        width = (width as f64 / dpi).round() as u32;
        height = (height as f64 / dpi).round() as u32;

        settings.size = (width, height);

        let (gpu, context) =
            Gpu::window(settings.into_builder(), event_loop.raw());

        let window = context.window();

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
            context,
            gpu,
            width,
            height,
        }
    }

    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }

    pub fn frame(&mut self) -> Frame {
        Frame::new(self)
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub(crate) fn swap_buffers(&mut self) {
        self.context.swap_buffers(&mut self.gpu);
    }

    pub fn resize(&mut self, new_size: event::NewSize) {
        self.context.update_viewport();

        let dpi = self.context.window().get_hidpi_factor();
        let physical_size = new_size.to_physical(dpi);

        self.width = physical_size.width as f32;
        self.height = physical_size.height as f32;
    }
}
