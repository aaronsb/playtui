use crate::events::{Action, EventHandler};
use crate::logger::Logger;

pub mod registry;
pub mod lifecycle;
pub mod relationships;

pub use registry::ComponentRegistry;
pub use lifecycle::ComponentLifecycle;
pub use relationships::ComponentRelationships;

/// Manages component registration and interaction
pub struct ComponentManager {
    pub(crate) components: Vec<Box<dyn EventHandler>>,
    pub(crate) logger: Logger,
}

impl ComponentManager {
    /// Creates a new ComponentManager
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            logger: Logger::new().expect("Failed to initialize logger"),
        }
    }

    /// Updates all components with an action
    pub fn update_components(&mut self, action: Action) {
        let _ = self.logger.log_debug("\n=== Processing Action ===");
        let _ = self.logger.log_debug(&format!("Action: {:?}", action));
        
        let mut pending_actions = vec![action];
        let mut processed_actions = Vec::new();

        while let Some(current_action) = pending_actions.pop() {
            // Skip if we've already processed this action type
            if processed_actions.contains(&current_action) {
                let _ = self.logger.log_debug(&format!("Skipping already processed action: {:?}", current_action));
                continue;
            }
            processed_actions.push(current_action.clone());

            // Convert action to event
            if let Some(event) = self.convert_action_to_event(&current_action) {
                let _ = self.logger.log_debug(&format!("Converting action to event: {:?}", event));
                let _ = self.logger.log_event(&event);
                
                // Process the event through each component
                for component in &mut self.components {
                    if let Ok(Some(follow_up)) = component.handle_event(&event) {
                        let _ = self.logger.log_debug(&format!("Component generated follow-up action: {:?}", follow_up));
                        // Only add non-refresh follow-up actions that we haven't processed
                        if !matches!(follow_up, Action::Refresh) && !processed_actions.contains(&follow_up) {
                            pending_actions.push(follow_up);
                        }
                    }
                }
            } else {
                let _ = self.logger.log_debug(&format!("No event generated for action: {:?}", current_action));
            }
        }
        
        let _ = self.logger.log_debug("=== Action Processing Complete ===\n");
    }
}
