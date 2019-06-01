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

    pub fn align_items(mut self, align: Align) -> Self {
        self.0.align_items = align.into();
        self
    }

    pub fn justify_content(mut self, justify: Justify) -> Self {
        self.0.justify_content = justify.into();
        self
    }

    pub fn align_self(mut self, align: Align) -> Self {
        self.0.align_self = align.into();
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

pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[doc(hidden)]
impl From<Align> for style::AlignItems {
    fn from(align: Align) -> Self {
        match align {
            Align::Start => style::AlignItems::FlexStart,
            Align::Center => style::AlignItems::Center,
            Align::End => style::AlignItems::FlexEnd,
            Align::Stretch => style::AlignItems::Stretch,
        }
    }
}

#[doc(hidden)]
impl From<Align> for style::AlignSelf {
    fn from(align: Align) -> Self {
        match align {
            Align::Start => style::AlignSelf::FlexStart,
            Align::Center => style::AlignSelf::Center,
            Align::End => style::AlignSelf::FlexEnd,
            Align::Stretch => style::AlignSelf::Stretch,
        }
    }
}

pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[doc(hidden)]
impl From<Justify> for style::JustifyContent {
    fn from(justify: Justify) -> Self {
        match justify {
            Justify::Start => style::JustifyContent::FlexStart,
            Justify::Center => style::JustifyContent::Center,
            Justify::End => style::JustifyContent::FlexEnd,
            Justify::SpaceBetween => style::JustifyContent::SpaceBetween,
            Justify::SpaceAround => style::JustifyContent::SpaceAround,
            Justify::SpaceEvenly => style::JustifyContent::SpaceEvenly,
        }
    }
}
