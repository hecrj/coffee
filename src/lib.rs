//! Coffee is an opinionated 2D game engine focused on simplicity, explicitness,
//! and type-safety.
//!
//! # Features
//!   * Declarative, type-safe asset loading
//!   * Loading screens with progress tracking
//!   * Built-in [debug view with performance metrics]
//!   * Fixed timestep
//!   * Explicit, easy to use, hardware-accelerated 2D graphics API
//!   * Multiplatform support leveraging OpenGL, Vulkan, Metal, D3D11, and D3D12
//!   * Texture array support
//!   * Explicit and efficient batched draws
//!   * Off-screen rendering
//!   * TrueType font rendering
//!
//! Check out the [repository] for more details!
//!
//! # Usage
//! To get started, simply implement the [`Game`] trait. Then, call
//! [`Game::run`] with some [`WindowSettings`] to run your game.
//!
//! Here is a minimal example that will open a window:
//!
//! ```no_run
//! use coffee::{Game, Result, Timer};
//! use coffee::graphics::{Color, Frame, Window, WindowSettings};
//!
//! fn main() -> Result<()> {
//!     MyGame::run(WindowSettings {
//!         title: String::from("A caffeinated game"),
//!         size: (1280, 1024),
//!         resizable: true,
//!         fullscreen: false,
//!     })
//! }
//!
//! struct MyGame {
//!     // Your game state goes here...
//! }
//!
//! impl Game for MyGame {
//!     type View = (); // No view data.
//!     type Input = (); // No input data.
//!
//!     const TICKS_PER_SECOND: u16 = 60; // Update rate
//!
//!     fn new(_window: &mut Window) -> Result<(MyGame, Self::View, Self::Input)> {
//!         // Load your game assets here. Check out the `load` module!
//!         Ok((MyGame { /* ... */ }, (), ()))
//!     }
//!
//!     fn update(&mut self, _view: &Self::View, _window: &Window) {
//!         // Update your game here
//!     }
//!
//!     fn draw(&self, _view: &mut Self::View, frame: &mut Frame, _timer: &Timer) {
//!         // Clear the current frame
//!         frame.clear(Color::BLACK);
//!
//!         // Draw your game here. Check out the `graphics` module!
//!     }
//! }
//! ```
//!
//! [debug view with performance metrics]: struct.Debug.html
//! [repository]: https://github.com/hecrj/coffee
//! [`Game`]: trait.Game.html
//! [`Game::run`]: trait.Game.html#method.run
//! [`WindowSettings`]: graphics/struct.WindowSettings.html
//#![deny(missing_docs)]
//#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![deny(unsafe_code)]

mod debug;
mod game;
mod result;
mod state;
mod timer;

pub mod graphics;
pub mod input;
pub mod load;
pub mod ui;

pub use debug::Debug;
pub use game::Game;
pub use input::Input;
pub use result::{Error, Result};
pub use state::State;
pub use timer::Timer;
pub use ui::UserInterface;
