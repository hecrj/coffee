mod pipeline;
mod texture;

pub use crate::graphics::gpu::texture::Texture;

use gfx::{self, Device};
use gfx_device_gl as gl;

use crate::graphics::color::Color;
use crate::graphics::draw_parameters::DrawParameters;
use crate::graphics::transformation::Transformation;
use pipeline::Pipeline;

pub(super) const COLOR_FORMAT: gfx::format::Format = gfx::format::Format(
    gfx::format::SurfaceType::R8_G8_B8_A8,
    gfx::format::ChannelType::Unorm,
);

pub(super) const DEPTH_FORMAT: gfx::format::Format = gfx::format::Format(
    gfx::format::SurfaceType::D24_S8,
    gfx::format::ChannelType::Unorm,
);

pub(super) type TargetView = gfx::handle::RawRenderTargetView<gl::Resources>;

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
        let pipeline =
            Pipeline::new(&mut factory, screen_render_target, COLOR_FORMAT);

        Gpu {
            device,
            factory,
            depth_view,
            pipeline,
        }
    }

    pub(super) fn flush(&mut self) {
        self.pipeline.flush(&mut self.device);
    }

    pub(super) fn cleanup(&mut self) {
        self.device.cleanup();
    }

    pub(super) fn upload_image(
        &mut self,
        image: &image::DynamicImage,
    ) -> Texture {
        Texture::new(&mut self.factory, image)
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

    pub fn transform(&mut self, new_transformation: Transformation) -> Target {
        Target {
            gpu: self.gpu,
            view: self.view.clone(),
            transformation: self.transformation * new_transformation,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.gpu.pipeline.clear(&self.view, color.into())
    }

    pub(super) fn draw_texture(
        &mut self,
        texture: &Texture,
        parameters: DrawParameters,
    ) {
        self.gpu.pipeline.bind_texture(texture);

        self.gpu.pipeline.draw_quad(
            pipeline::Point::from_parameters(parameters),
            &self.transformation,
            &self.view,
        );
    }
}
