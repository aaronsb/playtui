# Code Quality Task List

> This document focuses on code quality improvements and technical debt management. For feature implementation tasks, see [tasks.md](tasks.md).

:heavy_check_mark: Important! Make sure between each module organization task - the parent module that exceeds our line length limit, that you run tests and commit your changes so that you can revert back to a clean state if needed.

## High Priority

### Module Organization
- [x] Split app/mod.rs (300 lines):
  - [x] Create app/state.rs for application state management
  - [x] Create app/initialization.rs for startup logic
  - [✓] Decided to keep rendering logic in ui.rs (see dev-11.md)
  - [x] Create app/lifecycle.rs for application lifecycle management

- [x] Split components/track_list.rs (298 lines):
  - [x] Create components/track_list/mod.rs for core functionality
  - [x] Create components/track_list/view.rs for view logic
  - [x] Create components/track_list/state.rs for state management
  - [x] Create components/track_list/actions.rs for action handling

- [x] Split components/library_browser.rs (251 lines):
  - [x] Create components/library_browser/mod.rs for core functionality
  - [x] Create components/library_browser/view.rs for view logic
  - [x] Create components/library_browser/state.rs for state management
  - [x] Create components/library_browser/events.rs for event handling

- [ ] Split components/tests.rs (237 lines):
  - [ ] Create components/tests/mod.rs for test organization
  - [ ] Create components/tests/library_tests.rs for library component tests
  - [ ] Create components/tests/track_tests.rs for track-related tests
  - [ ] Create components/tests/control_tests.rs for control component tests

- [ ] Split components/controls/events.rs (194 lines):
  - [ ] Create components/controls/events/mod.rs for event handling core
  - [ ] Create components/controls/events/handlers.rs for specific event handlers
  - [ ] Create components/controls/events/types.rs for event type definitions
  - [ ] Create components/controls/events/processing.rs for event processing logic

- [x] Split components/volume_control/mod.rs (177 lines):
  - [x] Create components/volume_control/view.rs for rendering logic
  - [x] Create components/volume_control/state.rs for state management
  - [x] Create components/volume_control/events.rs for event handling

- [ ] Split app/focus.rs (176 lines):
  - [ ] Create app/focus/mod.rs for core focus management
  - [ ] Create app/focus/state.rs for focus state handling
  - [ ] Create app/focus/navigation.rs for focus navigation logic
  - [ ] Create app/focus/events.rs for focus-related events

- [x] Split app/components.rs (200 lines):
  - [x] Create app/components/mod.rs for component management core
  - [x] Create app/components/registry.rs for component registration
  - [x] Create app/components/lifecycle.rs for component lifecycle
  - [x] Create app/components/relationships.rs for component relationships

[rest of file unchanged]
