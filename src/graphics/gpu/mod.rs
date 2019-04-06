mod format;
mod pipeline;
pub mod texture;
mod types;

use gfx::{self, Device};
use gfx_device_gl as gl;
use glutin;
use winit;

pub use self::pipeline::Vertex;
pub use self::texture::Texture;
pub use glutin::WindowedContext;
pub use types::TargetView;

use crate::graphics::{Color, DrawParameters, Transformation};
use pipeline::Pipeline;

pub struct Gpu {
    device: gl::Device,
    factory: gl::Factory,
    depth_view: gfx::handle::RawDepthStencilView<gl::Resources>,
    pipeline: Pipeline,
}

impl Gpu {
    pub(super) fn new(
        device: gl::Device,
        mut factory: gl::Factory,
        screen_render_target: &gfx::handle::RawRenderTargetView<gl::Resources>,
        depth_view: gfx::handle::RawDepthStencilView<gl::Resources>,
    ) -> Gpu {
        let pipeline = Pipeline::new(&mut factory, screen_render_target);

        Gpu {
            device,
            factory,
            depth_view,
            pipeline,
        }
    }

    pub(super) fn window(
        builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
    ) -> (Gpu, glutin::WindowedContext, TargetView) {
        let gl_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(1)
            // 24 color bits, 8 alpha bits
            .with_pixel_format(24, 8)
            .with_vsync(true);

        let (context, device, factory, screen_render_target, depth_view) =
            gfx_window_glutin::init_raw(
                builder,
                gl_builder,
                &events_loop,
                format::COLOR,
                format::DEPTH,
            )
            .unwrap();

        (
            Gpu::new(device, factory, &screen_render_target, depth_view),
            context,
            screen_render_target,
        )
    }

    pub(super) fn flush(&mut self) {
        self.pipeline.flush(&mut self.device);
    }

    pub(super) fn cleanup(&mut self) {
        self.device.cleanup();
    }

    pub(super) fn upload_texture(
        &mut self,
        image: &image::DynamicImage,
    ) -> Texture {
        Texture::new(&mut self.factory, image)
    }

    pub(super) fn upload_texture_array(
        &mut self,
        layers: &[image::DynamicImage],
    ) -> Texture {
        Texture::new_array(&mut self.factory, layers)
    }

    pub(super) fn create_drawable_texture(
        &mut self,
        width: u16,
        height: u16,
    ) -> texture::Drawable {
        texture::Drawable::new(&mut self.factory, width, height)
    }
}

pub struct Target<'a> {
    gpu: &'a mut Gpu,
    view: TargetView,
    transformation: Transformation,
}

impl<'a> Target<'a> {
    pub(super) fn new(
        gpu: &mut Gpu,
        view: TargetView,
        width: f32,
        height: f32,
    ) -> Target {
        Target {
            gpu,
            view,
            transformation: Transformation::orthographic(width, height),
        }
    }

    pub(super) fn with_transformation(
        gpu: &mut Gpu,
        view: TargetView,
        width: f32,
        height: f32,
        transformation: Transformation,
    ) -> Target {
        let mut target = Self::new(gpu, view, width, height);
        target.transformation = transformation * target.transformation;
        target
    }

    pub fn transform(&mut self, new_transformation: Transformation) -> Target {
        Target {
            gpu: self.gpu,
            view: self.view.clone(),
            transformation: self.transformation * new_transformation,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.gpu.pipeline.clear(&self.view, color);
    }

    pub(super) fn draw_texture(
        &mut self,
        texture: &Texture,
        parameters: DrawParameters,
    ) {
        self.gpu.pipeline.bind_texture(texture);

        self.gpu.pipeline.draw_quad(
            Vertex::from_parameters(parameters),
            &self.transformation,
            &self.view,
        );
    }

    pub(super) fn draw_texture_quads_from_vertices(
        &mut self,
        texture: &Texture,
        vertices: &[Vertex],
    ) {
        self.gpu.pipeline.bind_texture(texture);

        self.gpu.pipeline.draw_quads_from_vertices(
            vertices,
            &self.transformation,
            &self.view,
        );
    }
}
