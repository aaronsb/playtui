use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum EventError {
    InvalidEvent(String),
    HandlerError(String),
    DispatchError(String),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::InvalidEvent(msg) => write!(f, "Invalid event: {}", msg),
            EventError::HandlerError(msg) => write!(f, "Handler error: {}", msg),
            EventError::DispatchError(msg) => write!(f, "Dispatch error: {}", msg),
        }
    }
}

impl Error for EventError {}

pub type EventResult<T> = Result<T, EventError>;
