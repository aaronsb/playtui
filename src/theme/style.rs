use ratatui::style::{Modifier, Style};
use super::{Theme, StyleConfig};
use super::color::parse_color;

pub fn parse_modifiers(modifiers: &[String]) -> Modifier {
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

pub fn style_from_config(config: &StyleConfig) -> Style {
    let mut style = Style::default();

    if let Some(ref fg) = config.fg {
        if let Ok(color) = parse_color(fg) {
            style = style.fg(color);
        }
    }

    if let Some(ref bg) = config.bg {
        if let Ok(color) = parse_color(bg) {
            style = style.bg(color);
        }
    }

    if let Some(ref modifiers) = config.modifiers {
        style = style.add_modifier(parse_modifiers(modifiers));
    }

    style
}

pub fn get_style(theme: &Theme, style_name: &str) -> Style {
    match style_name {
        "border_focused" => style_from_config(&theme.styles.border_focused),
        "border_unfocused" => style_from_config(&theme.styles.border_unfocused),
        "text_normal" => style_from_config(&theme.styles.text_normal),
        "text_bold" => style_from_config(&theme.styles.text_bold),
        "text_dim" => style_from_config(&theme.styles.text_dim),
        "button" => style_from_config(&theme.styles.button),
        "list_item" => style_from_config(&theme.styles.list_item),
        "list_selected" => style_from_config(&theme.styles.list_selected),
        "playing_item" => style_from_config(&theme.styles.playing_item),
        "progress_bar" => style_from_config(&theme.styles.progress_bar),
        "volume_indicator" => style_from_config(&theme.styles.volume_indicator),
        "tab_active" => style_from_config(&theme.styles.tab_active),
        "tab_inactive" => style_from_config(&theme.styles.tab_inactive),
        _ => Style::default(),
    }
}
