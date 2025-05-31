# PlayTUI

A colorful, retro-inspired terminal music player with a cassette tape visualization.

![PlayTUI Demo](demo.gif)

## Features

- 🎵 Multi-format audio playback (MP3, FLAC, OGG, WAV, AAC, ALAC)
- 📼 Animated cassette tape visualization showing current track
- 🎨 Clean, btop-inspired terminal interface
- ⌨️ Context-aware keyboard controls
- 📁 Built-in file browser
- 🎶 Playlist management
- 🔊 Volume control
- ⏩ Real-time playback progress
- 🎯 Auto-advance to next track

## Installation

### Prerequisites

- Rust 1.70 or higher
- A terminal with Unicode support

### Build from source

```bash
git clone https://github.com/yourusername/playtui.git
cd playtui
cargo build --release
```

The binary will be available at `target/release/playtui`.

## Usage

```bash
playtui
```

### Keyboard Controls

#### Global Controls (work anywhere)
- `Space` - Play/Pause
- `+`/`-` - Volume Up/Down
- `Tab` - Switch between panes
- `q` - Quit

#### File Browser (when focused)
- `↑`/`↓` - Navigate files
- `Enter` - Add file to playlist or enter directory

#### Playlist (when focused)  
- `↑`/`↓` - Navigate playlist
- `Enter` - Play selected track
- `d` or `Delete` - Remove from playlist
- `c` - Clear entire playlist
- `←`/`→` - Previous/Next track

## Layout

```
┌─────────────────┬────────────────────────────┬──────────────────┐
│ Library         │      Now Playing           │ Playlist         │
│                 │  ┌──────────────────────┐  │                  │
│ 📁 Music        │  │ A  Current Track     │  │ ▶ Track 1       │
│  └ Artist       │  │    Next Track        │  │   Track 2       │  
│    └ Album      │  │  ◉  ████████████  ◉  │  │   Track 3       │
│                 │  └──────────────────────┘  │                  │
│                 │      ▶ Playing              │                  │
│                 │  ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬   │                  │
│                 │  02:45 ████████░░ 04:15    │                  │
│                 │  Volume: ████████░░ 80%    │                  │
└─────────────────┴────────────────────────────┴──────────────────┘
Enter: Add to playlist | Tab: Switch | +/-: Volume | q: Quit
```

## Configuration

Theme files are stored in the `themes/` directory. The default theme can be changed by modifying the theme selection in the code.

## Building and Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Build for release
cargo build --release
```

## Architecture

PlayTUI uses:
- **Ratatui** for the terminal UI
- **Symphonia** for audio decoding
- **CPAL** for cross-platform audio output
- **Crossterm** for terminal input handling

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Inspired by [btop](https://github.com/aristocratos/btop) for its beautiful terminal interface
- Built with the excellent [Ratatui](https://github.com/ratatui-org/ratatui) TUI framework