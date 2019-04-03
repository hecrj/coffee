mod pipeline;
mod texture;

use gfx::{self, Device};
use gfx_device_gl as gl;

use pipeline::Pipeline;
use crate::graphics::color::Color;

pub(super) const COLOR_FORMAT: gfx::format::Format = gfx::format::Format(
    gfx::format::SurfaceType::R8_G8_B8_A8,
    gfx::format::ChannelType::Unorm,
);

pub(super) const DEPTH_FORMAT: gfx::format::Format = gfx::format::Format(
    gfx::format::SurfaceType::D24_S8,
    gfx::format::ChannelType::Unorm,
);

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
        let mut encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer> =
            factory.create_command_buffer().into();

        let pipeline = Pipeline::new(&mut factory, &screen_render_target);

        Gpu {
            device,
            factory,
            encoder,
            depth_view,
            pipeline,
        }
    }

    pub fn clear(&mut self, target: Target, color: Color) {
        self.encoder.clear(&target.0, color.into())
    }

    pub(super) fn flush(&mut self, target: Target) {
        self.encoder.flush(&mut self.device);
    }

    pub(super) fn cleanup(&mut self) {
        self.device.cleanup();
    }
}

#[derive(Clone)]
pub struct Target(
    pub(super) gfx::handle::RenderTargetView<gl::Resources, gfx::format::Srgba8>,
);
