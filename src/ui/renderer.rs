mod button;
mod checkbox;
mod image;
mod panel;
mod progress_bar;
mod radio;
mod slider;
mod text;

use crate::graphics::{Batch, Color, Font, Frame, Image, Mesh, Shape};
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
    pub(crate) images: Vec<Batch>,
    pub(crate) font: Rc<RefCell<Font>>,
    explain_mesh: Mesh,
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            .field("sprites", &self.sprites)
            .field("images", &self.images)
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
                images: Vec::new(),
                font: Rc::new(RefCell::new(font)),
                explain_mesh: Mesh::new(),
            })
    }

    fn explain(&mut self, layout: &core::Layout<'_>, color: Color) {
        self.explain_mesh
            .stroke(Shape::Rectangle(layout.bounds()), color, 1.0);

        layout
            .children()
            .for_each(|layout| self.explain(&layout, color));
    }

    fn flush(&mut self, frame: &mut Frame<'_>) {
        let target = &mut frame.as_target();

        self.sprites.draw(target);
        self.sprites.clear();

        for image in &self.images {
            image.draw(target);
        }

        self.images.clear();

        self.font.borrow_mut().draw(target);

        if !self.explain_mesh.is_empty() {
            self.explain_mesh.draw(target);
            self.explain_mesh = Mesh::new();
        }
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
                    &::image::load_from_memory(include_bytes!(
                        "../../resources/ui.png"
                    ))?,
                )
            }),
            font: Font::load_from_bytes(include_bytes!(
                "../../resources/font/Inconsolata-Regular.ttf"
            )),
        }
    }
}
