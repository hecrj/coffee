/// An RGBA color in the sRGB color space.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Color {
    /// Red component.
    pub r: f32,

    /// Green component.
    pub g: f32,

    /// Blue component.
    pub b: f32,

    /// Alpha component.
    pub a: f32,
}

impl Color {
    /// White color.
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// Black color.
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Create a new color from components in the [0, 1.0] range.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    /// Create a new color from its RGB components in the [0, 255] range.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Get the color components in the [0, 255] range.
    pub fn to_rgba(&self) -> [u8; 4] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        ]
    }

    pub(crate) fn into_linear(self) -> [f32; 4] {
        // As described in:
        // https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
        fn linear_component(u: f32) -> f32 {
            if u < 0.04045 {
                u / 12.92
            } else {
                ((u + 0.055) / 1.055).powf(2.4)
            }
        }

        [
            linear_component(self.r),
            linear_component(self.g),
            linear_component(self.b),
            self.a,
        ]
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::WHITE
    }
}

impl From<[u8; 3]> for Color {
    fn from(values: [u8; 3]) -> Color {
        let [r, g, b] = values;

        Color::from_rgb(r, g, b)
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> [f32; 4] {
        [color.r, color.g, color.b, color.a]
    }
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> [u8; 4] {
        color.to_rgba()
    }
}
