use super::{Axis, Button};

use std::convert::TryFrom;

/// A gamepad event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// A gamepad was connected.
    Connected,

    /// A gamepad was disconnected.
    Disconnected,

    /// A button was pressed.
    ButtonPressed(Button),

    /// A button was released.
    ButtonReleased(Button),

    /// The value of a button was changed.
    ButtonChanged(Button, f32),

    /// The value of an axis was changed.
    AxisChanged(Axis, f32),
}

impl TryFrom<gilrs::EventType> for Event {
    type Error = ();

    fn try_from(event_type: gilrs::EventType) -> Result<Self, Self::Error> {
        match event_type {
            gilrs::EventType::Connected => Ok(Event::Connected),
            gilrs::EventType::Disconnected => Ok(Event::Disconnected),
            gilrs::EventType::ButtonPressed(button, _) => {
                Ok(Event::ButtonPressed(button))
            }
            gilrs::EventType::ButtonReleased(button, _) => {
                Ok(Event::ButtonReleased(button))
            }
            gilrs::EventType::ButtonChanged(button, value, _) => {
                Ok(Event::ButtonChanged(button, value))
            }
            gilrs::EventType::AxisChanged(axis, value, _) => {
                Ok(Event::AxisChanged(axis, value))
            }
            gilrs::EventType::ButtonRepeated(_, _) => Err(()),
            gilrs::EventType::Dropped => Err(()),
        }
    }
}
