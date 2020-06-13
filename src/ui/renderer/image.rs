use crate::graphics::{Batch, Image, Rectangle, Sprite};
use crate::ui::{image, Renderer};

impl image::Renderer for Renderer {
    fn draw(
        &mut self,
        bounds: Rectangle<f32>,
        image: Image,
        source: Rectangle<u16>,
    ) {
        let ratio_x = bounds.width / (source.width as f32);
        let ratio_y = bounds.height / (source.height as f32);
        let position = bounds.center();

        let scale = if ratio_x > ratio_y {
            (ratio_y, ratio_y)
        } else {
            (ratio_x, ratio_x)
        };

        let rotation = 0.0;

        let mut batch = Batch::new(image);
        batch.add(Sprite {
            source,
            position,
            rotation,
            scale,
        });

        self.images.push(batch);
    }
}
