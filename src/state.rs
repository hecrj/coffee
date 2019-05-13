use crate::graphics::Window;
use crate::load::Task;

/// The state of your game.
pub trait State {
    /// Defines how many times the [`update`] function should be called per
    /// second.
    ///
    /// A common value is `60`.
    ///
    /// [`update`]: #tymethod.update
    const TICKS_PER_SECOND: u16;

    fn load(window: &Window) -> Task<Self>
    where
        Self: Sized;

    /// Update your game state here.
    ///
    /// The [`TICKS_PER_SECOND`] constant defines how many times this function
    /// will be called per second. This function may be called multiple times
    /// per frame if it is necessary.
    ///
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
