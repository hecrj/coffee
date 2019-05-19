use crate::graphics::{
    Batch, Color, Font, Image, Point, Quad, Rectangle, Sprite, Text, Window,
};
use crate::load::{Join, Task};
use crate::ui::{
    button, column, panel, text, MouseCursor, Node, Number, Size, Style,
};

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
    sprites: Batch,
    debug: Batch,
    font: Rc<RefCell<Font>>,
}

impl Renderer for Basic {
    fn load() -> Task<Basic> {
        let load_sprites = Image::load("resources/ui.png").map(Batch::new);

        let load_debug = Task::using_gpu(|gpu| {
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

        let load_font = Font::load(include_bytes!(
            "../../resources/font/Inconsolata-Regular.ttf"
        ));

        (load_sprites, load_debug, load_font).join().map(
            |(sprites, debug, font)| Basic {
                sprites,
                debug,
                font: Rc::new(RefCell::new(font)),
            },
        )
    }

    fn flush(&mut self, window: &mut Window) {
        let mut frame = window.frame();
        let target = &mut frame.as_target();

        //self.debug.draw(Point::new(0.0, 0.0), target);
        self.debug.clear();

        self.sprites.draw(Point::new(0.0, 0.0), target);
        self.sprites.clear();

        self.font.borrow_mut().draw(target);
    }
}

impl column::Renderer for Basic {
    fn draw(&mut self, bounds: Rectangle<f32>) {
        self.debug.add(Quad {
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

        self.debug.add(Quad {
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
            content: label,
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
        let font = self.font.clone();
        let content = String::from(content);
        let measure = RefCell::new(None);

        Node::new_leaf(style, move |size| {
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
                    content: &content,
                    size: 20.0,
                    bounds,
                    ..Text::default()
                };

                let (width, height) = font.borrow_mut().measure(text);

                *measure = Some(Size {
                    width,
                    height: height + 5.0,
                });
            }

            measure.unwrap()
        })
    }

    fn draw(
        &mut self,
        content: &str,
        bounds: Rectangle<f32>,
        _cursor_position: Point,
    ) -> MouseCursor {
        self.font.borrow_mut().add(Text {
            content,
            position: Point::new(bounds.x, bounds.y),
            bounds: (bounds.width, bounds.height),
            color: Color::WHITE,
            size: 20.0,
            ..Text::default()
        });

        MouseCursor::Default
    }
}

// Panel
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

impl panel::Renderer for Basic {
    fn draw(&mut self, bounds: Rectangle<f32>) {
        self.debug.add(Quad {
            source: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.5,
                height: 1.0,
            },
            position: Point::new(bounds.x, bounds.y),
            size: (bounds.width, bounds.height),
        });

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
