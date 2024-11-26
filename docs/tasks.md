# PlayTUI Master Task List

## Core Components

### PlayerComponent
- [x] Create basic playback structure
- [x] Implement basic state management
- [x] Add position tracking
- [ ] Initialize audio system (PipeWire)
- [ ] Handle playback controls (play/pause/stop)
- [ ] Manage audio stream
- [ ] Volume control
- [ ] Equalizer support
- Audio format support:
  - [x] Create format-specific decoder structures
  - [x] Implement format detection
  - [ ] FLAC frame decoding
  - [ ] MP3 frame parsing and Huffman decoding
  - [ ] OGG/Vorbis packet handling
  - [ ] WAV chunk processing and PCM decoding
- Audio stream optimization:
  - [ ] Real-time streaming
  - [ ] Buffer underrun protection
  - [ ] Sample rate conversion
  - [ ] Format conversion utilities

### PlaylistComponent
- [x] Create basic playlist view
- [x] Implement track selection interface
- [x] Directory scanning system
- [ ] Music library management
- [x] Playlist navigation
- [ ] Search capabilities
- [ ] Advanced sorting options
- Playlist persistence:
  - [ ] Save/load functionality
  - [ ] Format for storing music library data
  - [ ] Playlist state management

### UIComponent
- [x] Implement basic frame navigation
- [x] Add focus system with visual feedback
- [x] Create component layout structure (60/25/15 split)
- [ ] Optimize frame rendering for large playlists
- [ ] Implement high contrast themes
- [ ] Add configurable key bindings
- [ ] Add album art display
- [ ] Create lyrics view
- [ ] Add visualizer component
- Navigation system:
  - [x] Basic keyboard navigation
  - [x] Mouse event handling
  - [ ] Enhanced scroll navigation
  - [ ] Advanced focus management

### MetadataComponent
- [x] Create basic metadata structures
- [ ] Implement metadata system:
  - [ ] Generic parsing logic
  - [ ] Metadata caching system
  - [ ] Missing information handling
- Format-specific handlers:
  - [ ] ID3 tag handling
  - [ ] Vorbis comment support
  - [ ] FLAC metadata support
  - [ ] WAV metadata support

## Infrastructure

### Event System
- [x] Create Event enum for key/mouse/system events
- [x] Implement basic event handling
- [x] Add event filtering system
- [ ] Enhance event dispatcher:
  - [ ] Central event routing
  - [ ] Event correlation across components
- [ ] Expand event handlers:
  - [ ] Complex keyboard combinations
  - [ ] Advanced mouse interactions
  - [ ] System event handling
- Action system:
  - [x] Basic action definitions
  - [ ] Action prioritization
  - [ ] Action result handling
  - [ ] State update triggers

### State Management
- [x] Create basic state management system
- [x] Implement component state
- [ ] Enhance state system:
  - [ ] Global app state
  - [ ] State history for debugging
  - [ ] State transition paths
  - [ ] Immutable state updates
- State synchronization:
  - [ ] Cross-component state updates
  - [ ] State persistence
  - [ ] State recovery

### Error Handling
- [x] Add basic error types
- [ ] Expand error handling system:
  - [ ] Custom error types for each component
  - [ ] Error conversion traits
  - [ ] Recovery strategies
  - [ ] Error logging and metrics
- Component-specific errors:
  - [ ] Audio operation errors
  - [ ] File system errors
  - [ ] UI errors
  - [ ] State management errors

### Configuration
- [x] Set up basic theme support
- [ ] Implement configuration system:
  - [ ] User preferences management
  - [ ] Theme configuration
  - [ ] Key binding configuration
  - [ ] Audio settings
- Configuration persistence:
  - [ ] Save/load settings
  - [ ] Default configurations
  - [ ] Configuration validation

## Performance Optimization
- [x] Implement event batching
- [ ] Add performance improvements:
  - [ ] Lazy component updates
  - [ ] Render caching
  - [ ] Thread pooling for heavy operations
- Memory optimization:
  - [ ] Optimize metadata cache
  - [ ] Efficient state management
  - [ ] Buffer management
  - [ ] Resource cleanup

## Testing
- [x] Create basic test suite
- [x] Add format detection tests
- [x] Implement component state tests
- [ ] Expand component testing:
  - [ ] Unit tests for each component
  - [ ] Integration tests
  - [ ] Event handling tests
- Performance testing:
  - [ ] Startup time verification
  - [ ] Playback performance
  - [ ] UI responsiveness
  - [ ] Memory usage monitoring
- Error handling tests:
  - [ ] Recovery scenario tests
  - [ ] Error propagation tests
  - [ ] Boundary condition tests

## Documentation
- [x] Create initial project structure docs
- [x] Document OODA-based development guidelines
- Technical documentation:
  - [ ] Component API documentation
  - [ ] Event flow patterns
  - [ ] State management guide
  - [ ] Error handling guide
- User documentation:
  - [ ] Setup instructions
  - [ ] Keyboard shortcuts guide
  - [ ] Configuration guide
  - [ ] Troubleshooting guide
