use gfx;
use gfx_window_glutin;
use glutin;
use winit;

use crate::graphics::gpu::Gpu;

pub struct Window {
    context: glutin::WindowedContext,
    events_loop: winit::EventsLoop,
    gpu: Gpu,
}

impl Window {
    pub fn new(settings: Settings) -> Window {
        let window_builder = winit::WindowBuilder::new()
            .with_title(settings.title)
            .with_dimensions(winit::dpi::LogicalSize {
                width: settings.size.0 as f64,
                height: settings.size.1 as f64,
            })
            .with_resizable(settings.resizable);

        let gl_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(1)
            // 24 color bits, 8 alpha bits
            .with_pixel_format(24, 8)
            .with_vsync(true);

        let events_loop = winit::EventsLoop::new();
        let color_format = gfx::format::Format(
            gfx::format::SurfaceType::R8_G8_B8_A8,
            gfx::format::ChannelType::Unorm,
        );

        let depth_format = gfx::format::Format(
            gfx::format::SurfaceType::D24_S8,
            gfx::format::ChannelType::Unorm,
        );

        let (context, device, mut factory, screen_render_target, depth_view) =
            gfx_window_glutin::init_raw(
                window_builder,
                gl_builder,
                &events_loop,
                color_format,
                depth_format,
            )
            .unwrap();

        Window {
            context,
            events_loop,
            gpu: Gpu::new(),
        }
    }

    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }

    pub fn frame(&mut self) -> Frame {
        Frame { window: self }
    }

    pub fn physical_size(&self) -> Option<(f32, f32)> {
        let window = &self.context.window();

        window.get_inner_size().map(|inner_size| {
            let dpi = window.get_hidpi_factor();
            (
                (inner_size.width * dpi) as f32,
                (inner_size.height * dpi) as f32,
            )
        })
    }

    pub fn poll_events<F>(&mut self, mut f: F)
    where
        F: FnMut(Event),
    {
        self.events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => f(Event::CloseRequested),
                _ => (),
            };
        });
    }
}

pub struct Settings {
    pub title: String,
    pub size: (u32, u32),
    pub resizable: bool,
}

pub enum Event {
    CloseRequested,
}

pub struct Frame<'a> {
    window: &'a mut Window,
}

impl<'a> Frame<'a> {
    pub fn clear(&mut self) {}

    pub fn present(self) {
        self.window.context.swap_buffers().unwrap();
    }
}
