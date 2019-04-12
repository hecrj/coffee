use gfx_device_gl as gl;
use gfx_window_glutin;
pub use gfx_winit as winit;
use glutin;

use super::{format, DepthView, Gpu, TargetView};

pub struct Surface {
    context: glutin::WindowedContext,
    target: TargetView,
    depth: DepthView,
}

impl Surface {
    pub(super) fn new(
        builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
    ) -> (Self, gl::Device, gl::Factory) {
        let gl_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(0)
            // 24 color bits, 8 alpha bits
            .with_pixel_format(24, 8)
            .with_vsync(true);

        let (context, device, factory, target, depth) =
            gfx_window_glutin::init_raw(
                builder,
                gl_builder,
                &events_loop,
                format::COLOR,
                format::DEPTH,
            )
            .unwrap();

        (
            Self {
                context,
                target,
                depth,
            },
            device,
            factory,
        )
    }

    pub fn window(&self) -> &winit::Window {
        self.context.window()
    }

    pub fn target(&self) -> &TargetView {
        &self.target
    }

    pub fn depth(&self) -> &DepthView {
        &self.depth
    }

    pub fn update_viewport(&mut self) {
        let dimensions = self.target.get_dimensions();

        if let Some((target, depth)) = gfx_window_glutin::update_views_raw(
            &self.context,
            dimensions,
            format::COLOR,
            format::DEPTH,
        ) {
            self.target = target;
            self.depth = depth;
        }
    }

    pub fn swap_buffers(&mut self, gpu: &mut Gpu) {
        gpu.flush();
        self.context.swap_buffers().unwrap();
        gpu.cleanup();
    }
}
