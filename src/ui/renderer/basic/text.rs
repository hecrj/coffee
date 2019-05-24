use super::Basic;

use crate::graphics::{Color, Point, Rectangle, Text};
use crate::ui::{text, Node, Number, Size, Style};

use std::cell::RefCell;
use std::f32;

impl text::Renderer for Basic {
    fn node(&self, style: Style, content: &str, size: f32) -> Node {
        let font = self.font.clone();
        let content = String::from(content);
        let measure = RefCell::new(None);

        Node::new_leaf(style, move |bounds| {
            // TODO: Investigate why stretch tries to measure this MANY times
            // with every ancestor's bounds.
            // Bug? Using the library wrong? I should probably open an issue on
            // the stretch repository.
            // I noticed that the first measure is the one that matters in
            // practice. Here, we simply use a RefCell to store the cached
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

                let text = Text {
                    content: &content,
                    size,
                    bounds,
                    ..Text::default()
                };

                let (width, height) = font.borrow_mut().measure(text);

                let size = Size {
                    width,
                    height: height + size / 4.0,
                };

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
        content: &str,
        size: f32,
        color: Color,
        bounds: Rectangle<f32>,
    ) {
        self.font.borrow_mut().add(Text {
            content,
            position: Point::new(bounds.x, bounds.y),
            bounds: (bounds.width, bounds.height),
            color,
            size,
            ..Text::default()
        });
    }
}
