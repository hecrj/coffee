use gfx;
use gfx::format::{ChannelTyped, SurfaceTyped};
use gfx_core::factory::Factory;
use gfx_device_gl as gl;
use image;

type Surface = <gfx::format::Rgba8 as gfx::format::Formatted>::Surface;
type Channel = <gfx::format::Rgba8 as gfx::format::Formatted>::Channel;

pub struct Texture {
    texture: gfx::handle::RawTexture<gl::Resources>,
    view: gfx::handle::RawShaderResourceView<gl::Resources>,
}

impl Texture {
    pub fn new(
        factory: &mut gl::Factory,
        image: &image::DynamicImage,
    ) -> Texture {
        let rgba = image.to_rgba();

        let kind = gfx::texture::Kind::D2(
            rgba.width() as u16,
            rgba.height() as u16,
            gfx::texture::AaMode::Single,
        );

        let info = gfx::texture::Info {
            kind: kind,
            levels: 1,
            format: Surface::get_surface_type(),
            bind: gfx::memory::Bind::SHADER_RESOURCE
                | gfx::memory::Bind::RENDER_TARGET
                | gfx::memory::Bind::TRANSFER_SRC,
            usage: gfx::memory::Usage::Data,
        };

        let channel_type = Channel::get_channel_type();

        let texture = factory
            .create_texture_raw(
                info,
                Some(channel_type),
                Some((&[&rgba], gfx::texture::Mipmap::Provided)),
            )
            .unwrap();

        let descriptor = gfx::texture::ResourceDesc {
            channel: channel_type,
            layer: None,
            min: 0,
            max: texture.get_info().levels - 1,
            swizzle: gfx::format::Swizzle::new(),
        };

        let view = factory
            .view_texture_as_shader_resource_raw(&texture, descriptor)
            .unwrap();

        Texture { texture, view }
    }

    pub(super) fn handle(&self) -> &gfx::handle::RawTexture<gl::Resources> {
        &self.texture
    }

    pub(super) fn view(
        &self,
    ) -> &gfx::handle::RawShaderResourceView<gl::Resources> {
        &self.view
    }
}
