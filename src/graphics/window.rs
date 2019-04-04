use gfx_window_glutin;
use glutin;
use winit;

use crate::graphics::color::Color;
use crate::graphics::gpu::{self, Gpu};

pub struct Window {
    context: glutin::WindowedContext,
    events_loop: winit::EventsLoop,
    gpu: Gpu,
    screen_render_target: gpu::TargetView,
    width: f32,
    height: f32,
}

impl Window {
    pub fn new(settings: Settings) -> Window {
        let (width, height) = settings.size;

        let gl_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(1)
            // 24 color bits, 8 alpha bits
            .with_pixel_format(24, 8)
            .with_vsync(true);

        let events_loop = winit::EventsLoop::new();

        let (context, device, factory, screen_render_target, depth_view) =
            gfx_window_glutin::init_raw(
                settings.into_builder(),
                gl_builder,
                &events_loop,
                gpu::COLOR_FORMAT,
                gpu::DEPTH_FORMAT,
            )
            .unwrap();

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
            events_loop,
            gpu: Gpu::new(device, factory, &screen_render_target, depth_view),
            screen_render_target,
            width,
            height,
        }
    }

    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }

    pub fn frame(&mut self) -> Frame {
        Frame { window: self }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
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

impl Settings {
    fn into_builder(self) -> winit::WindowBuilder {
        winit::WindowBuilder::new()
            .with_title(self.title)
            .with_dimensions(winit::dpi::LogicalSize {
                width: self.size.0 as f64,
                height: self.size.1 as f64,
            })
            .with_resizable(self.resizable)
    }
}

pub enum Event {
    CloseRequested,
}

pub struct Frame<'a> {
    window: &'a mut Window,
}

impl<'a> Frame<'a> {
    pub fn as_target(&mut self) -> gpu::Target {
        let view = self.window.screen_render_target.clone();
        let width = self.window.width;
        let height = self.window.height;

        gpu::Target::new(self.window.gpu(), view, width, height)
    }

    pub fn clear(&mut self, color: Color) {
        self.as_target().clear(color);
    }

    pub fn present(self) {
        self.window.gpu.flush();
        self.window.context.swap_buffers().unwrap();
        self.window.gpu.cleanup();
    }
}
