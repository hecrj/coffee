#[cfg(not(any(debug_assertions, feature = "debug")))]
mod null;

#[cfg(any(debug_assertions, feature = "debug"))]
mod basic;

#[cfg(not(any(debug_assertions, feature = "debug")))]
pub use null::Debug;

#[cfg(any(debug_assertions, feature = "debug"))]
pub use basic::Debug;
