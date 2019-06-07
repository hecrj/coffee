use crate::graphics::Point;
use crate::ui::core::{Event, Hasher, Layout, MouseCursor, Node};

/// A component that displays information or allows interaction.
///
/// Implementors of this trait should also implement `From` for [`Element`].
///
/// [`Element`]: struct.Element.html
pub trait Widget<Message, Renderer> {
    /// Returns the [`Node`] of the [`Widget`].
    ///
    /// This [`Node`] is used by the runtime to compute the [`Layout`] of the
    /// user interface.
    ///
    /// [`Node`]: struct.Node.html
    /// [`Widget`]: trait.Widget.html
    /// [`Layout`]: struct.Layout.html
    fn node(&self, renderer: &Renderer) -> Node;

    /// Draws the [`Widget`] using the associated `Renderer`.
    ///
    /// [`Widget`]: trait.Widget.html
    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor;

    /// Computes the _layout_ hash of the [`Widget`].
    ///
    /// The produced hash is used to decide if the [`Layout`] needs to be
    /// recomputed between frames. Therefore, to ensure maximum efficiency, the
    /// hash should only be affected by the properties of the [`Widget`] that
    /// may affect layouting.
    ///
    /// For example, the [`Text`] widget does not hash its color property, as
    /// its value cannot affect the overall [`Layout`] of the user interface.
    ///
    /// [`Widget`]: trait.Widget.html
    /// [`Layout`]: struct.Layout.html
    /// [`Text`]: ../widget/text/struct.Text.html
    fn hash(&self, state: &mut Hasher);

    /// Processes a runtime [`Event`].
    ///
    /// It receives:
    ///   * an [`Event`] describing user interaction.
    ///   * the computed [`Layout`] of the [`Widget`]
    ///   * the current cursor position
    ///   * a mutable `Message` vector, allowing the [`Widget`] to produce
    ///   new messages based on user interaction.
    ///
    /// By default, it does nothing.
    ///
    /// [`Event`]: enum.Event.html
    /// [`Widget`]: trait.Widget.html
    /// [`Layout`]: struct.Layout.html
    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
    ) {
    }
}
