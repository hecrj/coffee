use winit;

use crate::graphics::gpu::{self, Font, Gpu};
use crate::graphics::Color;

pub struct Window {
    gpu: Gpu,
    context: gpu::WindowedContext,
    screen_render_target: gpu::TargetView,
    depth_view: gpu::DepthView,
    width: f32,
    height: f32,
}

impl Window {
    pub fn new(mut settings: Settings, event_loop: &EventLoop) -> Window {
        let (mut width, mut height) = settings.size;

        // Try to revert DPI
        let dpi = event_loop.0.get_primary_monitor().get_hidpi_factor();

        width = (width as f64 / dpi).round() as u32;
        height = (height as f64 / dpi).round() as u32;

        settings.size = (width, height);

        let (gpu, context, screen_render_target, depth_view) =
            Gpu::window(settings.into_builder(), &event_loop.0);

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
            screen_render_target,
            depth_view,
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

    pub(crate) fn swap_buffers(&mut self) {
        self.gpu.flush();
        self.context.swap_buffers().unwrap();
        self.gpu.cleanup();
    }

    pub fn resize(&mut self, new_size: NewSize) {
        let dpi = self.context.window().get_hidpi_factor();
        let physical_size = new_size.0.to_physical(dpi);
        let new_viewport = Gpu::resize_viewport(
            &self.context,
            &self.screen_render_target,
            &self.depth_view,
        );

        if let Some((screen_render_target, depth_view)) = new_viewport {
            self.screen_render_target = screen_render_target;
            self.depth_view = depth_view;
        }

        self.width = physical_size.width as f32;
        self.height = physical_size.height as f32;
    }
}

pub struct EventLoop(winit::EventsLoop);

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop(winit::EventsLoop::new())
    }

    pub fn poll<F>(&mut self, mut f: F)
    where
        F: FnMut(Event),
    {
        self.0.poll_events(|event| {
            match event {
                winit::Event::WindowEvent { event, .. } => match event {
                    winit::WindowEvent::CloseRequested => {
                        f(Event::CloseRequested)
                    }
                    winit::WindowEvent::Resized(logical_size) => {
                        f(Event::Resized(NewSize(logical_size)))
                    }
                    _ => {}
                },
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
    Resized(NewSize),
}

pub struct NewSize(winit::dpi::LogicalSize);

pub struct Frame<'a> {
    window: &'a mut Window,
}

impl<'a> Frame<'a> {
    pub fn width(&self) -> f32 {
        self.window.width()
    }

    pub fn height(&self) -> f32 {
        self.window.height()
    }

    pub fn as_target(&mut self) -> gpu::Target {
        let view = self.window.screen_render_target.clone();
        let width = self.window.width;
        let height = self.window.height;

        gpu::Target::new(self.window.gpu(), view, width, height)
    }

    pub fn clear(&mut self, color: Color) {
        self.as_target().clear(color);
    }

    pub(super) fn draw_font(&mut self, font: &mut Font) {
        self.window.gpu.draw_font(
            font,
            &self.window.screen_render_target,
            &self.window.depth_view,
        );
    }
}
