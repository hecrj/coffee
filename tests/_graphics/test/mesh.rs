use coffee::graphics::{self, Canvas, Color, Point, Shape};
use coffee::load::Task;

pub struct Mesh {}

impl Mesh {
    pub fn draw() -> Task<Canvas> {
        Task::using_gpu(|gpu| {
            let mut canvas =
                Canvas::new(gpu, 300, 300).expect("Canvas creation");

            let mut mesh = graphics::Mesh::new();

            mesh.stroke(
                Shape::Circle {
                    center: Point::new(150.0, 150.0),
                    radius: 40.0,
                },
                Color::RED,
                1.0,
            );

            mesh.stroke(
                Shape::Circle {
                    center: Point::new(150.0, 150.0),
                    radius: 80.0,
                },
                Color::GREEN,
                2.0,
            );

            mesh.stroke(
                Shape::Circle {
                    center: Point::new(150.0, 150.0),
                    radius: 120.0,
                },
                Color::BLUE,
                3.0,
            );

            mesh.draw(&mut canvas.as_target(gpu));

            Ok(canvas)
        })
    }
}
