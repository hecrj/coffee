use gfx_device_gl as gl;

use super::format;

pub type TargetView = gfx::handle::RawRenderTargetView<gl::Resources>;

pub type RawTexture = gfx::handle::RawTexture<gl::Resources>;

pub type ShaderResource =
    gfx::handle::ShaderResourceView<gl::Resources, format::View>;
