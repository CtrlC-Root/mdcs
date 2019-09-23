use std::fmt;
use std::collections::HashMap;

use super::attribute::Attribute;
use super::action::Action;

pub enum Member<'a> {
    Attribute(&'a dyn Attribute),
    Action(&'a dyn Action)
}

impl fmt::Debug for Member<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Member::Attribute(attribute) => {
                fmt.debug_tuple("Attribute")
                    .field(&attribute)
                    .finish()
            }
            Member::Action(action) => {
                fmt.debug_tuple("Action")
                    .field(&action)
                    .finish()
            }
        }
    }
}

pub trait Device {
    // TODO
}

impl fmt::Debug for &dyn Device {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Device")
            .finish()
    }
}
