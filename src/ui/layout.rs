#[derive(Debug)]
pub struct Layout(stretch::result::Layout);

impl Layout {
    pub(super) fn new(layout: stretch::result::Layout) -> Layout {
        Layout(layout)
    }
}
