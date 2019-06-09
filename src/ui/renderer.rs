mod button;
mod checkbox;
mod radio;
mod slider;
mod text;

use crate::graphics::{Batch, Font, Frame, Image, Point};
use crate::load::{Join, Task};
use crate::ui::core;

use std::cell::RefCell;
use std::rc::Rc;

/// A renderer capable of drawing all the [built-in widgets].
///
/// It can be configured using [`Configuration`] and
/// [`UserInterface::configuration`].
///
/// [built-in widgets]: widget/index.html
/// [`Configuration`]: struct.Configuration.html
/// [`UserInterface::configuration`]: trait.UserInterface.html#method.configuration
pub struct Renderer {
    pub(crate) sprites: Batch,
    pub(crate) font: Rc<RefCell<Font>>,
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            .field("sprites", &self.sprites)
            .finish()
    }
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

    fn flush(&mut self, frame: &mut Frame<'_>) {
        let target = &mut frame.as_target();

        self.sprites.draw(Point::new(0.0, 0.0), target);
        self.sprites.clear();

        self.font.borrow_mut().draw(target);
    }
}

/// The [`Renderer`] configuration.
///
/// You can implement [`UserInterface::configuration`] and return your own
/// [`Configuration`] to customize the built-in [`Renderer`].
///
/// [`Renderer`]: struct.Renderer.html
/// [`UserInterface::configuration`]: trait.UserInterface.html#method.configuration
/// [`Configuration`]: struct.Configuration.html
///
/// # Example
/// ```no_run
/// use coffee::graphics::Image;
/// use coffee::ui::Configuration;
///
/// Configuration {
///     sprites: Image::load("resources/my_ui_sprites.png"),
///     ..Configuration::default()
/// };
/// ```
#[derive(Debug)]
pub struct Configuration {
    /// The spritesheet used to render the [different widgets] of the user interface.
    ///
    /// The spritesheet needs to be structured like [the default spritesheet].
    ///
    /// [different widgets]: widget/index.html
    /// [the default spritesheet]: https://raw.githubusercontent.com/hecrj/coffee/92aa6b64673116fdc49d8694a10ee5bf53afb1b5/resources/ui.png
    pub sprites: Task<Image>,

    /// The font used to render [`Text`].
    ///
    /// By default, it uses [Inconsolata Regular].
    ///
    /// [`Text`]: widget/text/struct.Text.html
    /// [Inconsolata Regular]: https://fonts.google.com/specimen/Inconsolata
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
