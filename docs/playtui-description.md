# PlayTUI: Component-Based Terminal Music Player

## Project Overview
A terminal-based music player written in Rust using a component-driven architecture, featuring a colorful, retro-inspired interface for local media playback.

## Architecture Overview

### Core Components

#### 1. PlayerComponent
Responsible for audio playback functionality
- **State**:
  - Current playback status (playing/paused/stopped)
  - Current track information
  - Volume level
  - Playback progress
  - Recording status
  - Seek state (normal/fast-forward/rewind)
- **Events**:
  - Play/Pause/Stop/Eject commands
  - Record functionality
  - Fast Forward/Rewind operations
  - Next/Previous track
  - Volume adjustments
- **Actions**:
  - Initialize audio system (PipeWire)
  - Handle playback controls
  - Manage audio stream
  - Process recording requests
  - Handle seek operations
- **Features**:
  - Support for FLAC, MP3, OGG, WAV
  - Advanced playback controls
  - Volume management
  - Recording capability
  - Variable speed playback

#### 2. PlaylistComponent
Manages media library and playlist functionality
- **State**:
  - Current playlist
  - Selected track
  - Directory scan status
- **Events**:
  - Track selection
  - Directory scanning
  - Playlist navigation
- **Actions**:
  - Scan directories
  - Build playlists
  - Handle track selection
- **Features**:
  - Directory scanning
  - Playlist building
  - Metadata handling

#### 3. UIComponent
Handles the terminal user interface
- **State**:
  - Current view/layout
  - Selected UI element
  - Visual theme settings
  - Focus state
  - Button states (normal/active/pressed)
- **Events**:
  - Key presses
  - Window resizing
  - Focus changes
  - Button interactions
- **Actions**:
  - Render interface
  - Handle navigation
  - Update displays
  - Process button events
- **Features**:
  - Retro-inspired design
  - True color support
  - Nerd Font icons
  - ANSI art elements
  - Button highlight/shadow effects
  - Volume slider widget
  - 80/20 split control layout

#### 4. MetadataComponent
Manages audio file metadata
- **State**:
  - Current track metadata
  - Parsing status
  - Error states
- **Events**:
  - New file loading
  - Metadata updates
- **Actions**:
  - Parse metadata
  - Handle missing information
- **Features**:
  - Basic metadata reading
  - Error handling

### Component Event System

#### Event Flow and Message Passing
1. **Event Generation**
   - User Input → UIComponent (keyboard/mouse events)
   - System Events → Respective Components
   - Timer Events → App-level dispatcher

2. **Event Processing Chain**
   ```
   App
   ├── UIComponent
   │   └── Generates Actions (UserAction, NavigationAction, ButtonAction)
   ├── PlaylistComponent
   │   └── Generates Actions (TrackSelectedAction, PlaylistUpdateAction)
   ├── PlayerComponent
   │   └── Generates Actions (PlaybackAction, AudioStateAction, RecordAction, SeekAction)
   └── MetadataComponent
       └── Generates Actions (MetadataUpdateAction)
   ```

3. **Inter-Component Communication**
   - Components communicate through Actions
   - No direct component-to-component calls
   - Central event dispatcher pattern
   ```rust
   enum Action {
       Player(PlayerAction),
       Playlist(PlaylistAction),
       UI(UIAction),
       Metadata(MetadataAction),
       App(AppAction),
   }
   ```

## Technical Implementation

### Core Dependencies
- rataui: UI framework
- rodio: Audio playback
- id3: Metadata parsing
- walkdir: File system traversal
- log: Logging infrastructure
- anyhow: Error handling

### Component Trait Implementation
```rust
pub trait Component {
    fn init(&mut self) -> Result<()>;
    fn handle_events(&mut self, event: Option<Event>) -> Action;
    fn handle_key_events(&mut self, key: KeyEvent) -> Action;
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action;
    fn update(&mut self, action: Action) -> Action;
    fn render(&mut self, f: &mut Frame, rect: Rect);
}
```

### Error Handling
- Component-level error handling
- Graceful degradation
- Error propagation between components
- Structured logging with levels

## Testing Strategy

### Unit Testing
- Component-specific tests
- State management verification
- Event handling validation
- Button interaction testing
- Seek operation validation

### Integration Testing
- Cross-component communication
- Event flow verification
- State synchronization
- Recording functionality testing
- Playback control integration

### Performance Testing
- Component initialization time
- Event handling latency
- Rendering performance
- Seek operation responsiveness

## Performance Requirements
- Startup time < 1 second
- Smooth playback
- Responsive UI
- Efficient memory usage
- Fast seek operations

## Future Enhancements

### Playlist Management Component
- Save/Load functionality
- Advanced sorting
- Search capabilities

### Extended UI Components
- Album art display
- Lyrics view
- Visualizer component
- Advanced recording interface
- Seek position indicator

### Accessibility Features
- High contrast themes
- Configurable bindings
- Screen reader support

## Documentation Requirements
- Component API documentation
- Event flow documentation
- Setup instructions
- Keyboard shortcuts guide
- Recording feature guide

## Security Considerations
- File system access control
- Resource usage limits
- Input validation
- Recording storage management
