use crate::graphics::gpu::TargetView;
use crate::graphics::{
    HorizontalAlignment, Text, Transformation, VerticalAlignment,
};

use wgpu_glyph::GlyphCruncher;

pub struct Font {
    glyphs: wgpu_glyph::GlyphBrush<'static, ()>,
}

impl Font {
    pub fn from_bytes(device: &mut wgpu::Device, bytes: &'static [u8]) -> Font {
        Font {
            glyphs: wgpu_glyph::GlyphBrushBuilder::using_font_bytes(bytes)
                .expect("Load font")
                .texture_filter_method(wgpu::FilterMode::Nearest)
                .build(device, wgpu::TextureFormat::Bgra8UnormSrgb),
        }
    }

    pub fn add(&mut self, text: Text<'_>) {
        let section: wgpu_glyph::Section<'_> = text.into();
        self.glyphs.queue(section);
    }

    pub fn measure(&mut self, text: Text<'_>) -> (f32, f32) {
        let section: wgpu_glyph::Section<'_> = text.into();
        let bounds = self.glyphs.glyph_bounds(section);

        match bounds {
            Some(bounds) => (bounds.width(), bounds.height()),
            None => (0.0, 0.0),
        }
    }

    pub fn draw(
        &mut self,
        device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        target: &TargetView,
        transformation: Transformation,
    ) {
        self.glyphs
            .draw_queued_with_transform(
                device,
                encoder,
                target,
                transformation.into(),
            )
            .expect("Draw font");
    }
}

impl<'a> From<Text<'a>> for wgpu_glyph::Section<'a> {
    fn from(text: Text<'a>) -> wgpu_glyph::Section<'a> {
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

        wgpu_glyph::Section {
            text: &text.content,
            screen_position: (x, y),
            scale: wgpu_glyph::Scale {
                x: text.size,
                y: text.size,
            },
            color: text.color.into_linear(),
            bounds: text.bounds,
            layout: wgpu_glyph::Layout::default()
                .h_align(text.horizontal_alignment.into())
                .v_align(text.vertical_alignment.into()),
            ..Default::default()
        }
    }
}

impl From<HorizontalAlignment> for wgpu_glyph::HorizontalAlign {
    fn from(alignment: HorizontalAlignment) -> wgpu_glyph::HorizontalAlign {
        match alignment {
            HorizontalAlignment::Left => wgpu_glyph::HorizontalAlign::Left,
            HorizontalAlignment::Center => wgpu_glyph::HorizontalAlign::Center,
            HorizontalAlignment::Right => wgpu_glyph::HorizontalAlign::Right,
        }
    }
}

impl From<VerticalAlignment> for wgpu_glyph::VerticalAlign {
    fn from(alignment: VerticalAlignment) -> wgpu_glyph::VerticalAlign {
        match alignment {
            VerticalAlignment::Top => wgpu_glyph::VerticalAlign::Top,
            VerticalAlignment::Center => wgpu_glyph::VerticalAlign::Center,
            VerticalAlignment::Bottom => wgpu_glyph::VerticalAlign::Bottom,
        }
    }
}
