[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/N4N416MTT1)
# PlayTUI

A terminal-based audio player with a rich text user interface.

## Documentation

- [Event System Architecture](docs/event-system.md) - Comprehensive guide to the event system, state management, and component interactions
- [Development Guidelines](docs/ooda-based-development-guidelines.md)
- [Theme Specification](docs/theme-specification.md)
- [UI Layout](docs/ui-layout.md)

## Features

- File browser for navigating audio files
- Playback controls (play, pause, stop, seek)
- Volume control
- Track information display
- Theme support
- Mouse and keyboard navigation

## Development

### Event System

PlayTUI uses a sophisticated event system for handling user input and component interactions. Key features:
- Event flow from raw terminal input to component actions
- Focus management for keyboard navigation
- State sharing between components
- Comprehensive testing support

See the [Event System Architecture](docs/event-system.md) documentation for detailed information about implementing new components and event handlers.

### Project Structure

```
src/
  ├── app/          # Application core and management
  ├── audio/        # Audio playback and processing
  ├── components/   # UI components
  ├── events/       # Event system
  ├── metadata/     # Audio metadata handling
  ├── state/        # Application state
  └── theme/        # Theme system
```

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## Controls

- Tab/Shift+Tab: Navigate between components
- Arrow keys: Navigate within components
- Enter: Activate selected item
- Space: Play/Pause
- q: Quit
