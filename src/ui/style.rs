use stretch::style;

#[derive(Clone, Copy)]
pub struct Style(pub(crate) style::Style);

impl Style {
    pub fn width(mut self, width: f32) -> Self {
        self.0.size.width = style::Dimension::Points(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.0.size.height = style::Dimension::Points(height);
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.0.size.width = stretch::style::Dimension::Percent(1.0);
        self.0.max_size.width = style::Dimension::Points(max_width);
        self
    }

    pub fn grow(mut self) -> Self {
        self.0.flex_grow = 1.0;
        self
    }

    pub fn padding(mut self, px: u32) -> Self {
        self.0.padding = stretch::geometry::Rect {
            start: style::Dimension::Points(px as f32),
            end: style::Dimension::Points(px as f32),
            top: style::Dimension::Points(px as f32),
            bottom: style::Dimension::Points(px as f32),
        };

        self
    }

    pub fn center_children(mut self) -> Self {
        self.0.align_items = style::AlignItems::Center;
        self.0.justify_content = style::JustifyContent::Center;
        self
    }
}

impl Default for Style {
    fn default() -> Style {
        Style(style::Style {
            ..style::Style::default()
        })
    }
}
