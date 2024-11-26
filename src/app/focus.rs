use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::events::{Event, KeyEvent, FocusDirection, EventResult};

/// Manages focus state and navigation between components.
/// 
/// This module is kept as a single file despite being slightly over the typical
/// line count guideline (150 lines) because:
/// 1. It maintains high cohesion - all methods directly relate to focus management
/// 2. The logic is tightly coupled - methods work together as a unified system
/// 3. It has a single, well-defined responsibility
/// 4. Splitting would likely increase complexity rather than reduce it
/// 
/// The focus system handles:
/// - Tracking which component currently has focus
/// - Managing focus navigation between components
/// - Filtering events based on focus state
/// - Updating component focus states
pub struct FocusManager {
    component_order: Vec<String>,
    current_focus: usize,
}

impl FocusManager {
    /// Creates a new FocusManager with default component order
    pub fn new() -> Self {
        Self {
            component_order: vec![
                "library_browser".to_string(),
                "track_list".to_string(),
                "track_details".to_string(),
                "current_track_info".to_string(),
                "playback_status".to_string(),
                "controls".to_string(),
                "volume_control".to_string(),
            ],
            current_focus: 0,
        }
    }

    /// Returns the currently focused component name
    pub fn current_focus(&self) -> &str {
        &self.component_order[self.current_focus]
    }

    /// Sets focus to a specific component by name
    pub fn set_focus(&mut self, component_name: &str) {
        if let Some(index) = self.component_order.iter().position(|name| name == component_name) {
            self.current_focus = index;
        }
    }

    /// Moves focus in the specified direction
    pub fn move_focus(&mut self, direction: FocusDirection) {
        match direction {
            FocusDirection::Next => {
                self.current_focus = (self.current_focus + 1) % self.component_order.len();
            },
            FocusDirection::Previous => {
                if self.current_focus == 0 {
                    self.current_focus = self.component_order.len() - 1;
                } else {
                    self.current_focus -= 1;
                }
            },
        }
    }

    /// Determines if an event should be processed based on focus rules
    pub fn should_process_event(&self, event: &Event, component_name: &str) -> bool {
        match event {
            // Global Navigation Events - Always Available
            Event::Key(KeyEvent::Tab) |
            Event::Key(KeyEvent::BackTab) => true,
            
            // Global Hotkeys - Always Available
            Event::Key(KeyEvent::Quit) |
            Event::Key(KeyEvent::Escape) |
            Event::Key(KeyEvent::Space) |
            Event::Key(KeyEvent::Play) |
            Event::Key(KeyEvent::Pause) |
            Event::Key(KeyEvent::Stop) |
            Event::Key(KeyEvent::Next) |
            Event::Key(KeyEvent::Previous) |
            Event::Key(KeyEvent::VolumeUp) |
            Event::Key(KeyEvent::VolumeDown) => true,
            
            // Frame-Specific Events - Only process if component has focus
            Event::Key(KeyEvent::Enter) |
            Event::Key(KeyEvent::Left) |
            Event::Key(KeyEvent::Right) |
            Event::Key(KeyEvent::Up) |
            Event::Key(KeyEvent::Down) => {
                component_name == self.current_focus()
            },
            
            // System events are always processed
            Event::System(_) => true,
            
            // Navigation events are treated like arrow keys - require focus
            Event::Navigation(_) => component_name == self.current_focus(),
            
            // Mouse events require focus
            Event::Mouse(_) => component_name == self.current_focus(),
            
            // Other events require focus by default
            _ => component_name == self.current_focus(),
        }
    }

    /// Updates component focus states based on current focus
    pub fn update_focus_states(
        &self,
        library_browser: &mut LibraryBrowser,
        track_list: &mut TrackList,
        track_details: &mut TrackDetails,
        current_track_info: &mut CurrentTrackInfo,
        playback_status: &mut PlaybackStatus,
        controls: &mut Controls,
        volume_control: &mut VolumeControl,
    ) {
        let focused = self.current_focus();
        Component::set_focused(library_browser, focused == "library_browser");
        Component::set_focused(track_list, focused == "track_list");
        Component::set_focused(track_details, focused == "track_details");
        Component::set_focused(current_track_info, focused == "current_track_info");
        Component::set_focused(playback_status, focused == "playback_status");
        Component::set_focused(controls, focused == "controls");
        Component::set_focused(volume_control, focused == "volume_control");
    }

    /// Handles focus-related events
    pub fn handle_event(&mut self, event: &Event) -> EventResult<()> {
        if let Event::Key(KeyEvent::Focus(direction)) = event {
            self.move_focus(*direction);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_events_always_processed() {
        let manager = FocusManager::new();
        
        // Test global navigation events
        assert!(manager.should_process_event(&Event::Key(KeyEvent::Tab), "any_component"));
        assert!(manager.should_process_event(&Event::Key(KeyEvent::BackTab), "any_component"));
        
        // Test global hotkeys
        assert!(manager.should_process_event(&Event::Key(KeyEvent::Quit), "any_component"));
        assert!(manager.should_process_event(&Event::Key(KeyEvent::Space), "any_component"));
    }

    #[test]
    fn test_frame_specific_events_require_focus() {
        let manager = FocusManager::new();
        let focused_component = manager.current_focus();
        let unfocused_component = "unfocused";
        
        // Test arrow keys
        assert!(manager.should_process_event(&Event::Key(KeyEvent::Left), focused_component));
        assert!(!manager.should_process_event(&Event::Key(KeyEvent::Left), unfocused_component));
        
        // Test enter key
        assert!(manager.should_process_event(&Event::Key(KeyEvent::Enter), focused_component));
        assert!(!manager.should_process_event(&Event::Key(KeyEvent::Enter), unfocused_component));
    }

    #[test]
    fn test_set_focus() {
        let mut manager = FocusManager::new();
        let initial_focus = manager.current_focus().to_string();
        
        // Set focus to a different component
        manager.set_focus("track_list");
        assert_eq!(manager.current_focus(), "track_list");
        assert_ne!(manager.current_focus(), initial_focus);
        
        // Set focus to invalid component should not change focus
        manager.set_focus("invalid_component");
        assert_eq!(manager.current_focus(), "track_list");
    }
}
