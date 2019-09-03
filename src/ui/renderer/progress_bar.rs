use crate::graphics::{Rectangle, Sprite, Point};
use crate::ui::{progress_bar, Renderer};

const LEFT: Rectangle<u16> = Rectangle {
    x: 0,
    y: 34,
    width: 6,
    height: 49,
};

const BACKGROUND: Rectangle<u16> = Rectangle {
    x: LEFT.width,
    y: LEFT.y,
    width: 1,
    height: LEFT.height,
};

const RIGHT: Rectangle<u16> = Rectangle {
    x: LEFT.height - LEFT.width,
    y: LEFT.y,
    width: LEFT.width,
    height: LEFT.height,
};

impl progress_bar::Renderer for Renderer {
    fn draw(
        &mut self,
        bounds: Rectangle<f32>,
        progress: f32,
    ) {
        let active_class = 0;
        let background_class = 1;
        let full = 1.0;
        let left_width_f32 = LEFT.width as f32 / 100.0;
        let background_width = 1.0 - 2.0 * left_width_f32;

        self.sprites.add(left_sprite(bounds, background_class, full));
        self.sprites.add(background_sprite(bounds, background_class, full));
        self.sprites.add(right_sprite(bounds, background_class, full));

        if progress > 0.0 {
            let area = bound(progress / left_width_f32);
            self.sprites.add(left_sprite(bounds, active_class, area));
        }

        if progress > left_width_f32 {
            let area = bound((progress - left_width_f32) / background_width);
            self.sprites.add(background_sprite(bounds, active_class, area));
        }

        if progress > left_width_f32 + background_width {
            let area = bound((progress - left_width_f32 - background_width) / left_width_f32);
            self.sprites.add(right_sprite(bounds, active_class, area));
        }
    }
}

fn bound(v: f32) -> f32 {
    if v > 1.0 {
        1.0
    } else {
        v
    }
}

fn left_sprite(bounds: Rectangle<f32>, class_index: u16, area: f32) -> Sprite {
    Sprite {
        source: Rectangle {
            x: LEFT.x,
            y: LEFT.y + class_index * LEFT.height,
            width: (LEFT.width as f32 * area) as u16,
            height: LEFT.height,
        },
        position: Point::new(bounds.x, bounds.y),
        scale: (1.0, 1.0),
    }
}

fn background_sprite(bounds: Rectangle<f32>, class_index: u16, area: f32) -> Sprite {
    Sprite {
        source: Rectangle {
            x: BACKGROUND.x,
            y: BACKGROUND.y + class_index * BACKGROUND.height,
            ..BACKGROUND
        },
        position: Point::new(bounds.x + LEFT.width as f32, bounds.y),
        scale: ((bounds.width - (LEFT.width + RIGHT.width) as f32) * area, 1.0),
    }
}

fn right_sprite(bounds: Rectangle<f32>, class_index: u16, area: f32) -> Sprite {
    Sprite {
        source: Rectangle {
            x: RIGHT.x,
            y: RIGHT.y + class_index * RIGHT.height,
            width: (RIGHT.width as f32 * area) as u16,
            height: RIGHT.height,
        },
        position: Point::new(
            bounds.x + bounds.width - RIGHT.width as f32,
            bounds.y,
        ),
        scale: (1.0, 1.0),
    }
}
