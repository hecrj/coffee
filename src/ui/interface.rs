use std::hash::Hasher;
use stretch::result;

use crate::graphics::{Point, Window};
use crate::ui::{Element, Event, Layout, MouseCursor, Renderer};

pub struct Interface<'a, M, R> {
    hash: u64,
    root: Element<'a, M, R>,
    layout: result::Layout,
}

pub struct Cache {
    hash: u64,
    layout: result::Layout,
}

impl<'a, M, R: Renderer> Interface<'a, M, R> {
    pub fn compute(
        root: Element<'a, M, R>,
        renderer: &R,
    ) -> Interface<'a, M, R> {
        let hasher = &mut twox_hash::XxHash::default();
        root.hash(hasher);

        let hash = hasher.finish();
        let layout = root.compute_layout(renderer);

        Interface { hash, root, layout }
    }

    pub fn compute_with_cache(
        root: Element<'a, M, R>,
        renderer: &R,
        cache: Cache,
    ) -> Interface<'a, M, R> {
        let hasher = &mut twox_hash::XxHash::default();
        root.hash(hasher);

        let hash = hasher.finish();

        let layout = if hash == cache.hash {
            cache.layout
        } else {
            root.compute_layout(renderer)
        };

        Interface { hash, root, layout }
    }

    pub fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        messages: &mut Vec<M>,
    ) {
        let Interface { root, layout, .. } = self;

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
        let Interface { root, layout, .. } = self;

        let cursor =
            root.widget
                .draw(renderer, Self::layout(layout), cursor_position);

        renderer.flush(window);

        cursor
    }

    pub fn cache(self) -> Cache {
        Cache {
            hash: self.hash,
            layout: self.layout,
        }
    }

    fn layout(layout: &result::Layout) -> Layout {
        Layout::new(layout, Point::new(0.0, 0.0))
    }
}
