use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum EventError {
    /// Error during event dispatch
    DispatchError(String),
    
    /// Error during event handling
    HandlingError(String),
    
    /// Error during event processing
    ProcessingError(String),
    
    /// Error during focus management
    FocusError(String),
    
    /// Error during component operations
    ComponentError(String),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::DispatchError(msg) => write!(f, "Event dispatch error: {}", msg),
            EventError::HandlingError(msg) => write!(f, "Event handling error: {}", msg),
            EventError::ProcessingError(msg) => write!(f, "Event processing error: {}", msg),
            EventError::FocusError(msg) => write!(f, "Focus management error: {}", msg),
            EventError::ComponentError(msg) => write!(f, "Component error: {}", msg),
        }
    }
}

impl Error for EventError {}

/// Result type for event operations
pub type EventResult<T> = Result<T, EventError>;
