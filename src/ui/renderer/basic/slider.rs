use std::ops::Range;

use super::Basic;
use crate::graphics::{
    Color, HorizontalAlignment, Point, Quad, Rectangle, Sprite, Text,
    VerticalAlignment,
};
use crate::ui::{slider, MouseCursor};

const RAIL: Rectangle<u16> = Rectangle {
    x: 98,
    y: 56,
    width: 1,
    height: 4,
};

const MARKER: Rectangle<u16> = Rectangle {
    x: RAIL.x + 28,
    y: RAIL.y,
    width: 16,
    height: 24,
};

impl slider::Renderer for Basic {
    fn draw(
        &mut self,
        state: &slider::State,
        range: &Range<f32>,
        value: f32,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor {
        self.sprites.add(Sprite {
            source: RAIL,
            position: Point::new(
                bounds.x + MARKER.width as f32 / 2.0,
                bounds.y + 12.5,
            ),
            scale: (bounds.width - MARKER.width as f32, 1.0),
        });

        let marker_offset = (bounds.width - MARKER.width as f32)
            * ((value - range.start) / (range.end - range.start).max(1.0));

        let mouse_over = bounds.contains(cursor_position);
        let is_active = state.is_dragging() || mouse_over;

        self.sprites.add(Sprite {
            source: Rectangle {
                x: MARKER.x + (if is_active { MARKER.width } else { 0 }),
                ..MARKER
            },
            position: Point::new(
                bounds.x + marker_offset.round(),
                bounds.y + (if state.is_dragging() { 2.0 } else { 0.0 }),
            ),
            scale: (1.0, 1.0),
        });

        if state.is_dragging() {
            MouseCursor::Grabbing
        } else if mouse_over {
            MouseCursor::Grab
        } else {
            MouseCursor::Default
        }
    }
}
