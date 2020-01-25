//! Load your game assets with type-safety and build loading screens with
//! consistent progress tracking.
//!
//! # Tasks
//! The generic [`Task`] struct represents a lazy loading operation that can
//! be combined and composed with other tasks. Most of the types representing
//! resources in Coffee have `load` functions that return a [`Task`].
//!
//! Tasks are defined declaratively in a functional style. This allows them to
//! keep track of all the work they have to complete before even executing them.
//! Read the [`Task`] docs to learn more!
//!
//! # Loading screens
//! The [`LoadingScreen`] trait allows you to implement a loading screen that is
//! compatible with any [`Task`]. Currently, Coffee includes a built-in loading
//! screen: [`ProgressBar`], which shows a simple progress bar with some text.
//!
//! [`Task`]: struct.Task.html
//! [`LoadingScreen`]: loading_screen/trait.LoadingScreen.html
//! [`ProgressBar`]: loading_screen/struct.ProgressBar.html
mod task;

pub mod loading_screen;

pub use loading_screen::LoadingScreen;
pub use task::{Join, Progress, Task};
