//! Coffee is an opinionated 2D game engine focused on simplicity, explicitness,
//! and type-safety.
//!
//! # Features
//!   * [Responsive, customizable GUI]
//!   * Declarative, type-safe loading screens with progress tracking
//!   * Built-in [debug view with performance metrics]
//!   * Fixed, deterministic timestep
//!   * Explicit, easy to use, hardware-accelerated 2D graphics API
//!   * Multiplatform support leveraging OpenGL, Vulkan, Metal, D3D11, and D3D12
//!   * [Explicit and efficient batched draws]
//!   * [Mesh support]
//!   * Texture array support
//!   * Off-screen rendering
//!   * TrueType font rendering
//!   * Gamepad support
//!
//! Check out the [repository] and the [examples] for more details!
//!
//! [Responsive, customizable GUI]: https://gfycat.com/gloomyweakhammerheadshark
//! [debug view with performance metrics]: struct.Debug.html
//! [Explicit and efficient batched draws]: https://gfycat.com/beautifulseparatebeetle
//! [Mesh support]: https://gfycat.com/academicglossykingfisher
//! [examples]: https://github.com/hecrj/coffee/tree/master/examples
//! [repository]: https://github.com/hecrj/coffee
//!
//! # Usage
//! To get started, implement the [`Game`] trait. Then, call [`Game::run`] with
//! some [`WindowSettings`] to run your game.
//!
//! Here is a minimal example that will open a window:
//!
//! ```no_run
//! use coffee::graphics::{Color, Frame, Window, WindowSettings};
//! use coffee::load::Task;
//! use coffee::{Game, Result, Timer};
//!
//! fn main() -> Result<()> {
//!     MyGame::run(WindowSettings {
//!         title: String::from("A caffeinated game"),
//!         size: (1280, 1024),
//!         resizable: true,
//!         fullscreen: false,
//!         maximized: false,
//!         vsync: false,
//!     })
//! }
//!
//! struct MyGame {
//!     // Your game state and assets go here...
//! }
//!
//! impl Game for MyGame {
//!     type Input = (); // No input data
//!     type LoadingScreen = (); // No loading screen
//!
//!     fn load(_window: &Window) -> Task<MyGame> {
//!         // Load your game assets here. Check out the `load` module!
//!         Task::succeed(|| MyGame { /* ... */ })
//!     }
//!
//!     fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
//!         // Clear the current frame
//!         frame.clear(Color::BLACK);
//!
//!         // Draw your game here. Check out the `graphics` module!
//!     }
//! }
//! ```
//!
//! [`Game`]: trait.Game.html
//! [`Game::run`]: trait.Game.html#method.run
//! [`WindowSettings`]: graphics/struct.WindowSettings.html
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

mod debug;
mod game;
mod result;
mod timer;

pub mod graphics;
pub mod input;
pub mod load;
pub mod ui;

pub use debug::Debug;
pub use game::Game;
pub use result::{Error, Result};
pub use timer::Timer;
