use std::collections::HashMap;
use std::fmt;
use std::result::Result;

use super::action::Action;
use super::attribute::Attribute;

use super::error::{DeviceError, ErrorKind};

pub enum Member {
    Attribute(Box<dyn Attribute>),
    Action(Box<dyn Action>),
}

impl fmt::Debug for Member {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Member::Attribute(attribute) => fmt.debug_tuple("Attribute").field(attribute).finish(),
            Member::Action(action) => fmt.debug_tuple("Action").field(action).finish(),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    members: HashMap<String, Member>,
}

impl Device {
    pub fn new() -> Device {
        Device {
            members: HashMap::new(),
        }
    }

    pub fn get(&self, path: &str) -> Option<&Member> {
        self.members.get(path)
    }

    pub fn insert(&mut self, path: &str, member: Member) -> Result<(), DeviceError> {
        let path = String::from(path);
        if self.members.contains_key(&path) {
            Err(DeviceError::new(ErrorKind::PathExists(path)))
        } else {
            self.members.insert(path, member);
            Ok(())
        }
    }

    pub fn remove(&mut self, path: &str) -> Result<(), DeviceError> {
        let path = String::from(path);
        if self.members.contains_key(&path) {
            self.members.remove(&path);
            Ok(())
        } else {
            Err(DeviceError::new(ErrorKind::PathNotFound(path)))
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Member> {
        self.members.iter()
    }
}
