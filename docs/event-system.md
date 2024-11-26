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

1. **KeyEvent**: Raw keyboard input
   ```rust
   enum KeyEvent {
       Char(char),
       Enter,
       Left,
       Right,
       Up,
       Down,
       Tab,
       BackTab,
       // ...
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

### 3. Event Processing Chain

1. **Event Manager** (`EventManager`)
   - Receives raw events from the application
   - Converts events to actions
   - Example: `KeyEvent::Left` → `Action::NavigateLeft`

2. **Component Manager** (`ComponentManager`)
   - Receives actions from Event Manager
   - Converts actions back to events for component processing
   - Example: `Action::NavigateLeft` → `Event::Navigation(NavigationEvent::Left)`
   - Distributes events to appropriate components

3. **Component Event Handlers**
   - Process events specific to their functionality
   - Update internal state
   - Return follow-up actions if needed

## Focus System

### Focus Management

The focus system determines which component receives certain events:

1. **Global Focus** (`FocusManager`)
   - Manages which component has primary focus
   - Handles Tab/BackTab navigation between components
   - Updates UI state to reflect focused component

2. **Component Focus**
   - Components can be focused/unfocused
   - Navigation events work regardless of focus state
   - Other events (keys, mouse) require focus

### Focus Navigation Rules

1. **Between Components**
   - Tab/BackTab cycles through focusable components
   - Focus order is defined in `FocusManager`
   - Components update their visual state based on focus

2. **Within Components**
   - Arrow keys navigate within focused component
   - Navigation works even when component isn't focused
   - Visual feedback shows current focused element

## State Management

### Component State Sharing

Components use `Rc<RefCell>` for shared state:
```rust
pub struct App {
    pub controls: Rc<RefCell<Controls>>,
    // ...
}
```

This ensures:
- Multiple managers can access the same component instance
- State changes are immediately visible across the system
- Thread-safe state management in a single-threaded context

### State Update Flow

1. Event triggers state change in component
2. Component updates its internal state
3. Component returns Action::Refresh if needed
4. UI automatically reflects state changes through shared references

## Best Practices

### Implementing New Events

1. **Define the Event Type**
   ```rust
   enum CustomEvent {
       EventTypeOne,
       EventTypeTwo(CustomData),
   }
   ```

2. **Add to Event Enum**
   ```rust
   enum Event {
       // Existing events...
       Custom(CustomEvent),
   }
   ```

3. **Implement Event Handler**
   ```rust
   fn handle_event(&mut self, event: Event) -> Option<Action> {
       match event {
           Event::Custom(custom_event) => handle_custom_event(custom_event),
           // ...
       }
   }
   ```

### Creating New Components

1. **Implement Component Trait**
   ```rust
   impl Component for NewComponent {
       fn new() -> Self;
       fn handle_event(&mut self, event: Event) -> Option<Action>;
       fn focused(&self) -> bool;
       fn set_focused(&mut self, focused: bool);
   }
   ```

2. **Define State Management**
   - Use `ComponentState` for focus management
   - Implement clear state update methods
   - Consider what state should be shared

3. **Handle Navigation**
   - Allow navigation events regardless of focus
   - Block other events when not focused
   - Implement proper focus visualization

### Event Handler Guidelines

1. **Focus Handling**
   ```rust
   fn can_handle(&self, event: &Event) -> bool {
       match event {
           Event::Navigation(_) => true,  // Always handle navigation
           _ => self.focused()            // Other events need focus
       }
   }
   ```

2. **State Updates**
   - Keep state updates atomic
   - Return Action::Refresh when visual update needed
   - Use proper error handling for state changes

3. **Event Processing**
   - Handle one event type per match arm
   - Consider component state in event handling
   - Document event handling behavior

## Testing

### Event Flow Testing

1. **Test Navigation Events**
   ```rust
   #[test]
   fn test_navigation_events_work_regardless_of_focus() {
       let mut component = Component::new();
       component.set_focused(false);
       let result = component.handle_event(Event::Navigation(NavigationEvent::Right));
       assert_eq!(result, Some(Action::Refresh));
   }
   ```

2. **Test Focus Behavior**
   ```rust
   #[test]
   fn test_key_events_blocked_when_not_focused() {
       let mut component = Component::new();
       component.set_focused(false);
       let result = component.handle_event(Event::Key(KeyEvent::Enter));
       assert_eq!(result, None);
   }
   ```

### Component Testing

1. Test state updates
2. Test focus management
3. Test event handling
4. Test visual rendering

## Common Issues and Solutions

1. **Events Not Reaching Components**
   - Verify event mapping in main.rs
   - Check focus state
   - Ensure proper event conversion

2. **State Updates Not Visible**
   - Verify Rc<RefCell> usage
   - Check if Action::Refresh is returned
   - Verify render method updates

3. **Focus Issues**
   - Check FocusManager configuration
   - Verify focus update propagation
   - Test focus-dependent event handling
