use crate::theme::{Theme, ThemeMetadata, Colors, Controls, Styles, StyleConfig};

/// Creates a test theme with predefined values for testing purposes
pub fn create_test_theme() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Test Theme".to_string(),
            author: "Test Author".to_string(),
            version: "1.0".to_string(),
        },
        colors: Colors {
            primary: "#2E8B57".to_string(),
            secondary: "#98FB98".to_string(),
            background: "#1A1A1A".to_string(),
            foreground: "#F5F5F5".to_string(),
            active: "#98FB98".to_string(),
            inactive: "#696969".to_string(),
            playing: "#00FF7F".to_string(),
            error: "#FF6347".to_string(),
        },
        controls: Controls {
            record: "⏺".to_string(),
            play: "⏵".to_string(),
            rewind: "⏪".to_string(),
            fast_forward: "⏩".to_string(),
            stop: "⏹".to_string(),
            pause: "⏸".to_string(),
            next: "⏭".to_string(),
            previous: "⏮".to_string(),
        },
        styles: Styles {
            border_focused: StyleConfig {
                fg: Some("#98FB98".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            border_unfocused: StyleConfig {
                fg: Some("#2E8B57".to_string()),
                bg: None,
                modifiers: None,
            },
            text_normal: StyleConfig {
                fg: Some("#F5F5F5".to_string()),
                bg: None,
                modifiers: None,
            },
            text_bold: StyleConfig {
                fg: Some("#F5F5F5".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            text_dim: StyleConfig {
                fg: Some("#696969".to_string()),
                bg: None,
                modifiers: None,
            },
            button: StyleConfig {
                fg: Some("#2E8B57".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            list_item: StyleConfig {
                fg: Some("#F5F5F5".to_string()),
                bg: None,
                modifiers: None,
            },
            list_selected: StyleConfig {
                fg: Some("#98FB98".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            playing_item: StyleConfig {
                fg: Some("#00FF7F".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            progress_bar: StyleConfig {
                fg: Some("#98FB98".to_string()),
                bg: Some("#1A1A1A".to_string()),
                modifiers: None,
            },
            volume_indicator: StyleConfig {
                fg: Some("#00FF7F".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            tab_active: StyleConfig {
                fg: Some("#1A1A1A".to_string()),
                bg: Some("#2E8B57".to_string()),
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            tab_inactive: StyleConfig {
                fg: Some("#2E8B57".to_string()),
                bg: None,
                modifiers: None,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::create_block;

    #[test]
    fn test_create_block() {
        let title = "Test Block";
        let focused = true;
        let theme = create_test_theme();
        
        let _block = create_block(title, focused, &theme);
        // Block creation succeeded if we got here
    }

    #[test]
    fn test_theme_creation() {
        let theme = create_test_theme();
        assert_eq!(theme.metadata.name, "Test Theme");
        assert_eq!(theme.colors.primary, "#2E8B57");
        assert_eq!(theme.controls.play, "⏵");
        assert!(theme.styles.border_focused.modifiers.is_some());
    }
}
