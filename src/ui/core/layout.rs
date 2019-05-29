use stretch::result;

use crate::graphics::{Point, Rectangle, Vector};

#[derive(Debug)]
pub struct Layout<'a> {
    layout: &'a result::Layout,
    position: Point,
}

impl<'a> Layout<'a> {
    pub(crate) fn new(
        layout: &'a result::Layout,
        parent_position: Point,
    ) -> Self {
        let position =
            parent_position + Vector::new(layout.location.x, layout.location.y);

        Layout { layout, position }
    }

    pub fn bounds(&self) -> Rectangle<f32> {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.layout.size.width,
            height: self.layout.size.height,
        }
    }

    pub fn children(&'a self) -> impl Iterator<Item = Layout<'a>> {
        self.layout
            .children
            .iter()
            .map(move |layout| Layout::new(layout, self.position))
    }
}
