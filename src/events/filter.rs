use super::types::{Event, KeyEvent, MouseEvent, SystemEvent};

// Helper trait for type-based event filtering
pub trait EventFilter {
    fn accepts(&self, event: &Event) -> bool;
}

// Implement EventFilter for common event types
impl EventFilter for KeyEvent {
    fn accepts(&self, event: &Event) -> bool {
        matches!(event, Event::Key(_))
    }
}

impl EventFilter for MouseEvent {
    fn accepts(&self, event: &Event) -> bool {
        matches!(event, Event::Mouse(_))
    }
}

impl EventFilter for SystemEvent {
    fn accepts(&self, event: &Event) -> bool {
        matches!(event, Event::System(_))
    }
}
