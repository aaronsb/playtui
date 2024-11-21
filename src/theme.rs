use std::fs;
use serde::{Deserialize, Serialize};
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub modifiers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorderStyleConfig {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub modifiers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FocusableBorderStyleConfig {
    pub unfocused: StyleConfig,
    pub focused: StyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockConfig {
    pub borders: bool,
    pub border_style: BorderStyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FocusableBlockConfig {
    pub borders: bool,
    pub border_style: FocusableBorderStyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NowPlayingConfig {
    pub name: String,
    pub block: BlockConfig,
    pub style: StyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressGaugeConfig {
    pub name: String,
    pub block: BlockConfig,
    pub gauge_style: StyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressTextConfig {
    pub name: String,
    pub block: BlockConfig,
    pub style: StyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWidgetConfig {
    pub name: String,
    pub block: FocusableBlockConfig,
    pub list_style: StyleConfig,
    pub highlight_style: StyleConfig,
    pub highlight_symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableListWidgetConfig {
    pub name: String,
    pub block: FocusableBlockConfig,
    pub list_style: StyleConfig,
    pub highlight_style: StyleConfig,
    pub playing_style: StyleConfig,
    pub highlight_symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlsConfig {
    pub name: String,
    pub block: BlockConfig,
    pub style: StyleConfig,
    pub volume_style: StyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuConfig {
    pub name: String,
    pub block: BlockConfig,
    pub style: StyleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Widgets {
    pub widgets: WidgetConfigs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WidgetConfigs {
    pub now_playing: NowPlayingConfig,
    pub progress_gauge: ProgressGaugeConfig,
    pub progress_text: ProgressTextConfig,
    pub browser: ListWidgetConfig,
    pub songs: PlayableListWidgetConfig,
    pub playlist: PlayableListWidgetConfig,
    pub controls: ControlsConfig,
    pub menu: MenuConfig,
}

#[derive(Debug)]
pub struct Theme {
    config: Widgets,
}

impl Theme {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("theme.json")?;
        let config: Widgets = serde_json::from_str(&content)?;
        Ok(Self { config })
    }

    fn parse_color(color_str: &str) -> Color {
        match color_str {
            "Black" => Color::Black,
            "Red" => Color::Red,
            "Green" => Color::Green,
            "Yellow" => Color::Yellow,
            "Blue" => Color::Blue,
            "Magenta" => Color::Magenta,
            "Cyan" => Color::Cyan,
            "Gray" => Color::Gray,
            "DarkGray" => Color::DarkGray,
            "LightRed" => Color::LightRed,
            "LightGreen" => Color::LightGreen,
            "LightYellow" => Color::LightYellow,
            "LightBlue" => Color::LightBlue,
            "LightMagenta" => Color::LightMagenta,
            "LightCyan" => Color::LightCyan,
            "White" => Color::White,
            _ => Color::Reset,
        }
    }

    fn parse_modifiers(modifiers: &[String]) -> ratatui::style::Modifier {
        let mut result = ratatui::style::Modifier::empty();
        for modifier in modifiers {
            match modifier.as_str() {
                "BOLD" => result |= Modifier::BOLD,
                "DIM" => result |= Modifier::DIM,
                "ITALIC" => result |= Modifier::ITALIC,
                "UNDERLINED" => result |= Modifier::UNDERLINED,
                "SLOW_BLINK" => result |= Modifier::SLOW_BLINK,
                "RAPID_BLINK" => result |= Modifier::RAPID_BLINK,
                "REVERSED" => result |= Modifier::REVERSED,
                "HIDDEN" => result |= Modifier::HIDDEN,
                "CROSSED_OUT" => result |= Modifier::CROSSED_OUT,
                _ => {}
            }
        }
        result
    }

    fn style_from_config(style_config: &StyleConfig) -> Style {
        let mut style = Style::default();
        
        if let Some(fg) = &style_config.fg {
            style = style.fg(Self::parse_color(fg));
        }
        
        if let Some(bg) = &style_config.bg {
            style = style.bg(Self::parse_color(bg));
        }
        
        if !style_config.modifiers.is_empty() {
            style = style.add_modifier(Self::parse_modifiers(&style_config.modifiers));
        }
        
        style
    }

    fn border_style_from_config(style_config: &BorderStyleConfig) -> Style {
        let mut style = Style::default();
        
        if let Some(fg) = &style_config.fg {
            style = style.fg(Self::parse_color(fg));
        }
        
        if let Some(bg) = &style_config.bg {
            style = style.bg(Self::parse_color(bg));
        }
        
        if !style_config.modifiers.is_empty() {
            style = style.add_modifier(Self::parse_modifiers(&style_config.modifiers));
        }
        
        style
    }

    pub fn now_playing_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.now_playing.style)
    }

    pub fn now_playing_border_style(&self) -> Style {
        Self::border_style_from_config(&self.config.widgets.now_playing.block.border_style)
    }

    pub fn progress_gauge_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.progress_gauge.gauge_style)
    }

    pub fn progress_gauge_border_style(&self) -> Style {
        Self::border_style_from_config(&self.config.widgets.progress_gauge.block.border_style)
    }

    pub fn progress_text_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.progress_text.style)
    }

    pub fn progress_text_border_style(&self) -> Style {
        Self::border_style_from_config(&self.config.widgets.progress_text.block.border_style)
    }

    pub fn browser_border_style(&self, focused: bool) -> Style {
        if focused {
            Self::style_from_config(&self.config.widgets.browser.block.border_style.focused)
        } else {
            Self::style_from_config(&self.config.widgets.browser.block.border_style.unfocused)
        }
    }

    pub fn browser_highlight_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.browser.highlight_style)
    }

    pub fn browser_highlight_symbol(&self) -> &str {
        &self.config.widgets.browser.highlight_symbol
    }

    pub fn songs_border_style(&self, focused: bool) -> Style {
        if focused {
            Self::style_from_config(&self.config.widgets.songs.block.border_style.focused)
        } else {
            Self::style_from_config(&self.config.widgets.songs.block.border_style.unfocused)
        }
    }

    pub fn songs_highlight_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.songs.highlight_style)
    }

    pub fn songs_playing_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.songs.playing_style)
    }

    pub fn songs_highlight_symbol(&self) -> &str {
        &self.config.widgets.songs.highlight_symbol
    }

    pub fn playlist_border_style(&self, focused: bool) -> Style {
        if focused {
            Self::style_from_config(&self.config.widgets.playlist.block.border_style.focused)
        } else {
            Self::style_from_config(&self.config.widgets.playlist.block.border_style.unfocused)
        }
    }

    pub fn playlist_highlight_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.playlist.highlight_style)
    }

    pub fn playlist_playing_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.playlist.playing_style)
    }

    pub fn playlist_highlight_symbol(&self) -> &str {
        &self.config.widgets.playlist.highlight_symbol
    }

    pub fn controls_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.controls.style)
    }

    pub fn controls_border_style(&self) -> Style {
        Self::border_style_from_config(&self.config.widgets.controls.block.border_style)
    }

    pub fn controls_volume_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.controls.volume_style)
    }

    pub fn menu_style(&self) -> Style {
        Self::style_from_config(&self.config.widgets.menu.style)
    }

    pub fn menu_border_style(&self) -> Style {
        Self::border_style_from_config(&self.config.widgets.menu.block.border_style)
    }
}
