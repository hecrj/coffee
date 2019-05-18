use stretch::result;

use crate::graphics::{Point, Window};
use crate::ui::{Event, Layout, MouseCursor, Renderer, Root};

pub struct Interface<'a, M, R> {
    root: Root<'a, M, R>,
    layout: result::Layout,
}

impl<'a, M, R: Renderer> Interface<'a, M, R> {
    pub fn compute(root: Root<'a, M, R>, renderer: &R) -> Interface<'a, M, R> {
        let layout = root.compute_layout(renderer);

        Interface { root, layout }
    }

    pub fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        messages: &mut Vec<M>,
    ) {
        let Interface { root, layout } = self;

        root.widget.on_event(
            event,
            Self::layout(layout),
            cursor_position,
            messages,
        );
    }

    pub fn draw(
        &self,
        renderer: &mut R,
        window: &mut Window,
        cursor_position: Point,
    ) -> MouseCursor {
        let Interface { root, layout } = self;

        let cursor =
            root.widget
                .draw(renderer, Self::layout(layout), cursor_position);

        renderer.flush(window);

        cursor
    }

    fn layout(layout: &result::Layout) -> Layout {
        Layout::new(layout, Point::new(0.0, 0.0))
    }
}
