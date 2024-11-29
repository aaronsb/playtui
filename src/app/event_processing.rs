use crate::events::{Event, KeyEvent, MouseEvent, FocusDirection, EventResult, EventError};
use super::App;

/// Event processing implementation for the App
impl App {
    /// Process a mouse event and update focus if needed
    fn process_mouse_event(&mut self, mouse_event: MouseEvent) -> EventResult<()> {
        match mouse_event {
            MouseEvent::Click { x, y } => {
                // Check if click is in a component area
                if let Some(component_name) = self.area_manager.component_at_position(x, y) {
                    // Update focus if clicking unfocused component
                    if component_name != self.focus_manager.current_focus() {
                        let _ = self.logger.log_debug(&format!("Focusing component from click: {}", component_name));
                        self.focus_manager.set_focus(component_name);
                        self.update_focus_states();
                    }
                    
                    // Process the click event
                    if let Ok(action) = self.event_manager.dispatch_event(&Event::Mouse(mouse_event)) {
                        let _ = self.logger.log_debug(&format!("Generated action from click: {:?}", action));
                        self.component_manager.update_components(action);
                    }
                }
            },
            MouseEvent::Scroll { .. } => {
                // Only process scroll events for the focused component
                let _focused_component = self.focus_manager.current_focus();
                if let Ok(action) = self.event_manager.dispatch_event(&Event::Mouse(mouse_event)) {
                    let _ = self.logger.log_debug(&format!("Generated action from scroll: {:?}", action));
                    self.component_manager.update_components(action);
                }
            }
        }
        Ok(())
    }

    /// Process a navigation event (Tab/BackTab/Focus)
    fn process_navigation_event(&mut self, key_event: KeyEvent) -> EventResult<()> {
        match key_event {
            KeyEvent::Tab => {
                let _ = self.logger.log_debug("Processing Tab event for global navigation");
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(FocusDirection::Next)))
                    .map_err(|_| EventError::FocusError("Failed to handle Tab event".to_string()))?;
            },
            KeyEvent::BackTab => {
                let _ = self.logger.log_debug("Processing BackTab event for global navigation");
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(FocusDirection::Previous)))
                    .map_err(|_| EventError::FocusError("Failed to handle BackTab event".to_string()))?;
            },
            KeyEvent::Focus(direction) => {
                let _ = self.logger.log_debug(&format!("Processing Focus event: {:?}", direction));
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(direction)))
                    .map_err(|_| EventError::FocusError("Failed to handle Focus event".to_string()))?;
            },
            _ => return Ok(()),
        }
        
        self.update_focus_states();
        let _ = self.logger.log_debug(&format!("New focus after navigation: {}", self.focus_manager.current_focus()));
        Ok(())
    }

    /// Process a frame-specific event (arrow keys, enter)
    fn process_frame_event(&mut self, key_event: KeyEvent) -> EventResult<()> {
        let focused_component = self.focus_manager.current_focus();
        let _ = self.logger.log_debug(&format!("Processing frame-specific event for {}: {:?}", focused_component, key_event));
        
        if self.focus_manager.should_process_event(&Event::Key(key_event), focused_component) {
            let _ = self.logger.log_debug("Component should process event");
            if let Ok(action) = self.event_manager.dispatch_event(&Event::Key(key_event)) {
                let _ = self.logger.log_debug(&format!("Generated action: {:?}", action));
                self.component_manager.update_components(action);
            }
        } else {
            let _ = self.logger.log_debug("Event ignored - component not focused");
        }
        Ok(())
    }

    /// Process a global hotkey event
    fn process_hotkey_event(&mut self, key_event: KeyEvent) -> EventResult<()> {
        let _ = self.logger.log_debug("Processing global hotkey");
        if let Ok(action) = self.event_manager.dispatch_event(&Event::Key(key_event)) {
            let _ = self.logger.log_debug(&format!("Generated action from hotkey: {:?}", action));
            self.component_manager.update_components(action);
        }
        Ok(())
    }

    /// Handles incoming events with proper logging and error handling
    pub fn handle_event(&mut self, event: Event) -> EventResult<()> {
        // Log the incoming event and current state
        let _ = self.logger.log_debug("=== Event Processing Start ===");
        let _ = self.logger.log_event(&event);
        let _ = self.logger.log_debug(&format!("Current focus: {}", self.focus_manager.current_focus()));

        let result = match event {
            // Mouse Events - Check position and update focus before processing
            Event::Mouse(mouse_event) => self.process_mouse_event(mouse_event),

            // Key Events - Handle based on type
            Event::Key(key_event) => {
                match key_event {
                    // Navigation Events
                    KeyEvent::Tab | KeyEvent::BackTab | KeyEvent::Focus(_) => {
                        self.process_navigation_event(key_event)
                    },

                    // Frame-Specific Events
                    KeyEvent::Left | KeyEvent::Right | KeyEvent::Up | KeyEvent::Down | KeyEvent::Enter => {
                        self.process_frame_event(key_event)
                    },

                    // Global Hotkeys
                    KeyEvent::Space | KeyEvent::Quit | KeyEvent::Escape |
                    KeyEvent::Play | KeyEvent::Pause | KeyEvent::Stop |
                    KeyEvent::Next | KeyEvent::Previous |
                    KeyEvent::VolumeUp | KeyEvent::VolumeDown => {
                        self.process_hotkey_event(key_event)
                    },

                    // Other Key Events
                    _ => {
                        let _ = self.logger.log_debug("Processing other key event type");
                        self.focus_manager.handle_event(&event)
                            .map_err(|_| EventError::HandlingError("Failed to handle key event".to_string()))?;
                        self.update_focus_states();

                        if let Ok(action) = self.event_manager.dispatch_event(&event) {
                            let _ = self.logger.log_debug(&format!("Generated action from other event: {:?}", action));
                            self.component_manager.update_components(action);
                        }
                        Ok(())
                    },
                }
            },

            // System Events - Always process
            Event::System(_) => {
                let _ = self.logger.log_debug("Processing system event");
                if let Ok(action) = self.event_manager.dispatch_event(&event) {
                    let _ = self.logger.log_debug(&format!("Generated action from system event: {:?}", action));
                    self.component_manager.update_components(action);
                }
                Ok(())
            },

            // Other Events
            _ => {
                let _ = self.logger.log_debug("Processing other event type");
                self.focus_manager.handle_event(&event)
                    .map_err(|_| EventError::HandlingError("Failed to handle event".to_string()))?;
                self.update_focus_states();

                if let Ok(action) = self.event_manager.dispatch_event(&event) {
                    let _ = self.logger.log_debug(&format!("Generated action from other event: {:?}", action));
                    self.component_manager.update_components(action);
                }
                Ok(())
            },
        };

        let _ = self.logger.log_debug("=== Event Processing End ===");
        result
    }
}
