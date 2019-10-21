use std::f32;

use crate::graphics::{Color, Point};

/// A section of text.
#[derive(Clone, PartialEq, Debug)]
pub struct Text<'a> {
    /// Text content
    pub content: &'a str,

    /// Text position
    pub position: Point,

    /// Text bounds, in screen coordinates
    pub bounds: (f32, f32),

    /// Text size
    pub size: f32,

    /// Text color
    pub color: Color,

    /// Text horizontal alignment
    pub horizontal_alignment: HorizontalAlignment,

    /// Text vertical alignment
    pub vertical_alignment: VerticalAlignment,
}

impl Default for Text<'static> {
    #[inline]
    fn default() -> Text<'static> {
        Text {
            content: "",
            position: Point::new(0.0, 0.0),
            bounds: (f32::INFINITY, f32::INFINITY),
            size: 16.0,
            color: Color::BLACK,
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }
}

/// The horizontal alignment of some resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// Align left
    Left,

    /// Horizontally centered
    Center,

    /// Align right
    Right,
}

/// The vertical alignment of some resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align top
    Top,

    /// Vertically centered
    Center,

    /// Align bottom
    Bottom,
}

impl From<HorizontalAlignment> for iced::text::HorizontalAlignment {
    fn from(
        horizontal_alignment: HorizontalAlignment,
    ) -> iced::text::HorizontalAlignment {
        match horizontal_alignment {
            HorizontalAlignment::Left => iced::text::HorizontalAlignment::Left,
            HorizontalAlignment::Center => {
                iced::text::HorizontalAlignment::Center
            }
            HorizontalAlignment::Right => {
                iced::text::HorizontalAlignment::Right
            }
        }
    }
}

impl From<iced::text::HorizontalAlignment> for HorizontalAlignment {
    fn from(
        horizontal_alignment: iced::text::HorizontalAlignment,
    ) -> HorizontalAlignment {
        match horizontal_alignment {
            iced::text::HorizontalAlignment::Left => HorizontalAlignment::Left,
            iced::text::HorizontalAlignment::Center => {
                HorizontalAlignment::Center
            }
            iced::text::HorizontalAlignment::Right => {
                HorizontalAlignment::Right
            }
        }
    }
}

impl From<VerticalAlignment> for iced::text::VerticalAlignment {
    fn from(
        vertical_alignment: VerticalAlignment,
    ) -> iced::text::VerticalAlignment {
        match vertical_alignment {
            VerticalAlignment::Top => iced::text::VerticalAlignment::Top,
            VerticalAlignment::Center => iced::text::VerticalAlignment::Center,
            VerticalAlignment::Bottom => iced::text::VerticalAlignment::Bottom,
        }
    }
}

impl From<iced::text::VerticalAlignment> for VerticalAlignment {
    fn from(
        vertical_alignment: iced::text::VerticalAlignment,
    ) -> VerticalAlignment {
        match vertical_alignment {
            iced::text::VerticalAlignment::Top => VerticalAlignment::Top,
            iced::text::VerticalAlignment::Center => VerticalAlignment::Center,
            iced::text::VerticalAlignment::Bottom => VerticalAlignment::Bottom,
        }
    }
}
