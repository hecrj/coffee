use crate::graphics::{Batch, Font, Frame, Image, Point};
use crate::load::{Join, Task};
use crate::ui::core;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Renderer {
    pub(crate) sprites: Batch,
    pub(crate) font: Rc<RefCell<Font>>,
}

impl core::Renderer for Renderer {
    type Configuration = Configuration;

    fn load(config: Configuration) -> Task<Renderer> {
        (config.sprites, config.font)
            .join()
            .map(|(sprites, font)| Renderer {
                sprites: Batch::new(sprites),
                font: Rc::new(RefCell::new(font)),
            })
    }

    fn flush(&mut self, frame: &mut Frame) {
        let target = &mut frame.as_target();

        self.sprites.draw(Point::new(0.0, 0.0), target);
        self.sprites.clear();

        self.font.borrow_mut().draw(target);
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
