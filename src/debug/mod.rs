#[cfg(not(debug_assertions))]
mod null;

#[cfg(debug_assertions)]
mod basic;

#[cfg(not(debug_assertions))]
pub use null::Debug;

#[cfg(debug_assertions)]
pub use basic::Debug;
