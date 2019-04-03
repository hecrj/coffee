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

pub(super) type TargetView =
    gfx::handle::RenderTargetView<gl::Resources, gfx::format::Srgba8>;

pub struct Gpu {
    device: gl::Device,
    factory: gl::Factory,
    encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer>,
    depth_view: gfx::handle::RawDepthStencilView<gl::Resources>,
    pipeline: Pipeline,
}

impl Gpu {
    pub(super) fn new(
        device: gl::Device,
        mut factory: gl::Factory,
        screen_render_target: gfx::handle::RawRenderTargetView<gl::Resources>,
        depth_view: gfx::handle::RawDepthStencilView<gl::Resources>,
    ) -> Gpu {
        let encoder = factory.create_command_buffer().into();
        let pipeline = Pipeline::new(&mut factory, &screen_render_target);

        Gpu {
            device,
            factory,
            encoder,
            depth_view,
            pipeline,
        }
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
    pub(super) fn new(gpu: &mut Gpu, view: TargetView) -> Target {
        Target {
            gpu,
            view,
            transformation: Transformation::identity(),
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.gpu.encoder.clear(&self.view, color.into())
    }

    pub fn transform(&mut self, new_transformation: Transformation) -> Target {
        Target {
            gpu: self.gpu,
            view: self.view.clone(),
            transformation: new_transformation * self.transformation,
        }
    }

    pub(super) fn draw_texture(
        &mut self,
        texture: &Texture,
        parameters: DrawParameters,
    ) {
        self.gpu.pipeline.bind_texture(texture);

        self.gpu.pipeline.draw_quad(
            pipeline::Instance::from_parameters(parameters),
            &self.transformation,
            &self.view,
        );
    }

    pub(super) fn flush(&mut self) {
        self.gpu.encoder.flush(&mut self.gpu.device);
    }
}
