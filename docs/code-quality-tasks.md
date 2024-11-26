# Code Quality Task List

> This document focuses on code quality improvements and technical debt management. For feature implementation tasks, see [tasks.md](tasks.md).

## High Priority

### Module Organization
- [x] Split controls.rs into smaller modules:
  - [x] Create controls/mod.rs for core traits and exports
  - [x] Create controls/layout.rs for rendering logic
  - [x] Create controls/events.rs for event handling
  - [x] Create controls/actions.rs for action handling
- [ ] Review and potentially split other modules exceeding 150 lines
  - Use ```bash``` to find lines exceeding 150 characters: ```find src -type f -name "*.rs" -exec wc -l {} \;```
  - [x] Split events/mod.rs (269 lines):
    - [x] Create events/error.rs for error handling
    - [x] Create events/types.rs for event type definitions
    - [x] Create events/actions.rs for action enums
    - [x] Create events/handler.rs for event handling
    - [x] Create events/filter.rs for event filtering
  - [ ] Split metadata/formats/vorbis.rs (208 lines):
    - [ ] Create metadata/formats/vorbis/parser.rs for core parser
    - [ ] Create metadata/formats/vorbis/tag_extractor.rs for tag handling
    - [ ] Create metadata/formats/vorbis/audio_properties.rs for audio analysis
    - [ ] Create metadata/formats/vorbis/tests.rs for tests
  - [ ] Split components/playlist.rs (200 lines):
    - [ ] Create components/playlist/mod.rs for core functionality
    - [ ] Create components/playlist/event_handler.rs for event handling
    - [ ] Create components/playlist/selection.rs for track selection
    - [ ] Create components/playlist/renderer.rs for UI rendering
  - [ ] Split theme.rs (191 lines):
    - [ ] Create theme/mod.rs for core functionality
    - [ ] Create theme/types.rs for data structures
    - [ ] Create theme/color.rs for color management
    - [ ] Create theme/style.rs for style management
  - [ ] Split app.rs (166 lines):
    - [ ] Create app/mod.rs for core functionality
    - [ ] Create app/components.rs for component management
    - [ ] Create app/event_handling.rs for event handling
    - [ ] Create app/focus.rs for focus management

### Core Audio Implementation
- [ ] Implement basic audio playback functionality in player.rs
- [ ] Complete audio stream handling in stream.rs
- [ ] Add proper error handling for audio operations
- [ ] Implement volume control system

### Format Decoders
- [ ] Complete FLAC decoder implementation:
  - [ ] Add FLAC-specific decoding state
  - [ ] Implement FLAC header reading
  - [ ] Implement actual FLAC decoding
- [ ] Complete MP3 decoder implementation:
  - [ ] Add MP3-specific decoding state
  - [ ] Implement MP3 header reading
  - [ ] Implement frame header parsing
- [ ] Complete OGG decoder implementation:
  - [ ] Add OGG/Vorbis-specific decoding state
  - [ ] Implement OGG/Vorbis header reading
  - [ ] Implement page validation
  - [ ] Implement Vorbis header parsing
- [ ] Complete WAV decoder implementation:
  - [ ] Add WAV-specific decoding state
  - [ ] Implement WAV header reading
  - [ ] Implement RIFF header validation
  - [ ] Implement fmt chunk parsing
  - [ ] Implement data chunk location

## Medium Priority

### Testing Infrastructure
- [ ] Add unit tests for components module
- [ ] Add integration tests for audio format decoders
- [ ] Add playback system tests
- [ ] Implement test utilities for audio format validation

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
- [ ] Keep development journal entries current
- [ ] Update architecture documentation as needed
- [ ] Maintain clear task progression records
- [ ] Document any deviations from guidelines

## Notes

- Priority levels may be adjusted based on project needs
- Tasks should be completed following OODA loop principles
- Regular reassessment of this list is recommended
- New tasks should be added as they are identified
- This list complements the main [tasks.md](tasks.md) which focuses on feature implementation
