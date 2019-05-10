use stretch::style;

#[derive(Default, Clone, Copy)]
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

    pub fn grow(mut self) -> Self {
        self.0.flex_grow = 1.0;
        self
    }

    pub fn center_children(mut self) -> Self {
        self.0.align_items = style::AlignItems::Center;
        self.0.justify_content = style::JustifyContent::Center;
        self
    }
}
