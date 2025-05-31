# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PlayTUI is a terminal-based music player inspired by btop's aesthetic and interface design. It aims to be a visually appealing, keyboard-driven music player that runs in a terminal window.

## Key Design Goals

1. **btop-inspired UI** - Colorful, smooth, modern terminal interface
2. **Context-aware controls** - Different keys do different things based on which pane is focused
3. **Global shortcuts** - Some keys (like space for play/pause) work everywhere
4. **Three-pane layout** - File browser, playlist, and controls/info

## Architecture Lessons Learned

### What Works
- Ratatui for the TUI interface
- Component-based UI structure
- Theme system for customization
- Mouse support in addition to keyboard

### Pain Points to Avoid
- Over-engineering the event system
- Building components in isolation without integration
- Complex state management before basic functionality works
- Status bar implementation complexities

## Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Check for compilation issues
cargo check
```

## Input Handling Strategy

The app needs to handle:
1. **Global keys** - Work regardless of which pane has focus (space, volume, quit)
2. **Context keys** - Behave differently based on focused pane (arrows, enter)
3. **Focus switching** - Tab to cycle between panes

## MVP Features

1. Browse music files
2. Build a playlist
3. Play/pause/stop music
4. Volume control
5. Show current track info
6. Clean, btop-style interface

## Technical Choices

- **Audio**: rodio (simple, works out of the box)
- **TUI**: ratatui (great for btop-style interfaces)
- **File browsing**: walkdir
- **Metadata**: lofty (when needed)