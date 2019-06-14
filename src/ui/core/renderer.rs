use crate::graphics::{Color, Frame};
use crate::load::Task;
use crate::ui::core::Layout;

/// The renderer of a user interface.
///
/// The implementor of this trait will also need to implement the `Renderer`
/// trait of the [widgets] you want to use.
///
/// [widgets]: ../widget/index.html
pub trait Renderer {
    /// The configuration of the renderer.
    ///
    /// It has to implement the `Default` trait.
    ///
    /// This type allows you to provide a way for your users to customize the
    /// renderer. For example, you could make the default text color and size of
    /// your configurable, support different fonts, etc.
    type Configuration: Default;

    /// Loads the renderer with the given configuration.
    fn load(config: Self::Configuration) -> Task<Self>
    where
        Self: Sized;

    /// Explains the [`Layout`] of an [`Element`] for debugging purposes.
    ///
    /// This will be called when [`Element::explain`] has been used. It should
    /// _explain_ the [`Layout`] graphically.
    ///
    /// [`Layout`]: struct.Layout.html
    /// [`Element`]: struct.Element.html
    /// [`Element::explain`]: struct.Element.html#method.explain
    fn explain(&mut self, layout: &Layout<'_>, color: Color);

    /// Flushes the renderer to draw on the given [`Frame`].
    ///
    /// This method will be called by the runtime after calling [`Widget::draw`]
    /// for all the widgets of the user interface.
    ///
    /// The recommended strategy to implement a [`Renderer`] is to use [`Batch`]
    /// and call [`Batch::draw`] here.
    ///
    /// [`Frame`]: ../../graphics/struct.Frame.html
    /// [`Widget::draw`]: trait.Widget.html#tymethod.draw
    /// [`Renderer`]: trait.Renderer.html
    /// [`Batch`]: ../../graphics/struct.Batch.html
    /// [`Batch::draw`]: ../../graphics/struct.Batch.html#method.draw
    fn flush(&mut self, frame: &mut Frame<'_>);
}
