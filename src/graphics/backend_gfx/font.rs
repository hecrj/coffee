use gfx;
use gfx_device_gl as gl;
use gfx_glyph;

use crate::graphics::gpu::{DepthView, TargetView};
use crate::graphics::Text;

pub struct Font {
    glyphs: gfx_glyph::GlyphBrush<'static, gl::Resources, gl::Factory>,
}

impl Font {
    pub fn from_bytes(factory: &mut gl::Factory, bytes: &'static [u8]) -> Font {
        Font {
            glyphs: gfx_glyph::GlyphBrushBuilder::using_font_bytes(bytes)
                .depth_test(gfx::preset::depth::PASS_TEST)
                .texture_filter_method(gfx::texture::FilterMethod::Scale)
                .build(factory.clone()),
        }
    }

    pub fn add(&mut self, text: Text) {
        self.glyphs.queue(gfx_glyph::Section {
            text: &text.content,
            screen_position: (text.position.x, text.position.y),
            scale: gfx_glyph::Scale {
                x: text.size,
                y: text.size,
            },
            color: text.color.into(),
            bounds: text.bounds,
            ..Default::default()
        });
    }

    pub fn draw(
        &mut self,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        target: &TargetView,
        depth: &DepthView,
    ) {
        let typed_target: gfx::handle::RenderTargetView<
            gl::Resources,
            gfx::format::Srgba8,
        > = gfx::memory::Typed::new(target.clone());

        let typed_depth: gfx::handle::DepthStencilView<
            gl::Resources,
            gfx::format::Depth,
        > = gfx::memory::Typed::new(depth.clone());

        self.glyphs
            .draw_queued(encoder, &typed_target, &typed_depth)
            .expect("gfx_glyph draw_queued failed");
    }
}
