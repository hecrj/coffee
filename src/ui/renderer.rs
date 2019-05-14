use crate::graphics::{
    Batch, Color, Font, Image, Point, Quad, Rectangle, Text, Window,
};
use crate::load::{Join, Task};
use crate::ui::{button, column};

pub trait Renderer {
    fn load() -> Task<Self>
    where
        Self: Sized;

    fn draw(&mut self, window: &mut Window);
}

pub struct Basic {
    batch: Batch,
    font: Font,
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

        let load_font = Font::load(include_bytes!(
            "../../resources/font/Inconsolata-Regular.ttf"
        ));

        (load_batch, load_font)
            .join()
            .map(|(batch, font)| Basic { batch, font })
    }

    fn draw(&mut self, window: &mut Window) {
        let mut frame = window.frame();
        let target = &mut frame.as_target();

        self.batch.draw(Point::new(0.0, 0.0), target);
        self.batch.clear();

        self.font.draw(target);
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
    ) {
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

        self.font.add(Text {
            content: String::from(label),
            position: Point::new(bounds.x, bounds.y),
            bounds: (bounds.width, bounds.height),
            color: if bounds.contains(cursor_position) {
                Color::BLACK
            } else {
                Color::WHITE
            },
            size: 20.0,
            ..Text::default()
        });
    }
}
