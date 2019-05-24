use std::hash::{Hash, Hasher};
use stretch::{geometry, style};

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

    pub fn min_width(mut self, min_width: f32) -> Self {
        self.0.min_size.width = style::Dimension::Points(min_width);
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.0.max_size.width = style::Dimension::Points(max_width);
        self.fill_width()
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.0.max_size.height = style::Dimension::Points(max_height);
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.0.size.width = stretch::style::Dimension::Percent(1.0);
        self
    }

    pub fn grow(mut self) -> Self {
        self.0.flex_grow = 1.0;
        self
    }

    pub fn shrink(mut self) -> Self {
        self.0.align_self = style::AlignSelf::Auto;
        self
    }

    pub fn align_left(mut self) -> Self {
        self.0.align_self = style::AlignSelf::FlexStart;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.0.align_self = style::AlignSelf::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.0.align_self = style::AlignSelf::FlexEnd;
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

    pub fn padding_top(mut self, px: u32) -> Self {
        self.0.padding.top = style::Dimension::Points(px as f32);

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
            align_items: style::AlignItems::Stretch,
            justify_content: style::JustifyContent::FlexStart,
            ..style::Style::default()
        })
    }
}

impl Hash for Style {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_size(&self.0.size, state);
        hash_size(&self.0.min_size, state);
        hash_size(&self.0.max_size, state);

        hash_rect(&self.0.margin, state);

        (self.0.flex_direction as u8).hash(state);
        (self.0.align_items as u8).hash(state);
        (self.0.justify_content as u8).hash(state);
        (self.0.align_self as u8).hash(state);
        (self.0.flex_grow as u32).hash(state);
    }
}

fn hash_size<H: Hasher>(
    size: &geometry::Size<style::Dimension>,
    state: &mut H,
) {
    hash_dimension(size.width, state);
    hash_dimension(size.height, state);
}

fn hash_rect<H: Hasher>(
    rect: &geometry::Rect<style::Dimension>,
    state: &mut H,
) {
    hash_dimension(rect.start, state);
    hash_dimension(rect.end, state);
    hash_dimension(rect.top, state);
    hash_dimension(rect.bottom, state);
}

fn hash_dimension<H: Hasher>(dimension: style::Dimension, state: &mut H) {
    match dimension {
        style::Dimension::Undefined => state.write_u8(0),
        style::Dimension::Auto => state.write_u8(1),
        style::Dimension::Points(points) => {
            state.write_u8(2);
            (points as u32).hash(state);
        }
        style::Dimension::Percent(percent) => {
            state.write_u8(3);
            (percent as u32).hash(state);
        }
    }
}
