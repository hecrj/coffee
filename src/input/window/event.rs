/// A window event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// The game window gained focus.
    Focused,

    /// The game window lost focus.
    Unfocused,

    /// The game window was moved.
    Moved {
        /// The new X coordinate of the window
        x: f32,

        /// The new Y coordinate of the window
        y: f32,
    },
}
