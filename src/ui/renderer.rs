use crate::graphics::{Batch, Color, Image, Point, Quad, Rectangle, Window};
use crate::load::Task;
use crate::ui::{button, column};

pub trait Renderer {
    fn load() -> Task<Self>
    where
        Self: Sized;

    fn draw(&mut self, window: &mut Window);
}

pub struct Basic {
    batch: Batch,
}

impl Renderer for Basic {
    fn load() -> Task<Basic> {
        Task::using_gpu(|gpu| {
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

            Ok(Basic {
                batch: Batch::new(image),
            })
        })
    }

    fn draw(&mut self, window: &mut Window) {
        let mut frame = window.frame();

        self.batch
            .draw(Point::new(0.0, 0.0), &mut frame.as_target());
        self.batch.clear();
    }
}

impl column::Renderer for Basic {
    fn draw(&mut self, position: Point, width: f32, height: f32) {
        self.batch.add(Quad {
            source: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.5,
                height: 1.0,
            },
            position,
            size: (width, height),
        });
    }
}

impl button::Renderer for Basic {
    fn draw(
        &mut self,
        state: &button::State,
        position: Point,
        width: f32,
        height: f32,
    ) {
        self.batch.add(Quad {
            source: Rectangle {
                x: 0.5,
                y: 0.0,
                width: 0.5,
                height: 1.0,
            },
            position,
            size: (width, height),
        });
    }
}
