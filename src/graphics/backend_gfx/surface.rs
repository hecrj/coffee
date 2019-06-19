use gfx_device_gl as gl;
pub use gfx_winit as winit;

use super::{format, Gpu, TargetView};
use crate::{Error, Result};

pub struct Surface {
    context: glutin::WindowedContext,
    target: TargetView,
}

impl Surface {
    pub(super) fn new(
        builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
    ) -> Result<(Self, gl::Device, gl::Factory)> {
        let gl_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(0)
            // 24 color bits, 8 alpha bits
            .with_pixel_format(24, 8)
            .with_vsync(true);

        let (context, device, factory, target, _depth) =
            gfx_window_glutin::init_raw(
                builder,
                gl_builder,
                &events_loop,
                format::COLOR,
                format::DEPTH,
            )
            .map_err(|error| Error::WindowCreation(error.to_string()))?;

        Ok((Self { context, target }, device, factory))
    }

    pub fn window(&self) -> &winit::Window {
        self.context.window()
    }

    pub fn resize(&self, size: winit::dpi::PhysicalSize) {
        self.context.resize(size)
    }

    pub fn target(&self) -> &TargetView {
        &self.target
    }

    pub fn update_viewport(&mut self, _gpu: &mut Gpu) {
        let dimensions = self.target.get_dimensions();

        if let Some((target, _depth)) = gfx_window_glutin::update_views_raw(
            &self.context,
            dimensions,
            format::COLOR,
            format::DEPTH,
        ) {
            self.target = target;
        }
    }

    pub fn swap_buffers(&mut self, gpu: &mut Gpu) {
        gpu.flush();
        self.context.swap_buffers().expect("Buffer swap");
        gpu.cleanup();
    }
}
