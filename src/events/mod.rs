//! Event system module
//! 
//! This module provides the event handling infrastructure for the application.
//! It includes event types, actions, handlers, and filtering capabilities.

// Internal modules
mod error;
mod types;
mod actions;
mod handler;
mod filter;

// Re-export all public items
pub use error::{EventError, EventResult};
pub use types::{Event, KeyEvent, MouseEvent, SystemEvent, FocusDirection};
pub use actions::{
    Action, PlayerAction, PlaylistAction, UIAction, MetadataAction, 
    AppAction, TrackMetadata
};
pub use handler::{EventHandler, EventDispatcher};
pub use filter::EventFilter;
