use crate::graphics::{
    Batch, Color, Font, Image, Point, Quad, Rectangle, Window,
};
use crate::load::{Join, Task};
use crate::ui::core;
use crate::ui::core::widget::column;

use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

pub struct Renderer {
    pub(crate) sprites: Batch,
    pub(crate) debug: Batch,
    pub(crate) font: Rc<RefCell<Font>>,
}

impl core::Renderer for Renderer {
    type Configuration = Configuration;

    fn load(config: Configuration) -> Task<Renderer> {
        let load_debug = Task::using_gpu(|gpu| {
            Image::from_colors(
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
            )
        });

        (config.sprites, load_debug, config.font).join().map(
            |(sprites, debug, font)| Renderer {
                sprites: Batch::new(sprites),
                debug: Batch::new(debug),
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

impl column::Renderer for Renderer {
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

pub struct Configuration {
    pub sprites: Task<Image>,
    pub font: Task<Font>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            sprites: Task::using_gpu(|gpu| {
                Image::from_image(
                    gpu,
                    image::load_from_memory(include_bytes!(
                        "../../resources/ui.png"
                    ))?,
                )
            }),
            font: Font::load(include_bytes!(
                "../../resources/font/Inconsolata-Regular.ttf"
            )),
        }
    }
}
