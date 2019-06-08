use crate::graphics::Window;
use crate::load::Task;

/// The state of your game.
///
/// Ideally, implementors of this trait should hold all the game data while
/// having clear boundaries established by a powerful API.
///
/// [`State`]: trait.State.html
pub trait State {
    /// Defines how many times the [`update`] function should be called per
    /// second.
    ///
    /// A common value is `60`.
    ///
    /// [`update`]: #tymethod.update
    const TICKS_PER_SECOND: u16;

    /// Loads the [`State`].
    ///
    /// Most of the time, you should use [`Task::new`] here.
    ///
    /// [`State`]: trait.State.html
    /// [`Task::new`]: load/struct.Task.html#method.new
    fn load(window: &Window) -> Task<Self>
    where
        Self: Sized;

    /// Updates the [`State`].
    ///
    /// All your game simulation logic should live here.
    ///
    /// The [`TICKS_PER_SECOND`] constant defines how many times this function
    /// will be called per second. This function may be called multiple times
    /// per frame if it is necessary.
    ///
    /// [`State`]: trait.State.html
    /// [`TICKS_PER_SECOND`]: #associatedconstant.TICKS_PER_SECOND
    fn update(&mut self);
}

impl State for () {
    const TICKS_PER_SECOND: u16 = 10;

    fn load(_window: &Window) -> Task<()> {
        Task::new(|| ())
    }

    fn update(&mut self) {}
}
