use crate::graphics::gpu::TargetView;
use crate::graphics::{Text, Transformation};

pub struct Font {
    glyphs: wgpu_glyph::GlyphBrush<'static>,
}

impl Font {
    pub fn from_bytes(device: &mut wgpu::Device, bytes: &'static [u8]) -> Font {
        Font {
            glyphs: wgpu_glyph::GlyphBrushBuilder::using_font_bytes(bytes)
                .texture_filter_method(wgpu::FilterMode::Nearest)
                .build(device, wgpu::TextureFormat::Bgra8UnormSrgb),
        }
    }

    pub fn add(&mut self, text: Text) {
        self.glyphs.queue(wgpu_glyph::Section {
            text: &text.content,
            screen_position: (text.position.x, text.position.y),
            scale: wgpu_glyph::Scale {
                x: text.size,
                y: text.size,
            },
            color: text.color.into_linear(),
            bounds: text.bounds,
            ..Default::default()
        });
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
                transformation.into(),
                device,
                encoder,
                target,
            )
            .expect("Draw font");
    }
}
