use crate::graphics::Text;

pub struct Font {
    //glyphs: gfx_glyph::GlyphBrush<'static, gl::Resources, gl::Factory>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Font {
        Font {}
    }

    pub fn add(&mut self, text: Text) {}
}
