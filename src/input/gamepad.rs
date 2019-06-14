//! Listen to gamepad events.

mod event;

pub use event::Event;

pub use gilrs::Axis;
pub use gilrs::Button;

use gilrs::Gilrs;
use std::convert::TryInto;
use std::time::SystemTime;

/// A gamepad identifier.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Id(gilrs::GamepadId);

pub(crate) struct Tracker {
    context: Gilrs,
}

impl Tracker {
    pub fn new() -> Option<Tracker> {
        match Gilrs::new() {
            Ok(context) => Some(Tracker { context }),
            Err(gilrs::Error::NotImplemented(dummy_context)) => {
                // Use the dummy context as a fallback on unsupported platforms
                Some(Tracker {
                    context: dummy_context,
                })
            }
            _ => {
                // Either `gilrs::error::InvalidAxisToBtn` has occured, or a
                // platform specific error has occured.
                None
            }
        }
    }

    pub fn next_event(&mut self) -> Option<(Id, Event, SystemTime)> {
        while let Some(gilrs::Event { id, event, time }) =
            self.context.next_event()
        {
            match event.try_into() {
                Ok(gamepad_event) => {
                    return Some((Id(id), gamepad_event, time));
                }
                Err(_) => {}
            }
        }

        None
    }
}
