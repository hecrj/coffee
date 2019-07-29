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

    /// Red color.
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Green color.
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    /// Blue color.
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    /// Creates a new [`Color`] from components in the [0, 1.0] range.
    ///
    /// [`Color`]: struct.Color.html
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        debug_assert!(r >= 0.0, "Red component is < 0.0");
        debug_assert!(r <= 1.0, "Red component is > 1.0");
        debug_assert!(g >= 0.0, "Green component is < 0.0");
        debug_assert!(g <= 1.0, "Green component is > 1.0");
        debug_assert!(b >= 0.0, "Blue component is < 0.0");
        debug_assert!(b <= 1.0, "Blue component is > 1.0");
        debug_assert!(a >= 0.0, "Alpha component is < 0.0");
        debug_assert!(a <= 1.0, "Alpha component is > 1.0");
        Color { r, g, b, a }
    }

    /// Creates a new [`Color`] from its RGB components in the [0, 255] range.
    ///
    /// [`Color`]: struct.Color.html
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Creates a new [`Color`] from its RGB representation (0xRRGGBB).
    ///
    /// [`Color`]: struct.Color.html
    pub fn from_rgb_u32(color: u32) -> Color {
        debug_assert!(
            color <= 0xFFFFFF,
            "Color contains value higher than 0xFFFFFF"
        );
        let r = ((color & 0xFF0000) >> 16) as u8;
        let g = ((color & 0x00FF00) >> 8) as u8;
        let b = ((color & 0x0000FF) >> 0) as u8;
        Color::from_rgb(r, g, b)
    }

    /// Returns the [`Color`] components in the [0, 255] range.
    ///
    /// [`Color`]: struct.Color.html
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
