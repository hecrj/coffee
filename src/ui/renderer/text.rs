use crate::graphics::{self, Color, Point};
use crate::ui::core::{Node, Number, Size, Style};
use crate::ui::Renderer;

use iced::text;
use std::cell::RefCell;
use std::f32;

impl text::Renderer<Color> for Renderer {
    fn node(&self, style: Style, content: &str, size: Option<u16>) -> Node {
        let font = self.font.clone();
        let content = String::from(content);
        let size = size.unwrap_or(20);
        let measure = RefCell::new(None);

        Node::with_measure(style, move |bounds| {
            // TODO: Investigate why stretch tries to measure this MANY times
            // with every ancestor's bounds.
            // Bug? Using the library wrong? I should probably open an issue on
            // the stretch repository.
            // I noticed that the first measure is the one that matters in
            // practice. Here, we use a RefCell to store the cached
            // measurement.
            let mut measure = measure.borrow_mut();

            if measure.is_none() {
                let bounds = (
                    match bounds.width {
                        Number::Undefined => f32::INFINITY,
                        Number::Defined(w) => w,
                    },
                    match bounds.height {
                        Number::Undefined => f32::INFINITY,
                        Number::Defined(h) => h,
                    },
                );

                let text = graphics::Text {
                    content: &content,
                    size: f32::from(size),
                    bounds,
                    ..graphics::Text::default()
                };

                let (width, height) = font.borrow_mut().measure(text);

                let size = Size { width, height };

                // If the text has no width boundary we avoid caching as the
                // layout engine may just be measuring text in a row.
                if bounds.0 == f32::INFINITY {
                    return size;
                } else {
                    *measure = Some(size);
                }
            }

            measure.unwrap()
        })
    }

    fn draw(
        &mut self,
        bounds: iced::Rectangle,
        content: &str,
        size: Option<u16>,
        color: Option<Color>,
        horizontal_alignment: text::HorizontalAlignment,
        vertical_alignment: text::VerticalAlignment,
    ) {
        self.font.borrow_mut().add(graphics::Text {
            content,
            position: Point::new(bounds.x, bounds.y),
            bounds: (bounds.width, bounds.height),
            color: color.unwrap_or(Color::WHITE),
            size: f32::from(size.unwrap_or(20)),
            horizontal_alignment: horizontal_alignment.into(),
            vertical_alignment: vertical_alignment.into(),
        });
    }
}
