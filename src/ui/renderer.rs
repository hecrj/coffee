use crate::graphics::{
    Batch, Color, Font, Image, Point, Quad, Rectangle, Text, Window,
};
use crate::load::{Join, Task};
use crate::ui::{button, column, text, MouseCursor, Node, Number, Size, Style};

use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

pub trait Renderer {
    fn load() -> Task<Self>
    where
        Self: Sized;

    fn flush(&mut self, window: &mut Window);
}

pub struct Basic {
    batch: Batch,
    font: Rc<RefCell<Font>>,
}

impl Renderer for Basic {
    fn load() -> Task<Basic> {
        let load_batch = Task::using_gpu(|gpu| {
            let image = Image::from_colors(
                gpu,
                &[
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 0.02,
                    },
                    Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.5,
                        a: 1.0,
                    },
                ],
            )?;

            Ok(Batch::new(image))
        });

        let load_font = Font::load(Font::DEFAULT);

        (load_batch, load_font).join().map(|(batch, font)| Basic {
            batch,
            font: Rc::new(RefCell::new(font)),
        })
    }

    fn flush(&mut self, window: &mut Window) {
        let mut frame = window.frame();
        let target = &mut frame.as_target();

        self.batch.draw(Point::new(0.0, 0.0), target);
        self.batch.clear();

        self.font.borrow_mut().draw(target);
    }
}

impl column::Renderer for Basic {
    fn draw(&mut self, bounds: Rectangle<f32>) {
        self.batch.add(Quad {
            source: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.5,
                height: 1.0,
            },
            position: Point::new(bounds.x, bounds.y),
            size: (bounds.width, bounds.height),
        });
    }
}

impl button::Renderer for Basic {
    fn draw(
        &mut self,
        _state: &button::State,
        label: &str,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor {
        let mouse_over = bounds.contains(cursor_position);

        self.batch.add(Quad {
            source: Rectangle {
                x: 0.5,
                y: 0.0,
                width: 0.5,
                height: 1.0,
            },
            position: Point::new(bounds.x, bounds.y),
            size: (bounds.width, bounds.height),
        });

        self.font.borrow_mut().add(Text {
            content: String::from(label),
            position: Point::new(bounds.x, bounds.y),
            bounds: (bounds.width, bounds.height),
            color: if mouse_over {
                Color::BLACK
            } else {
                Color::WHITE
            },
            size: 20.0,
            ..Text::default()
        });

        if mouse_over {
            MouseCursor::Pointer
        } else {
            MouseCursor::Default
        }
    }
}

impl text::Renderer for Basic {
    fn node(&self, style: Style, content: &str) -> Node {
        let content = String::from(content);
        let font = self.font.clone();

        Node::new_leaf(style, move |size| {
            let bounds = (
                match size.width {
                    Number::Undefined => f32::INFINITY,
                    Number::Defined(w) => w,
                },
                match size.height {
                    Number::Undefined => f32::INFINITY,
                    Number::Defined(h) => h,
                },
            );

            let text = Text {
                content: content.clone(),
                position: Point::new(0.0, 0.0),
                color: Color::WHITE,
                size: 20.0,
                bounds,
                ..Text::default()
            };

            let (width, height) = font.borrow_mut().measure(text);

            Size {
                width,
                height: height + 5.0,
            }
        })
    }

    fn draw(
        &mut self,
        content: &str,
        bounds: Rectangle<f32>,
        _cursor_position: Point,
    ) -> MouseCursor {
        self.font.borrow_mut().add(Text {
            content: String::from(content),
            position: Point::new(bounds.x, bounds.y),
            bounds: (bounds.width, bounds.height),
            color: Color::WHITE,
            size: 20.0,
            ..Text::default()
        });

        MouseCursor::Default
    }
}
