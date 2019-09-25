use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ErrorKind {
    Generic,
    Disconnected,
    OutOfSync,
    PathExists(String),
    PathNotFound(String)
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Generic => write!(fmt, "General failure"),
            ErrorKind::Disconnected => write!(fmt, "Device disconnected"),
            ErrorKind::OutOfSync => write!(fmt, "Device state not synchronized"),
            ErrorKind::PathExists(path) => {
                write!(fmt, "Device path already exists: {}", path)
            }
            ErrorKind::PathNotFound(path) => {
                write!(fmt, "Device path not found: {}", path)
            }
        }
    }
}

#[derive(Debug)]
pub struct DeviceError {
    kind: ErrorKind,
    source: Option<Box<dyn Error>>
}

impl fmt::Display for DeviceError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = self.source.as_ref() {
            write!(fmt, "{}: {}", self.kind, source)
        } else {
            write!(fmt, "{}", self.kind)
        }
    }
}

impl DeviceError {
    pub fn new(kind: ErrorKind) -> DeviceError {
        DeviceError {
            kind,
            source: None
        }
    }

    pub fn from(source: Box<dyn Error>) -> DeviceError {
        DeviceError {
            kind: ErrorKind::Generic,
            source: Some(source)
        }
    }
}
