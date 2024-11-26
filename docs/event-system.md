# Event System Architecture

## Overview

The event system in PlayTUI follows a multi-layered architecture that handles everything from raw terminal events to high-level component actions. This document describes the flow of events through the system and provides guidelines for implementing new event handlers and components.

## Event Flow

### 1. Raw Terminal Events
- Terminal events (key presses, mouse clicks) are captured in `main.rs`
- These raw events are mapped to our internal event types:
  ```rust
  match event::read()? {
      event @ CrosstermEvent::Key(key) => {
          if let Some(key_event) = map_key_event(key.code, key.modifiers) {
              app.handle_event(Event::Key(key_event))
          }
      }
      // ...
  }
  ```

### 2. Event Types
The system uses several event types that represent different stages of event processing:

1. **KeyEvent**: Keyboard input categorized by availability:
   ```rust
   enum KeyEvent {
       // Global Navigation Events (always available)
       Tab,
       BackTab,
       
       // Global Hotkeys (available regardless of focus)
       Quit,         // 'q'
       Escape,       // ESC key
       Space,        // Global pause/play
       
       // Frame-Specific Events (require focus)
       Enter,        // Activate selected item in focused frame
       Left,         // Navigate within focused frame
       Right,        // Navigate within focused frame
       Up,          // Navigate within focused frame
       Down,        // Navigate within focused frame
       
       // Other events...
   }
   ```

2. **Event**: High-level event types
   ```rust
   enum Event {
       Key(KeyEvent),
       Mouse(MouseEvent),
       Navigation(NavigationEvent),
       System(SystemEvent),
   }
   ```

3. **Action**: Component-specific actions
   ```rust
   enum Action {
       NavigateLeft,
       NavigateRight,
       Player(PlayerAction),
       UI(UIAction),
       // ...
   }
   ```

## Focus System

### Focus Management Rules

1. **Global Navigation**
   - Tab/BackTab events are always available globally
   - Used to change focus between frames
   - Focus cycles through frames in a predefined order
   - Visual feedback indicates the currently focused frame

2. **Frame-Specific Controls**
   - Arrow keys and Enter operate only in the focused frame
   - Each frame type handles these events differently:
     * Library Browser: Navigate directory structure, Enter to select
     * Play Controls: Navigate between buttons, Enter to activate
     * Volume Control: Left/Right arrows adjust volume

3. **Global Hotkeys**
   - Available regardless of focus state
   - Examples:
     * 'q' for quit
     * Escape for cancel/back
     * Space for global pause/play
     * Direct playback controls (play, stop, next, etc.)

### Focus Navigation Implementation

1. **Between Frames**
   ```rust
   impl FocusManager {
       pub fn should_process_event(&self, event: &Event, component_name: &str) -> bool {
           match event {
               // Global events always processed
               Event::Key(KeyEvent::Tab) |
               Event::Key(KeyEvent::BackTab) => true,
               
               // Frame-specific events require focus
               Event::Key(KeyEvent::Enter) |
               Event::Key(KeyEvent::Left) |
               Event::Key(KeyEvent::Right) |
               Event::Key(KeyEvent::Up) |
               Event::Key(KeyEvent::Down) => {
                   component_name == self.current_focus()
               },
               // ...
           }
       }
   }
   ```

2. **Within Frames**
   - Components handle their own internal navigation
   - Must check focus state before processing events
   - Provide visual feedback for navigation

## Best Practices

### Event Handler Implementation

1. **Focus Checking**
   ```rust
   fn can_handle(&self, event: &Event) -> bool {
       match event {
           // Global events
           Event::Key(KeyEvent::Tab) |
           Event::Key(KeyEvent::Quit) => true,
           
           // Frame-specific events
           Event::Key(KeyEvent::Enter) |
           Event::Key(KeyEvent::Left) |
           Event::Navigation(_) => self.focused(),
           
           _ => self.focused()
       }
   }
   ```

2. **Event Processing**
   ```rust
   fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
       match event {
           Event::Key(KeyEvent::Enter) if self.focused() => {
               // Handle activation in focused frame
           },
           Event::Key(KeyEvent::Left) if self.focused() => {
               // Handle left navigation in focused frame
           },
           Event::Key(KeyEvent::Tab) => {
               // Handle global navigation
           },
           // ...
       }
   }
   ```

### Component Guidelines

1. **Focus Management**
   - Implement clear focus indicators
   - Handle focus state changes appropriately
   - Update visual state based on focus

2. **Event Handling**
   - Respect focus rules for event processing
   - Handle global events appropriately
   - Provide clear visual feedback

3. **Navigation**
   - Implement consistent navigation patterns
   - Use arrow keys for internal navigation
   - Use Enter for activation/selection

## Testing

### Focus Testing
```rust
#[test]
fn test_event_processing_rules() {
    let component = MyComponent::new();
    
    // Global events should always process
    assert!(component.can_handle(&Event::Key(KeyEvent::Tab)));
    
    // Frame-specific events need focus
    component.set_focused(false);
    assert!(!component.can_handle(&Event::Key(KeyEvent::Enter)));
    
    component.set_focused(true);
    assert!(component.can_handle(&Event::Key(KeyEvent::Enter)));
}
```

### Navigation Testing
```rust
#[test]
fn test_navigation_behavior() {
    let mut component = MyComponent::new();
    component.set_focused(true);
    
    // Test internal navigation
    let result = component.handle_event(&Event::Key(KeyEvent::Right));
    assert!(result.is_ok());
    
    // Test activation
    let result = component.handle_event(&Event::Key(KeyEvent::Enter));
    assert!(result.is_ok());
}
```

## Common Issues and Solutions

1. **Events Not Reaching Components**
   - Verify focus state
   - Check event routing in FocusManager
   - Ensure proper event type categorization

2. **Inconsistent Navigation**
   - Follow focus rules strictly
   - Implement clear navigation patterns
   - Provide proper visual feedback

3. **Focus Issues**
   - Verify focus update propagation
   - Check focus state management
   - Test focus-dependent event handling
