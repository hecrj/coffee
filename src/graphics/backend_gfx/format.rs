pub const COLOR: gfx::format::Format = gfx::format::Format(
    gfx::format::SurfaceType::R8_G8_B8_A8,
    gfx::format::ChannelType::Unorm,
);

pub const DEPTH: gfx::format::Format = gfx::format::Format(
    gfx::format::SurfaceType::D24_S8,
    gfx::format::ChannelType::Unorm,
);

pub type View = <gfx::format::Srgba8 as gfx::format::Formatted>::View;
pub type Surface = <gfx::format::Srgba8 as gfx::format::Formatted>::Surface;
pub type Channel = <gfx::format::Srgba8 as gfx::format::Formatted>::Channel;
