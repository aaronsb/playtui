use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum EventError {
    InvalidEvent(String),
    HandlerError(String),
    DispatchError(String),
    IoError(io::Error),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::InvalidEvent(msg) => write!(f, "Invalid event: {}", msg),
            EventError::HandlerError(msg) => write!(f, "Handler error: {}", msg),
            EventError::DispatchError(msg) => write!(f, "Dispatch error: {}", msg),
            EventError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for EventError {}

impl From<io::Error> for EventError {
    fn from(error: io::Error) -> Self {
        EventError::IoError(error)
    }
}

pub type EventResult<T> = Result<T, EventError>;
