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

- [ ] Review app/event_handling.rs (204 lines):
  - [ ] Evaluate splitting into event processing and dispatch
  - [ ] Consider creating separate error handling module
  - [ ] Assess if some functionality belongs in events module

- [ ] Split components/controls/events/handlers.rs (166 lines):
  - [ ] Create separate files for different event categories
  - [ ] Extract common handler utilities
  - [ ] Consider creating handler factory pattern

### Module Organization Guidelines
1. File Size Assessment:
   - Base splitting decisions on complexity and cohesion, not just line count
   - The 150-line threshold is a guideline, not a strict rule
   - Files may exceed 150 lines if they maintain:
     * Single, focused purpose
     * Clear internal organization
     * Strong component cohesion
     * Manageable complexity
   - Immediate review required when files exceed 300 lines
2. When to Split Modules:
   - Primary Indicators (any of these suggest splitting):
     * Multiple distinct responsibilities
     * Complex state management across different concerns
     * Multiple independent feature sets
     * Test file size exceeding implementation size
   - Secondary Indicators (consider in combination):
     * File size over 150 lines
     * Multiple levels of nested logic
     * Complex event handling paths
     * Extensive configuration options
3. When to Keep Modules Together:
   - Keep together when:
     * Logic is tightly coupled
     * Splitting would increase complexity
     * Implementation and tests are closely related
     * Component has a single, well-defined purpose
   - Examples from current codebase:
     * track_list.rs: Exceeds line count but maintains focused purpose
     * library_browser.rs: Complex but cohesive file system navigation
     * volume_control/mod.rs: Slightly over limit but logically cohesive
     * focus.rs: Tightly coupled focus management system
4. Testing Organization:
   - Test Location Patterns:
     * Same-file tests: Use #[cfg(test)] for simple, focused tests
     * Parallel test file: module.rs → module_tests.rs
     * Test directory: module/tests/{unit,integration}_tests.rs
     * Integration tests: tests/integration/{feature}_tests.rs
   - Test Naming and Structure:
     * Clear relationship to tested module:
       - Direct: user.rs → user_tests.rs
       - Categorized: user_unit_tests.rs, user_integration_tests.rs
     * Group related tests in nested modules
     * Use descriptive test function names
     * Document test categories and setups
   - When to Keep Tests in Module:
     * Tests are simple and focused
     * Test setup is minimal
     * Tests directly verify basic behavior
     * Total test code is less than implementation
   - When to Split Tests:
     * Test code exceeds implementation code
     * Complex test setup required
     * Multiple test categories exist
     * Integration tests spanning components
     * Performance or specialized testing needed
5. Monitoring and Maintenance:
   - Regular line count checks using: find src -type f -name "*.rs" -exec wc -l {} \;
   - Review during code reviews
   - Document reasons for keeping larger files
   - Track module growth patterns
   - Consider splitting when complexity increases

## Medium Priority
### Testing Infrastructure
- [x] Add unit tests for components module
- [x] Add integration tests for audio format decoders
- [x] Add playback system tests
- [x] Implement test utilities for audio format validation
### Documentation
- [ ] Add implementation details to existing TODO comments
- [ ] Document audio format specifications
- [ ] Create debugging guide for audio issues
- [ ] Update development guidelines with audio system specifics
### Performance Optimization
- [ ] Add position tracking for audio playback
- [ ] Optimize audio buffer management
- [ ] Implement efficient seeking in audio streams
- [ ] Add caching for frequently accessed audio metadata
## Low Priority
### UI Enhancements
- [ ] Complete volume slider widget
- [ ] Add visual feedback for audio seeking
- [ ] Implement hover states for controls
- [ ] Add button press animations
### Code Quality
- [ ] Review and update error messages
- [ ] Add debug logging throughout audio system
- [ ] Standardize comment formatting
- [ ] Clean up unused imports
## Monitoring Tasks
### Regular Reviews
- [ ] Weekly code quality metrics check
- [ ] Monthly review of module sizes
- [ ] Regular assessment of TODO comment status
- [ ] Performance benchmark tracking
### Documentation Updates
- [x] Keep development journal entries current
- [ ] Update architecture documentation as needed
- [ ] Maintain clear task progression records
- [ ] Document any deviations from guidelines
## Notes
- Priority levels may be adjusted based on project needs
- Tasks should be completed following OODA loop principles
- Regular reassessment of this list is recommended
- New tasks should be added as they are identified
- This list complements the main [tasks.md](tasks.md) which focuses on feature implementation
