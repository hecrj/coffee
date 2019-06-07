use crate::graphics::{Point, Rectangle, Sprite};
use crate::ui::core::MouseCursor;
use crate::ui::{slider, Renderer};

use std::ops::RangeInclusive;

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

impl slider::Renderer for Renderer {
    fn draw(
        &mut self,
        cursor_position: Point,
        bounds: Rectangle<f32>,
        state: &slider::State,
        range: RangeInclusive<f32>,
        value: f32,
    ) -> MouseCursor {
        self.sprites.add(Sprite {
            source: RAIL,
            position: Point::new(
                bounds.x + MARKER.width as f32 / 2.0,
                bounds.y + 12.5,
            ),
            scale: (bounds.width - MARKER.width as f32, 1.0),
        });

        let (range_start, range_end) = range.into_inner();

        let marker_offset = (bounds.width - MARKER.width as f32)
            * ((value - range_start) / (range_end - range_start).max(1.0));

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
            MouseCursor::OutOfBounds
        }
    }
}
