use gfx_device_gl as gl;
use gfx_glyph::GlyphCruncher;

use crate::graphics::gpu::{TargetView, Transformation};
use crate::graphics::{HorizontalAlignment, Text, Vector, VerticalAlignment};

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

    pub fn add(&mut self, text: Text<'_>) {
        let section: gfx_glyph::Section<'_> = text.into();
        self.glyphs.queue(section);
    }

    pub fn measure(&mut self, text: Text<'_>) -> (f32, f32) {
        let section: gfx_glyph::Section<'_> = text.into();
        let bounds = self.glyphs.glyph_bounds(section);

        match bounds {
            Some(bounds) => (bounds.width(), bounds.height()),
            None => (0.0, 0.0),
        }
    }

    pub fn draw(
        &mut self,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        target: &TargetView,
        transformation: Transformation,
    ) {
        let typed_target: gfx::handle::RenderTargetView<
            gl::Resources,
            gfx::format::Srgba8,
        > = gfx::memory::Typed::new(target.clone());

        self.glyphs
            .use_queue()
            .transform(
                Transformation::nonuniform_scale(Vector::new(1.0, -1.0))
                    * transformation,
            )
            .draw(encoder, &typed_target)
            .expect("Font draw");
    }
}

impl<'a> From<Text<'a>> for gfx_glyph::Section<'a> {
    fn from(text: Text<'a>) -> gfx_glyph::Section<'a> {
        let x = match text.horizontal_alignment {
            HorizontalAlignment::Left => text.position.x,
            HorizontalAlignment::Center => {
                text.position.x + text.bounds.0 / 2.0
            }
            HorizontalAlignment::Right => text.position.x + text.bounds.0,
        };

        let y = match text.vertical_alignment {
            VerticalAlignment::Top => text.position.y,
            VerticalAlignment::Center => text.position.y + text.bounds.1 / 2.0,
            VerticalAlignment::Bottom => text.position.y + text.bounds.1,
        };

        gfx_glyph::Section {
            text: &text.content,
            screen_position: (x, y),
            scale: gfx_glyph::Scale {
                x: text.size,
                y: text.size,
            },
            color: text.color.into_linear(),
            bounds: text.bounds,
            layout: gfx_glyph::Layout::default()
                .h_align(text.horizontal_alignment.into())
                .v_align(text.vertical_alignment.into()),
            ..Default::default()
        }
    }
}

impl From<HorizontalAlignment> for gfx_glyph::HorizontalAlign {
    fn from(alignment: HorizontalAlignment) -> gfx_glyph::HorizontalAlign {
        match alignment {
            HorizontalAlignment::Left => gfx_glyph::HorizontalAlign::Left,
            HorizontalAlignment::Center => gfx_glyph::HorizontalAlign::Center,
            HorizontalAlignment::Right => gfx_glyph::HorizontalAlign::Right,
        }
    }
}

impl From<VerticalAlignment> for gfx_glyph::VerticalAlign {
    fn from(alignment: VerticalAlignment) -> gfx_glyph::VerticalAlign {
        match alignment {
            VerticalAlignment::Top => gfx_glyph::VerticalAlign::Top,
            VerticalAlignment::Center => gfx_glyph::VerticalAlign::Center,
            VerticalAlignment::Bottom => gfx_glyph::VerticalAlign::Bottom,
        }
    }
}
