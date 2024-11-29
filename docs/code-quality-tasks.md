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

- [x] Split components/tests.rs (237 lines):
  - [x] Create components/tests/mock.rs for mock component implementation
  - [x] Create components/tests/theme.rs for theme creation and testing utilities
  - [x] Create components/tests/events.rs for event handling tests
  - [x] Create components/tests/state.rs for component state tests
  - [x] Create components/tests/mod.rs for test organization

- [x] Split components/controls/events.rs (194 lines):
  - [x] Create components/controls/events/mod.rs for event handling core
  - [x] Create components/controls/events/handlers.rs for specific event handlers
  - [x] Create components/controls/events/types.rs for event type definitions
  - [x] Create components/controls/events/processing.rs for event processing logic

- [x] Split components/volume_control/mod.rs (177 lines):
  - [x] Create components/volume_control/view.rs for rendering logic
  - [x] Create components/volume_control/state.rs for state management
  - [x] Create components/volume_control/events.rs for event handling

- [✓] Review app/focus.rs (176 lines):
  - [✓] Decided to keep as single file due to high cohesion and tight coupling
  - [✓] Added comprehensive documentation explaining the decision
  - [✓] File maintains single responsibility and clear organization
  - [✓] Splitting would increase complexity without clear benefits

- [x] Split app/components.rs (200 lines):
  - [x] Create app/components/mod.rs for component management core
  - [x] Create app/components/registry.rs for component registration
  - [x] Create app/components/lifecycle.rs for component lifecycle
  - [x] Create app/components/relationships.rs for component relationships

- [x] Split preferences/manager.rs (299 lines):
  - [x] Create preferences/manager/mod.rs for core functionality
  - [x] Create preferences/manager/save_handler.rs for save operations
  - [x] Create preferences/manager/tests.rs for test organization

- [x] Review app/event_handling.rs (204 lines):
  - [x] Split into event_processing.rs and event_dispatch.rs
  - [x] Consolidated error handling into events/error.rs
  - [x] Moved event-related functionality to events module
  - [x] Improved component update mechanism
  - [x] Fixed recursive type issues in error handling
  - [x] Enhanced event propagation system

- [ ] Split components/controls/events/handlers.rs (166 lines):
  - [ ] Create separate files for different event categories
  - [ ] Extract common handler utilities
  - [ ] Consider creating handler factory pattern

[Rest of file unchanged...]
