use super::{Axis, Button};

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Connected,

    Disconnected,

    ButtonPressed(Button),

    ButtonReleased(Button),

    ButtonChanged(Button, f32),

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
