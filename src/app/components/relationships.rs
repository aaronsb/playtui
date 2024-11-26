use crate::events::{Event, Action};
use super::ComponentManager;

/// Trait for managing component relationships and interactions
pub trait ComponentRelationships {
    /// Check if a component can interact with another component
    fn can_interact(&self, source: &str, target: &str) -> bool;
    
    /// Propagate an event to related components
    fn propagate_event(&mut self, event: &Event, source: &str) -> Vec<Action>;
}

impl ComponentRelationships for ComponentManager {
    fn can_interact(&self, source: &str, target: &str) -> bool {
        // Define component interaction rules
        match (source, target) {
            // Library browser can interact with track list
            ("library_browser", "track_list") => true,
            // Track list can interact with track details and playback status
            ("track_list", "track_details") | ("track_list", "playback_status") => true,
            // Controls can interact with playback status and volume control
            ("controls", "playback_status") | ("controls", "volume_control") => true,
            // Current track info can interact with playback status
            ("current_track_info", "playback_status") => true,
            // By default, components cannot interact
            _ => false,
        }
    }

    fn propagate_event(&mut self, event: &Event, source: &str) -> Vec<Action> {
        let mut actions = Vec::new();
        let _ = self.logger.log_debug(&format!("Propagating event from {}: {:?}", source, event));

        // Process the event through components based on relationships
        for component in &mut self.components {
            if let Ok(Some(action)) = component.handle_event(event) {
                actions.push(action);
            }
        }

        actions
    }
}
