use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DeviceError {
    Io(io::Error),
    InternalError(String),
    NotImplemented,
    ActionRunInvalid(String),
    AttributeReadInvalid(String),
    AttributeWriteInvalid(String),
    PathExists(String),
    PathNotFound(String),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::Io(ref error) => write!(fmt, "IO error: {}", error),
            DeviceError::InternalError(msg) => write!(fmt, "Internal error: {}", msg),
            DeviceError::NotImplemented => write!(fmt, "Method not implemented"),
            DeviceError::ActionRunInvalid(path) => write!(fmt, "Action cannot be run: {}", path),
            DeviceError::AttributeReadInvalid(path) => write!(fmt, "Attribute cannot be read: {}", path),
            DeviceError::AttributeWriteInvalid(path) => write!(fmt, "Attribute cannot be written: {}", path),
            DeviceError::PathExists(path) => write!(fmt, "Device path already exists: {}", path),
            DeviceError::PathNotFound(path) => write!(fmt, "Device path not found: {}", path),
        }
    }
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeviceError::Io(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for DeviceError {
    fn from(err: io::Error) -> DeviceError {
        DeviceError::Io(err)
    }
}

impl From<&str> for DeviceError {
    fn from(msg: &str) -> DeviceError {
        DeviceError::InternalError(msg.to_string())
    }
}

impl From<String> for DeviceError {
    fn from(msg: String) -> DeviceError {
        DeviceError::InternalError(msg)
    }
}
