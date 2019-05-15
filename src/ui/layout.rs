use stretch::{geometry, result};

use crate::graphics::{Point, Rectangle, Vector, Window};
use crate::ui::{Event, MouseCursor, Renderer, Root, Widget};

pub struct Layout<'a, M, R> {
    root: Root<'a, M, R>,
    layout: result::Layout,
}

impl<'a, M, R> Layout<'a, M, R> {
    pub(crate) fn new(root: Root<'a, M, R>) -> Self {
        let layout = root
            .node()
            .0
            .compute_layout(geometry::Size::undefined())
            .expect("Compute layout");

        Layout { root, layout }
    }

    pub(crate) fn on_event(
        &mut self,
        events: &mut Vec<M>,
        event: Event,
        cursor_position: Point,
    ) {
        notify_recursively(
            events,
            event,
            &mut self.root.widget,
            &self.layout,
            cursor_position,
            Point::new(0.0, 0.0),
        );
    }

    pub(crate) fn draw(
        &mut self,
        renderer: &mut R,
        window: &mut Window,
        cursor_position: Point,
    ) -> MouseCursor
    where
        R: Renderer,
    {
        let mut mouse_cursor = MouseCursor::Default;

        draw_recursively(
            renderer,
            &mut self.root.widget,
            &self.layout,
            cursor_position,
            &mut mouse_cursor,
            Point::new(0.0, 0.0),
        );

        renderer.draw(window);

        mouse_cursor
    }
}

impl<'a, M, A> std::fmt::Debug for Layout<'a, M, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Layout {{ root: {:?}, layout: {:?} }}",
            self.root, self.layout
        )
    }
}

fn notify_recursively<'a, M, R>(
    events: &mut Vec<M>,
    event: Event,
    widget: &mut Box<Widget<Msg = M, Renderer = R> + 'a>,
    layout: &result::Layout,
    cursor_position: Point,
    position: Point,
) {
    let position = position + Vector::new(layout.location.x, layout.location.y);
    let bounds = Rectangle {
        x: position.x,
        y: position.y,
        width: layout.size.width,
        height: layout.size.height,
    };

    if let Some(event) = widget.on_event(event, bounds, cursor_position) {
        events.push(event);
    }

    if let Some(children) = widget.children() {
        for (widget, layout) in children.iter_mut().zip(&layout.children) {
            notify_recursively(
                events,
                event,
                widget,
                layout,
                cursor_position,
                position,
            );
        }
    }
}

fn draw_recursively<'a, M, R>(
    renderer: &mut R,
    widget: &mut Box<Widget<Msg = M, Renderer = R> + 'a>,
    layout: &result::Layout,
    cursor_position: Point,
    mouse_cursor: &mut MouseCursor,
    position: Point,
) {
    let position = position + Vector::new(layout.location.x, layout.location.y);
    let bounds = Rectangle {
        x: position.x,
        y: position.y,
        width: layout.size.width,
        height: layout.size.height,
    };

    let new_cursor = widget.draw(renderer, bounds, cursor_position);

    if new_cursor != MouseCursor::Default {
        *mouse_cursor = new_cursor;
    }

    if let Some(children) = widget.children() {
        for (widget, layout) in children.iter_mut().zip(&layout.children) {
            draw_recursively(
                renderer,
                widget,
                layout,
                cursor_position,
                mouse_cursor,
                position,
            );
        }
    }
}
