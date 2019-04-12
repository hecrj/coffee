use crate::graphics::Text;

pub struct Font {
    //glyphs: gfx_glyph::GlyphBrush<'static, gl::Resources, gl::Factory>,
}

impl Font {
    pub fn from_bytes(_bytes: &'static [u8]) -> Font {
        Font {}
    }

    pub fn add(&mut self, _text: Text) {}
}
