use crate::graphics::{Point, Rectangle, Sprite};
use crate::ui::core::MouseCursor;
use crate::ui::widget::checkbox;
use crate::ui::Renderer;

const SPRITE: Rectangle<u16> = Rectangle {
    x: 98,
    y: 0,
    width: 28,
    height: 28,
};

impl checkbox::Renderer for Renderer {
    fn draw(
        &mut self,
        cursor_position: Point,
        bounds: Rectangle<f32>,
        text_bounds: Rectangle<f32>,
        is_checked: bool,
    ) -> MouseCursor {
        let mouse_over = bounds.contains(cursor_position)
            || text_bounds.contains(cursor_position);

        self.sprites.add(Sprite {
            source: Rectangle {
                x: SPRITE.x + (if mouse_over { SPRITE.width } else { 0 }),
                ..SPRITE
            },
            position: Point::new(bounds.x, bounds.y),
            scale: (1.0, 1.0),
        });

        if is_checked {
            self.sprites.add(Sprite {
                source: Rectangle {
                    x: SPRITE.x + SPRITE.width * 2,
                    ..SPRITE
                },
                position: Point::new(bounds.x, bounds.y),
                scale: (1.0, 1.0),
            });
        }

        if mouse_over {
            MouseCursor::Pointer
        } else {
            MouseCursor::OutOfBounds
        }
    }
}
