use crate::graphics::{Point, Rectangle, Sprite};
use crate::ui::widget::panel;
use crate::ui::Renderer;

const PANEL_WIDTH: u16 = 28;
const PANEL_HEIGHT: u16 = 34;

const TOP_LEFT: Rectangle<u16> = Rectangle {
    x: 0,
    y: 0,
    width: 8,
    height: 8,
};

const TOP_BORDER: Rectangle<u16> = Rectangle {
    x: TOP_LEFT.width,
    y: 0,
    width: 1,
    height: TOP_LEFT.height,
};

const TOP_RIGHT: Rectangle<u16> = Rectangle {
    x: PANEL_WIDTH - TOP_LEFT.height,
    y: 0,
    width: TOP_LEFT.width,
    height: TOP_LEFT.height,
};

const CONTENT_BACKGROUND: Rectangle<u16> = Rectangle {
    x: TOP_LEFT.width,
    y: TOP_LEFT.height,
    width: 1,
    height: 1,
};

const LEFT_BORDER: Rectangle<u16> = Rectangle {
    x: TOP_LEFT.x,
    y: TOP_LEFT.height,
    width: TOP_LEFT.width,
    height: 1,
};

const RIGHT_BORDER: Rectangle<u16> = Rectangle {
    x: TOP_RIGHT.x,
    y: TOP_RIGHT.height,
    width: TOP_RIGHT.width,
    height: 1,
};

const BOTTOM_LEFT: Rectangle<u16> = Rectangle {
    x: TOP_LEFT.x,
    y: PANEL_HEIGHT - TOP_LEFT.height,
    width: TOP_LEFT.width,
    height: TOP_LEFT.height,
};

const BOTTOM_BORDER: Rectangle<u16> = Rectangle {
    x: TOP_BORDER.x,
    y: PANEL_HEIGHT - TOP_BORDER.height,
    width: 1,
    height: TOP_BORDER.height,
};

const BOTTOM_RIGHT: Rectangle<u16> = Rectangle {
    x: TOP_RIGHT.x,
    y: PANEL_HEIGHT - TOP_RIGHT.height,
    width: TOP_RIGHT.width,
    height: TOP_RIGHT.height,
};

impl panel::Renderer for Renderer {
    fn draw(&mut self, bounds: Rectangle<f32>) {
        self.sprites.add(Sprite {
            source: TOP_LEFT,
            position: Point::new(bounds.x, bounds.y),
            ..Sprite::default()
        });

        self.sprites.add(Sprite {
            source: TOP_BORDER,
            position: Point::new(bounds.x + TOP_LEFT.width as f32, bounds.y),
            scale: (
                bounds.width - (TOP_LEFT.width + TOP_RIGHT.width) as f32,
                1.0,
            ),
        });

        self.sprites.add(Sprite {
            source: TOP_RIGHT,
            position: Point::new(
                bounds.x + bounds.width - TOP_RIGHT.width as f32,
                bounds.y,
            ),
            ..Sprite::default()
        });

        self.sprites.add(Sprite {
            source: CONTENT_BACKGROUND,
            position: Point::new(bounds.x, bounds.y + TOP_BORDER.height as f32),
            scale: (
                bounds.width,
                bounds.height
                    - (TOP_BORDER.height + BOTTOM_BORDER.height) as f32,
            ),
        });

        self.sprites.add(Sprite {
            source: LEFT_BORDER,
            position: Point::new(bounds.x, bounds.y + TOP_BORDER.height as f32),
            scale: (
                1.0,
                bounds.height - (TOP_BORDER.height + BOTTOM_LEFT.height) as f32,
            ),
        });

        self.sprites.add(Sprite {
            source: RIGHT_BORDER,
            position: Point::new(
                bounds.x + bounds.width - RIGHT_BORDER.width as f32,
                bounds.y + TOP_BORDER.height as f32,
            ),
            scale: (
                1.0,
                bounds.height
                    - (TOP_BORDER.height + BOTTOM_RIGHT.height) as f32,
            ),
        });

        self.sprites.add(Sprite {
            source: BOTTOM_LEFT,
            position: Point::new(
                bounds.x,
                bounds.y + bounds.height - BOTTOM_LEFT.height as f32,
            ),
            ..Sprite::default()
        });

        self.sprites.add(Sprite {
            source: BOTTOM_BORDER,
            position: Point::new(
                bounds.x + BOTTOM_LEFT.width as f32,
                bounds.y + bounds.height - BOTTOM_BORDER.height as f32,
            ),
            scale: (
                bounds.width - (BOTTOM_LEFT.width + BOTTOM_LEFT.width) as f32,
                1.0,
            ),
        });

        self.sprites.add(Sprite {
            source: BOTTOM_RIGHT,
            position: Point::new(
                bounds.x + bounds.width - BOTTOM_RIGHT.width as f32,
                bounds.y + bounds.height - BOTTOM_RIGHT.height as f32,
            ),
            ..Sprite::default()
        });
    }
}
