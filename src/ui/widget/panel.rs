//! Wrap your widgets in a box.
use std::hash::Hash;

use crate::graphics::{Point, Rectangle};
use crate::ui::core::{
    Element, Event, Hasher, Layout, MouseCursor, Node, Style, Widget,
};

/// A box that can wrap a widget.
///
/// It implements [`Widget`] when the [`core::Renderer`] implements the
/// [`panel::Renderer`] trait.
///
/// [`Widget`]: ../../core/trait.Widget.html
/// [`core::Renderer`]: ../../core/trait.Renderer.html
/// [`panel::Renderer`]: trait.Renderer.html
///
/// # Example
///
/// ```
/// use coffee::ui::{Panel, Text};
/// use coffee::graphics::HorizontalAlignment;
///
/// pub enum Message { /* ... */ }
///
/// Panel::<Message>::new(
///     Text::new("I'm in a box!")
///         .horizontal_alignment(HorizontalAlignment::Center)
/// )
///     .width(500);
/// ```
pub struct Panel<'a, Message, Renderer> {
    style: Style,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> std::fmt::Debug for Panel<'a, Message, Renderer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Panel")
            .field("style", &self.style)
            .field("content", &self.content)
            .finish()
    }
}

impl<'a, Message, Renderer> Panel<'a, Message, Renderer> {
    /// Creates new [`Panel`] containing the given [`Widget`].
    ///
    /// [`Panel`]: struct.Panel.html
    /// [`Widget`]: ../../core/trait.Widget.html
    pub fn new<E>(content: E) -> Self
    where
        E: 'a + Into<Element<'a, Message, Renderer>>,
    {
        Panel {
            style: Style::default().padding(20),
            content: content.into(),
        }
    }

    /// Sets the width of the [`Panel`] in pixels.
    ///
    /// [`Panel`]: struct.Panel.html
    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width);
        self
    }

    /// Sets the maximum width of the [`Panel`] in pixels.
    ///
    /// [`Panel`]: struct.Panel.html
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.style = self.style.max_width(max_width);
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Panel<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn node(&self, renderer: &Renderer) -> Node {
        Node::with_children(
            self.style,
            vec![self.content.widget.node(renderer)],
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
    ) {
        [&mut self.content]
            .iter_mut()
            .zip(layout.children())
            .for_each(|(child, layout)| {
                child
                    .widget
                    .on_event(event, layout, cursor_position, messages)
            });
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> MouseCursor {
        let bounds = layout.bounds();
        let mut cursor = MouseCursor::OutOfBounds;
        renderer.draw(bounds);

        [&self.content].iter().zip(layout.children()).for_each(
            |(child, layout)| {
                let new_cursor =
                    child.widget.draw(renderer, layout, cursor_position);

                if new_cursor != MouseCursor::OutOfBounds {
                    cursor = new_cursor;
                }
            },
        );

        if cursor == MouseCursor::OutOfBounds {
            if bounds.contains(cursor_position) {
                MouseCursor::Idle
            } else {
                MouseCursor::OutOfBounds
            }
        } else {
            cursor
        }
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);
    }
}

/// The renderer of a [`Panel`].
///
/// Your [`core::Renderer`] will need to implement this trait before being
/// able to use a [`Panel`] in your user interface.
///
/// [`Panel`]: struct.Panel.html
/// [`core::Renderer`]: ../../core/trait.Renderer.html
pub trait Renderer {
    /// Draws a [`Panel`].
    ///
    /// It receives the bounds of the [`Panel`].
    ///
    /// [`Panel`]: struct.Panel.html
    fn draw(&mut self, bounds: Rectangle<f32>);
}

impl<'a, Message, Renderer> From<Panel<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'static + self::Renderer,
    Message: 'static,
{
    fn from(
        panel: Panel<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(panel)
    }
}
