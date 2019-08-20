use crate::graphics::{Rectangle, Sprite, Point, Image, Batch};
use crate::ui::{Renderer, image};

impl image::Renderer for Renderer {
    fn draw(
        &mut self,
        bounds: Rectangle<f32>,
        image: Image,
        source: Rectangle<u16>,
    ) {
        let ratio_x = bounds.width / (source.width as f32);
        let ratio_y = bounds.height / (source.height as f32);
        let center = bounds.center();

        let (scale, position) = if ratio_x > ratio_y {
            let position_x = center.x - source.width as f32 * ratio_y / 2.0;
            let position_y = bounds.y;
            ((ratio_y, ratio_y), Point::new(position_x, position_y))
        } else {
            let position_x = bounds.x;
            let position_y = center.y - source.height as f32 * ratio_x / 2.0;
            ((ratio_x, ratio_x), Point::new(position_x, position_y))
        };

        let mut batch = Batch::new(image); 
        batch.add(Sprite {
            source,
            position,
            scale,
        });

        self.images.push(batch);
    }
}

