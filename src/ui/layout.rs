use stretch::{geometry, result};

use crate::graphics::{Point, Vector};
use crate::ui::{Root, Widget};

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

    pub(crate) fn draw(&self, renderer: &mut R) {
        draw_recursively(
            renderer,
            self.root.widget(),
            &self.layout,
            Point::new(0.0, 0.0),
        );
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

fn draw_recursively<'a, M, R>(
    renderer: &mut R,
    widget: &Box<Widget<Msg = M, Renderer = R> + 'a>,
    layout: &result::Layout,
    location: Point,
) {
    let location = location + Vector::new(layout.location.x, layout.location.y);

    widget.draw(renderer, location, layout.size.width, layout.size.height);

    if let Some(children) = widget.children() {
        for (widget, layout) in children.iter().zip(&layout.children) {
            draw_recursively(renderer, widget, layout, location);
        }
    }
}
