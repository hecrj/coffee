use gfx_device_gl as gl;

use super::{format, Gpu, TargetView};
use crate::{graphics::WindowSettings, Error, Result};

pub struct Surface {
    context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    target: TargetView,
}

impl Surface {
    pub(super) fn new(
        settings: WindowSettings,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<(Self, gl::Device, gl::Factory)> {
        let gl_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(0)
            // 24 color bits, 8 alpha bits
            .with_pixel_format(24, 8)
            .with_vsync(settings.vsync);

        let builder = settings.into_builder(event_loop);
        let (context, device, factory, target, _depth) = init_raw(
            builder,
            gl_builder,
            &event_loop,
            format::COLOR,
            format::DEPTH,
        )
        .map_err(|error| Error::WindowCreation(error.to_string()))?;

        Ok((Self { context, target }, device, factory))
    }

    pub fn window(&self) -> &winit::window::Window {
        self.context.window()
    }

    pub fn target(&self) -> &TargetView {
        &self.target
    }

    pub fn resize(
        &mut self,
        _gpu: &mut Gpu,
        size: winit::dpi::PhysicalSize<u32>,
    ) {
        self.context.resize(size);

        let dimensions = self.target.get_dimensions();

        if let Some((target, _depth)) = update_views_raw(
            &self.context,
            dimensions,
            format::COLOR,
            format::DEPTH,
        ) {
            self.target = target;
        }
    }

    pub fn request_redraw(&mut self) {
        self.context.window().request_redraw();
    }

    pub fn swap_buffers(&mut self, gpu: &mut Gpu) {
        gpu.flush();
        self.context.swap_buffers().expect("Buffer swap");
        gpu.cleanup();
    }
}

fn init_raw(
    window: glutin::window::WindowBuilder,
    context: glutin::ContextBuilder<'_, glutin::NotCurrent>,
    events_loop: &glutin::event_loop::EventLoop<()>,
    color_format: gfx::format::Format,
    ds_format: gfx::format::Format,
) -> std::result::Result<
    (
        glutin::WindowedContext<glutin::PossiblyCurrent>,
        gl::Device,
        gl::Factory,
        gfx::handle::RawRenderTargetView<gl::Resources>,
        gfx::handle::RawDepthStencilView<gl::Resources>,
    ),
    glutin::CreationError,
> {
    let window = {
        let color_total_bits = color_format.0.get_total_bits();
        let alpha_bits = color_format.0.get_alpha_stencil_bits();
        let depth_total_bits = ds_format.0.get_total_bits();
        let stencil_bits = ds_format.0.get_alpha_stencil_bits();

        context
            .with_depth_buffer(depth_total_bits - stencil_bits)
            .with_stencil_buffer(stencil_bits)
            .with_pixel_format(color_total_bits - alpha_bits, alpha_bits)
            .with_srgb(color_format.1 == gfx::format::ChannelType::Srgb)
            .build_windowed(window, events_loop)?
    };

    let (window, device, factory, color_view, ds_view) =
        init_existing_raw(window, color_format, ds_format);

    Ok((window, device, factory, color_view, ds_view))
}

fn init_existing_raw(
    window: glutin::WindowedContext<glutin::NotCurrent>,
    color_format: gfx::format::Format,
    ds_format: gfx::format::Format,
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    gl::Device,
    gl::Factory,
    gfx::handle::RawRenderTargetView<gl::Resources>,
    gfx::handle::RawDepthStencilView<gl::Resources>,
) {
    #[allow(unsafe_code)]
    let window = unsafe { window.make_current().unwrap() };

    let (device, factory) = gl::create(|s| {
        window.get_proc_address(s) as *const std::os::raw::c_void
    });

    // create the main color/depth targets
    let dim = get_window_dimensions(&window);
    let (color_view, ds_view) =
        gl::create_main_targets_raw(dim, color_format.0, ds_format.0);

    // done
    (window, device, factory, color_view, ds_view)
}

pub fn update_views_raw(
    window: &glutin::WindowedContext<glutin::PossiblyCurrent>,
    old_dimensions: gfx::texture::Dimensions,
    color_format: gfx::format::Format,
    ds_format: gfx::format::Format,
) -> Option<(
    gfx::handle::RawRenderTargetView<gl::Resources>,
    gfx::handle::RawDepthStencilView<gl::Resources>,
)> {
    let dim = get_window_dimensions(window);

    if dim != old_dimensions {
        Some(gl::create_main_targets_raw(
            dim,
            color_format.0,
            ds_format.0,
        ))
    } else {
        None
    }
}

fn get_window_dimensions(
    ctx: &glutin::WindowedContext<glutin::PossiblyCurrent>,
) -> gfx::texture::Dimensions {
    let window = ctx.window();

    let (width, height) = {
        let size = window.inner_size();
        (size.width as _, size.height as _)
    };

    let aa = ctx.get_pixel_format().multisampling.unwrap_or(0)
        as gfx::texture::NumSamples;

    (width, height, 1, aa.into())
}
