# PlayTUI Project Backlog

## Overview
A terminal-based music player written in Rust that plays local media files with a colorful, retro-inspired interface.

## Core Features

### Media Playback
- Support for common audio formats:
  - ✅ FLAC
  - ⬜ MP3
  - ⬜ OGG
  - ⬜ WAV
- Basic playback controls:
  - ✅ Play/Pause
  - ✅ Stop
  - ✅ Next/Previous track
  - ✅ Volume control
- Track information display:
  - ✅ Title
  - ✅ Artist
  - ✅ Duration
  - ✅ Progress

### File Management
- ✅ Scan current directory and subdirectories for supported audio files
- ✅ Build a simple playlist from found media files
- ✅ Read basic metadata from audio files
- ✅ Handle missing or incomplete metadata gracefully

### User Interface
- ✅ Built using rataui library
- Retro-inspired design:
  - ✅ True color support
  - ✅ Nerd Font icons
  - ⬜ ANSI art elements
- Essential UI components:
  - ✅ Playlist view
  - ✅ Now playing information
  - ✅ Progress bar
  - ✅ Basic controls display
  - ✅ Volume indicator

### Audio System
- ✅ Integration with PipeWire on Arch Linux
- ✅ Basic audio stream management
- ✅ Clean playback initialization and shutdown

### Technical Foundation
- ✅ Clear separation between:
  - ✅ UI rendering
  - ✅ Audio playback
  - ✅ File management
- ✅ Error handling for common issues:
  - ✅ File not found
  - ✅ Unsupported format
  - ✅ Audio system errors
- ✅ Simple logging for debugging

### Controls
- Primary navigation:
  - ✅ Arrow keys (↑,↓,←,→)
  - ✅ Vim-style keys (h,j,k,l)
  - ✅ Enter: Activate selected item/button
- Playback controls:
  - ✅ Space: Play/Pause
  - ✅ Next track (.)
  - ✅ Previous track (,)
  - ✅ Quit (q)
- Volume adjustment:
  - ✅ +: Volume up
  - ✅ -: Volume down
- Navigation behavior:
  - ✅ Up/k: Move selection up
  - ✅ Down/j: Move selection down
  - ✅ Left/h: Previous menu/section
  - ✅ Right/l: Next menu/section or activate item

## Constraints
- ✅ Must run on Arch Linux
- ✅ Terminal-based interface only
- ✅ Focus on local file playback
- ✅ No network features
- ✅ No external services integration

## Performance Goals
- ✅ Quick startup (< 1 second)
- ✅ Smooth playback without interruptions
- ✅ Responsive UI (no noticeable lag)
- ✅ Efficient memory usage

## Error Handling
- Graceful handling of:
  - ✅ Corrupted audio files
  - ✅ System audio device changes
  - ✅ Memory constraints
  - ✅ File permission issues
- ✅ User-friendly error messages
- Recovery mechanisms:
  - ✅ Auto-skip corrupted files
  - ⬜ Retry mechanisms for temporary failures
  - ⬜ State recovery after crashes
- Logging levels:
  - ✅ ERROR: Critical failures
  - ✅ WARN: Recoverable issues
  - ✅ INFO: Standard operations
  - ✅ DEBUG: Detailed diagnostics

## Dependencies
### Required System Components
- ✅ Rust toolchain (2021 edition or newer)
- ✅ PipeWire audio system
- ✅ Terminal with:
  - ✅ True color support
  - ✅ Unicode compatibility
  - ✅ Nerd Font support
### Rust Crates
- ✅ rataui: UI framework
- ✅ rodio: Audio playback
- ✅ id3: Metadata parsing
- ✅ walkdir: File system traversal
- ✅ log: Logging infrastructure
- ✅ anyhow: Error handling

## Testing Requirements
### Unit Testing
- Component-level tests:
  - ⬜ Audio playback functions
  - ⬜ Metadata parsing
  - ⬜ File management
  - ⬜ UI state management
### Integration Testing
- ⬜ End-to-end scenarios
- ⬜ Cross-component interactions
- ⬜ Audio system integration
### Performance Testing
- ⬜ Startup time measurement
- ⬜ Memory usage monitoring
- ⬜ UI responsiveness tests
- ⬜ Large playlist handling

## Accessibility Features
- ⬜ High contrast color schemes
- ⬜ Configurable key bindings
- ⬜ Screen reader compatibility
- ✅ Clear status messages
- ⬜ Adjustable text size support
- ⬜ Alternative navigation methods

## Security Considerations
- ✅ File system access restrictions
- ✅ Input sanitization
- ✅ Resource usage limits
- ✅ Secure error logging
- ✅ Safe handling of file metadata

## Future Enhancements
### Playlist Management
- Save/Load features:
  - ⬜ Save playlists
  - ⬜ Load playlists
  - ⬜ Custom sorting options
  - ⬜ Search functionality
- Advanced playback:
  - ⬜ Shuffle mode
  - ⬜ Repeat modes
  - ⬜ A-B repeat
- Extended metadata:
  - ⬜ Album art in terminal
  - ⬜ Lyrics display
  - ⬜ Detailed track information

## Documentation
- ✅ Basic usage instructions
- ✅ Installation steps
- ✅ List of keyboard shortcuts
- ✅ System requirements
