use anyhow::Result;
use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    pub colors: Colors,
    pub styles: Styles,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Colors {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub foreground: String,
    pub active: String,
    pub inactive: String,
    pub playing: String,
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StyleConfig {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub modifiers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Styles {
    pub border_focused: StyleConfig,
    pub border_unfocused: StyleConfig,
    pub text_normal: StyleConfig,
    pub text_bold: StyleConfig,
    pub text_dim: StyleConfig,
    pub button: StyleConfig,
    pub list_item: StyleConfig,
    pub list_selected: StyleConfig,
    pub playing_item: StyleConfig,
    pub progress_bar: StyleConfig,
    pub volume_indicator: StyleConfig,
    pub tab_active: StyleConfig,
    pub tab_inactive: StyleConfig,
}

impl Theme {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&content)?;
        Ok(theme)
    }

    pub fn load_default() -> Result<Self> {
        Self::load("themes/default.json")
    }

    fn parse_color(color_str: &str) -> Result<Color> {
        if !color_str.starts_with('#') {
            return Ok(match color_str.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "gray" => Color::Gray,
                "darkgray" => Color::DarkGray,
                "lightred" => Color::LightRed,
                "lightgreen" => Color::LightGreen,
                "lightyellow" => Color::LightYellow,
                "lightblue" => Color::LightBlue,
                "lightmagenta" => Color::LightMagenta,
                "lightcyan" => Color::LightCyan,
                "white" => Color::White,
                _ => return Err(anyhow::anyhow!("Invalid color name: {}", color_str)),
            });
        }

        let hex = &color_str[1..];
        if hex.len() != 6 {
            return Err(anyhow::anyhow!("Invalid hex color: {}", color_str));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(Color::Rgb(r, g, b))
    }

    fn parse_modifiers(modifiers: &[String]) -> Modifier {
        let mut result = Modifier::empty();
        for modifier in modifiers {
            match modifier.to_uppercase().as_str() {
                "BOLD" => result.insert(Modifier::BOLD),
                "DIM" => result.insert(Modifier::DIM),
                "ITALIC" => result.insert(Modifier::ITALIC),
                "UNDERLINED" => result.insert(Modifier::UNDERLINED),
                "SLOW_BLINK" => result.insert(Modifier::SLOW_BLINK),
                "RAPID_BLINK" => result.insert(Modifier::RAPID_BLINK),
                "REVERSED" => result.insert(Modifier::REVERSED),
                "HIDDEN" => result.insert(Modifier::HIDDEN),
                "CROSSED_OUT" => result.insert(Modifier::CROSSED_OUT),
                _ => {}
            }
        }
        result
    }

    fn style_from_config(&self, config: &StyleConfig) -> Style {
        let mut style = Style::default();

        if let Some(ref fg) = config.fg {
            if let Ok(color) = Self::parse_color(fg) {
                style = style.fg(color);
            }
        }

        if let Some(ref bg) = config.bg {
            if let Ok(color) = Self::parse_color(bg) {
                style = style.bg(color);
            }
        }

        if let Some(ref modifiers) = config.modifiers {
            style = style.add_modifier(Self::parse_modifiers(modifiers));
        }

        style
    }

    pub fn get_style(&self, style_name: &str) -> Style {
        match style_name {
            "border_focused" => self.style_from_config(&self.styles.border_focused),
            "border_unfocused" => self.style_from_config(&self.styles.border_unfocused),
            "text_normal" => self.style_from_config(&self.styles.text_normal),
            "text_bold" => self.style_from_config(&self.styles.text_bold),
            "text_dim" => self.style_from_config(&self.styles.text_dim),
            "button" => self.style_from_config(&self.styles.button),
            "list_item" => self.style_from_config(&self.styles.list_item),
            "list_selected" => self.style_from_config(&self.styles.list_selected),
            "playing_item" => self.style_from_config(&self.styles.playing_item),
            "progress_bar" => self.style_from_config(&self.styles.progress_bar),
            "volume_indicator" => self.style_from_config(&self.styles.volume_indicator),
            "tab_active" => self.style_from_config(&self.styles.tab_active),
            "tab_inactive" => self.style_from_config(&self.styles.tab_inactive),
            _ => Style::default(),
        }
    }

    pub fn get_color(&self, color_name: &str) -> Option<Color> {
        let color_str = match color_name {
            "primary" => &self.colors.primary,
            "secondary" => &self.colors.secondary,
            "background" => &self.colors.background,
            "foreground" => &self.colors.foreground,
            "active" => &self.colors.active,
            "inactive" => &self.colors.inactive,
            "playing" => &self.colors.playing,
            "error" => &self.colors.error,
            _ => return None,
        };

        Self::parse_color(color_str).ok()
    }
}
