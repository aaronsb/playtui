use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    pub colors: ThemeColors,
    #[serde(default)]
    pub controls: ThemeControls,
    pub styles: HashMap<String, ThemeStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub foreground: String,
    pub active: String,
    pub inactive: String,
    pub playing: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeControls {
    #[serde(default = "default_record")]
    pub record: String,
    #[serde(default = "default_play")]
    pub play: String,
    #[serde(default = "default_rewind")]
    pub rewind: String,
    #[serde(default = "default_fast_forward")]
    pub fast_forward: String,
    #[serde(default = "default_stop")]
    pub stop: String,
    #[serde(default = "default_pause")]
    pub pause: String,
    #[serde(default = "default_next")]
    pub next: String,
    #[serde(default = "default_previous")]
    pub previous: String,
}

fn default_record() -> String { "⏺".to_string() }
fn default_play() -> String { "▶".to_string() }
fn default_rewind() -> String { "⏪".to_string() }
fn default_fast_forward() -> String { "⏩".to_string() }
fn default_stop() -> String { "⏹".to_string() }
fn default_pause() -> String { "⏸".to_string() }
fn default_next() -> String { "⏭".to_string() }
fn default_previous() -> String { "⏮".to_string() }

impl Default for ThemeControls {
    fn default() -> Self {
        Self {
            record: default_record(),
            play: default_play(),
            rewind: default_rewind(),
            fast_forward: default_fast_forward(),
            stop: default_stop(),
            pause: default_pause(),
            next: default_next(),
            previous: default_previous(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

impl Theme {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&content)?;
        Ok(theme)
    }

    pub fn get_style(&self, style_name: &str) -> Style {
        if let Some(theme_style) = self.styles.get(style_name) {
            let mut style = Style::default();

            if let Some(fg) = &theme_style.fg {
                style = style.fg(parse_color(fg));
            }

            if let Some(bg) = &theme_style.bg {
                style = style.bg(parse_color(bg));
            }

            for modifier in &theme_style.modifiers {
                style = style.add_modifier(parse_modifier(modifier));
            }

            style
        } else {
            Style::default()
        }
    }

    pub fn get_color(&self, color_name: &str) -> Color {
        match color_name {
            "primary" => parse_color(&self.colors.primary),
            "secondary" => parse_color(&self.colors.secondary),
            "background" => parse_color(&self.colors.background),
            "foreground" => parse_color(&self.colors.foreground),
            "active" => parse_color(&self.colors.active),
            "inactive" => parse_color(&self.colors.inactive),
            "playing" => parse_color(&self.colors.playing),
            "error" => parse_color(&self.colors.error),
            _ => Color::White,
        }
    }
}

fn parse_color(color_str: &str) -> Color {
    if color_str.starts_with('#') {
        // Parse hex color
        if let Ok(value) = u32::from_str_radix(&color_str[1..], 16) {
            let r = ((value >> 16) & 0xFF) as u8;
            let g = ((value >> 8) & 0xFF) as u8;
            let b = (value & 0xFF) as u8;
            return Color::Rgb(r, g, b);
        }
    }
    
    // Fallback to named colors
    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        _ => Color::White,
    }
}

fn parse_modifier(modifier_str: &str) -> Modifier {
    match modifier_str.to_uppercase().as_str() {
        "BOLD" => Modifier::BOLD,
        "DIM" => Modifier::DIM,
        "ITALIC" => Modifier::ITALIC,
        "UNDERLINED" => Modifier::UNDERLINED,
        "SLOW_BLINK" => Modifier::SLOW_BLINK,
        "RAPID_BLINK" => Modifier::RAPID_BLINK,
        "REVERSED" => Modifier::REVERSED,
        "HIDDEN" => Modifier::HIDDEN,
        "CROSSED_OUT" => Modifier::CROSSED_OUT,
        _ => Modifier::empty(),
    }
}

pub fn load_theme(theme_name: &str) -> Result<Theme, Box<dyn std::error::Error>> {
    let theme_path = format!("themes/{}.json", theme_name);
    Theme::load_from_file(Path::new(&theme_path))
}

pub fn list_available_themes() -> Vec<String> {
    let themes_dir = Path::new("themes");
    let mut themes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(themes_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    let theme_name = filename.trim_end_matches(".json");
                    themes.push(theme_name.to_string());
                }
            }
        }
    }
    
    themes.sort();
    themes
}