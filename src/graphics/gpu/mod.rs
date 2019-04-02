mod pipeline;
mod texture;

use gfx;
use gfx_device_gl as gl;

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
    screen_render_target: gfx::handle::RawRenderTargetView<gl::Resources>,
    depth_view: gfx::handle::RawDepthStencilView<gl::Resources>,
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

        Gpu {
            device,
            factory,
            encoder,
            screen_render_target,
            depth_view,
        }
    }
}
