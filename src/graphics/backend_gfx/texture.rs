use image;

use gfx::format::{ChannelTyped, SurfaceTyped};
use gfx_core::factory::Factory;
use gfx_device_gl as gl;

use super::format::{Channel, Surface};
use super::types::{RawTexture, ShaderResource, TargetView};
use crate::graphics::Transformation;

#[derive(Clone, Debug)]
pub struct Texture {
    texture: RawTexture,
    view: ShaderResource,
    width: u16,
    height: u16,
    layers: u16,
}

impl Texture {
    pub(super) fn new(
        factory: &mut gl::Factory,
        image: &image::DynamicImage,
    ) -> Texture {
        let rgba = image.to_rgba();
        let width = rgba.width() as u16;
        let height = rgba.height() as u16;

        let (texture, view) = create_texture_array(
            factory,
            width,
            height,
            Some(&[&rgba]),
            gfx::memory::Bind::SHADER_RESOURCE
                | gfx::memory::Bind::TRANSFER_SRC,
        );

        Texture {
            texture,
            view,
            width,
            height,
            layers: 1,
        }
    }

    pub(super) fn new_array(
        factory: &mut gl::Factory,
        layers: &[image::DynamicImage],
    ) -> Texture {
        let first_layer = &layers[0].to_rgba();
        let width = first_layer.width() as u16;
        let height = first_layer.height() as u16;

        let rgba: Vec<Vec<u8>> =
            layers.iter().map(|i| i.to_rgba().into_raw()).collect();

        let raw_layers: Vec<&[u8]> = rgba.iter().map(|i| &i[..]).collect();

        let (texture, view) = create_texture_array(
            factory,
            width,
            height,
            Some(&raw_layers[..]),
            gfx::memory::Bind::SHADER_RESOURCE
                | gfx::memory::Bind::TRANSFER_SRC,
        );

        Texture {
            texture,
            view,
            width,
            height,
            layers: layers.len() as u16,
        }
    }

    pub(super) fn handle(&self) -> &RawTexture {
        &self.texture
    }

    pub(super) fn view(&self) -> &ShaderResource {
        &self.view
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Clone)]
pub struct Drawable {
    texture: Texture,
    target: TargetView,
}

impl Drawable {
    pub fn new(factory: &mut gl::Factory, width: u16, height: u16) -> Drawable {
        let (texture, view) = create_texture_array(
            factory,
            width,
            height,
            None,
            gfx::memory::Bind::SHADER_RESOURCE
                | gfx::memory::Bind::RENDER_TARGET,
        );

        let texture = Texture {
            texture,
            view,
            width,
            height,
            layers: 1,
        };

        let render_desc = gfx::texture::RenderDesc {
            channel: Channel::get_channel_type(),
            level: 0,
            layer: Some(0),
        };

        let target = factory
            .view_texture_as_render_target_raw(texture.handle(), render_desc)
            .unwrap();

        Drawable { texture, target }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn target(&self) -> &TargetView {
        &self.target
    }

    pub fn render_transformation() -> Transformation {
        Transformation::nonuniform_scale(1.0, -1.0)
    }
}

// Helpers
fn create_texture_array(
    factory: &mut gl::Factory,
    width: u16,
    height: u16,
    layers: Option<&[&[u8]]>,
    bind: gfx::memory::Bind,
) -> (RawTexture, ShaderResource) {
    let kind = gfx::texture::Kind::D2Array(
        width,
        height,
        layers.map(|l| l.len()).unwrap_or(1) as u16,
        gfx::texture::AaMode::Single,
    );

    let info = gfx::texture::Info {
        kind: kind,
        levels: 1,
        format: Surface::get_surface_type(),
        bind: bind,
        usage: gfx::memory::Usage::Data,
    };

    let channel_type = Channel::get_channel_type();

    let texture = factory
        .create_texture_raw(
            info,
            Some(channel_type),
            layers.map(|l| (l, gfx::texture::Mipmap::Provided)),
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

    let typed_view: gfx::handle::ShaderResourceView<
        _,
        <gfx::format::Srgba8 as gfx::format::Formatted>::View,
    > = gfx::memory::Typed::new(view);

    (texture, typed_view)
}
