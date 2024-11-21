# PlayTUI - Terminal-based Music Player

A modern, feature-rich terminal user interface (TUI) music player written in Rust. PlayTUI provides a retro-inspired, colorful interface for managing and playing your local music collection.

![PlayTUI Interface Description]
- Top: Now Playing bar with track info and progress
- Middle: Three-panel view with file browser, song list, and playlist
- Bottom: Controls and volume indicator

## Features

- 🎵 Supports common audio formats (MP3, OGG, FLAC, WAV)
- 🎨 Colorful, retro-inspired terminal interface
- 📁 File system browser for easy navigation
- 📋 Playlist management
- 🎚️ Essential playback controls
- 📊 Real-time progress tracking
- 🔊 Volume control
- 🎯 Multi-panel focused interface

## Installation

### Prerequisites
- Rust toolchain (2021 edition or newer)
- PipeWire audio system
- Terminal with:
  - True color support
  - Unicode compatibility
  - Nerd Font support

### Building from Source
```bash
git clone [repository-url]
cd playtui
cargo build --release
```

The compiled binary will be available in `target/release/playtui`.

## Usage

Launch PlayTUI from your terminal:
```bash
./playtui
```

### Interface Layout

1. **Now Playing Bar** (Top)
   - Current track information
   - Playback status (▶️ Playing, ⏸️ Paused, ⏹️ Stopped)
   - Progress bar with time indicators

2. **File Browser** (Left Panel)
   - Navigate your file system
   - Shows directories with 📁 icons
   - Parent directory accessible via ".."

3. **Songs List** (Middle Panel)
   - Shows songs in current directory
   - Displays title and artist information
   - Currently playing track highlighted in green

4. **Playlist** (Right Panel)
   - Custom playlist management
   - Add/remove songs from current directory
   - Persistent playback queue

### Controls

#### Navigation
- `↑`/`↓` or `k`/`j`: Move selection up/down
- `Tab`: Cycle focus between panels (Browser → Songs → Playlist)
- `Shift+Tab`: Reverse cycle focus
- `Enter`: Enter directory/Play selected song

#### Playback
- `Space`: Play/Pause
- `.`: Next track
- `,`: Previous track
- `s`: Stop playback

#### Playlist Management
- `→` or `l`: Add selected song to playlist
- `←` or `h`: Remove selected song from playlist
- `a`: Add all songs to playlist
- `c`: Clear playlist

#### Volume
- `+`: Increase volume
- `-`: Decrease volume

#### Other
- `m`: Toggle menu
- `q`: Quit application

## Technical Details

### Dependencies
- `ratatui`: Terminal user interface framework
- `crossterm`: Terminal manipulation
- `rodio`: Audio playback
- `id3`: Metadata parsing
- `walkdir`: File system traversal
- `log`: Logging infrastructure
- `anyhow`: Error handling

### Features
- Efficient file system traversal
- Real-time audio playback
- Metadata extraction from audio files
- Memory-efficient playlist management
- Responsive terminal UI
- Error resilient design

## Error Handling

PlayTUI includes robust error handling for common scenarios:
- Corrupted audio files
- Missing metadata
- File system permission issues
- Audio device changes
- Resource constraints

## Performance

- Quick startup time (< 1 second)
- Smooth playback without interruptions
- Responsive UI with no noticeable lag
- Efficient memory usage
- Handles large music collections

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[License Information]
