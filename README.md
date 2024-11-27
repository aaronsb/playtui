[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/N4N416MTT1)

# PlayTUI 🎵

A colorful, retro-inspired terminal music player written in Rust, featuring a rich component-driven architecture and an immersive text user interface. PlayTUI brings the joy of classic music players to your terminal with modern functionality and style.

![Status: Active Development](https://img.shields.io/badge/Status-Active%20Development-brightgreen)

## ✨ Features

### 🎧 Audio Playback
- Support for multiple audio formats:
  - FLAC (Free Lossless Audio Codec)
  - MP3 (MPEG Layer-3)
  - OGG/Vorbis
  - WAV (Waveform Audio)
- Real-time audio streaming
- Volume control with visual slider
- Advanced playback controls (play, pause, stop, seek)
- Recording capability (coming soon)

### 🎨 User Interface
- Retro-inspired design with modern aesthetics
- True color support with customizable themes
- Nerd Font icons for visual richness
- 60/25/15 split layout design:
  - Library browser and track details (60%)
  - Current track information (25%)
  - Playback controls (15%)
- Mouse and keyboard navigation
- Focus-based navigation system with visual feedback

### 📚 Library Management
- Directory-based music library browsing
- Playlist management
- Metadata display and management
- Search capabilities (coming soon)
- Advanced sorting options (coming soon)

## 🎮 Controls

### Navigation
- `Tab`/`Shift+Tab`: Navigate between components
- `Arrow keys`: Navigate within components
- `Enter`: Activate selected item
- `Mouse`: Click to select and activate

### Playback
- `Space`: Play/Pause
- `⏵`: Play
- `⏸`: Pause
- `⏹`: Stop
- `⏮`: Previous Track
- `⏭`: Next Track
- `⏪`: Rewind
- `⏩`: Fast Forward
- `q`: Quit

## 🛠️ Development

### Project Structure
```
src/
  ├── app/          # Application core and lifecycle management
  ├── audio/        # Audio playback and format processing
  ├── components/   # UI components and widgets
  ├── events/       # Event system and action handling
  ├── metadata/     # Audio metadata parsing and caching
  ├── state/        # Application state management
  └── theme/        # Theme system and styling
```

### Building and Running
```bash
# Build the project
cargo build

# Run in development mode
cargo run

# Run tests
cargo test
```

## 📚 Documentation

Comprehensive documentation is available in the docs/ directory:

- [Event System Architecture](docs/event-system.md) - Deep dive into the event system, state management, and component interactions
- [Development Guidelines](docs/ooda-based-development-guidelines.md) - OODA-based development practices and patterns
- [Theme Specification](docs/theme-specification.md) - Theming system and customization
- [UI Layout](docs/ui-layout.md) - Detailed UI structure and navigation flow
- [Play Feature Flow](docs/play-feature-flow.md) - Music playback architecture
- [Tasks](docs/tasks.md) - Development roadmap and task tracking

## 🎨 Themes

PlayTUI comes with several built-in themes:
- Default (balanced, modern look)
- Cobalt (cool blue tones)
- Copper (warm, retro feel)
- Monokai (dark, vibrant colors)

Custom themes can be created by following the [Theme Specification](docs/theme-specification.md).

## 🧪 Testing

PlayTUI maintains a comprehensive test suite:
- Unit tests for individual components
- Integration tests for system interactions
- Audio format validation tests
- Performance benchmarks

Run all tests with:
```bash
cargo test
```

## 🚀 Future Enhancements

- Album art display
- Lyrics view
- Audio visualizer
- Advanced playlist management
- Extended metadata support
- Screen reader accessibility
- Performance optimizations

## 🤝 Contributing

Contributions are welcome! Please check our [Development Guidelines](docs/ooda-based-development-guidelines.md) for code style and architecture patterns.

## 📝 License

This project is open source and available under the MIT license.
