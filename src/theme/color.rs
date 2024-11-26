use anyhow::Result;
use ratatui::style::Color;
use super::Theme;

pub fn parse_color(color_str: &str) -> Result<Color> {
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

pub fn get_color(theme: &Theme, color_name: &str) -> Option<Color> {
    let color_str = match color_name {
        "primary" => &theme.colors.primary,
        "secondary" => &theme.colors.secondary,
        "background" => &theme.colors.background,
        "foreground" => &theme.colors.foreground,
        "active" => &theme.colors.active,
        "inactive" => &theme.colors.inactive,
        "playing" => &theme.colors.playing,
        "error" => &theme.colors.error,
        _ => return None,
    };

    parse_color(color_str).ok()
}
