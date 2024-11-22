use ratatui::style::Color as RatatuiColor;
use crate::theme::color_dictionary::COLOR_MAP;

/// Converts a hex color string to RGB components.
/// Supports both 6-digit (#RRGGBB) and 3-digit (#RGB) formats.
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    // Remove '#' if present
    let hex = hex.trim_start_matches('#');
    
    // Validate hex string only contains valid hex characters
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    
    match hex.len() {
        6 => {
            // Parse 6-digit hex (#RRGGBB)
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        3 => {
            // Parse 3-digit hex (#RGB)
            // Convert to 6-digit by duplicating each digit
            let r = u8::from_str_radix(&format!("{}{}", &hex[0..1], &hex[0..1]), 16).ok()?;
            let g = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).ok()?;
            let b = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).ok()?;
            Some((r, g, b))
        }
        _ => None
    }
}

/// Parses a color string into a RatatuiColor.
/// Supports:
/// - Hex colors (#RRGGBB or #RGB)
/// - Named CSS colors
/// - Custom color aliases
/// 
/// Returns black (0,0,0) for unknown or invalid colors.
pub fn parse_color(color_str: &str) -> RatatuiColor {
    // Validate input
    if color_str.is_empty() {
        return RatatuiColor::Rgb(0, 0, 0);
    }

    // Check for hex color format first
    if color_str.starts_with('#') {
        if let Some((r, g, b)) = hex_to_rgb(color_str) {
            return RatatuiColor::Rgb(r, g, b);
        }
    }
    
    // Convert to lowercase for case-insensitive matching
    let color_key = color_str.to_lowercase();
    
    // Look up color in dictionary
    if let Some(&(r, g, b)) = COLOR_MAP.get(color_key.as_str()) {
        RatatuiColor::Rgb(r, g, b)
    } else {
        RatatuiColor::Rgb(0, 0, 0) // Default to black for unknown colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_css_colors() {
        // Test standard CSS colors
        assert_eq!(parse_color("chartreuse"), RatatuiColor::Rgb(127, 255, 0));
        assert_eq!(parse_color("blue"), RatatuiColor::Rgb(0, 0, 255));
        assert_eq!(parse_color("BLUE"), RatatuiColor::Rgb(0, 0, 255)); // Case insensitive
        assert_eq!(parse_color("white"), RatatuiColor::Rgb(255, 255, 255));
        assert_eq!(parse_color("tomato"), RatatuiColor::Rgb(255, 99, 71));
    }

    #[test]
    fn test_parse_hex_colors() {
        // Test 6-digit hex colors
        assert_eq!(parse_color("#FF0000"), RatatuiColor::Rgb(255, 0, 0));
        assert_eq!(parse_color("#00FF00"), RatatuiColor::Rgb(0, 255, 0));
        assert_eq!(parse_color("#0000FF"), RatatuiColor::Rgb(0, 0, 255));
        assert_eq!(parse_color("#D2691E"), RatatuiColor::Rgb(210, 105, 30)); // chocolate

        // Test 3-digit hex colors
        assert_eq!(parse_color("#F00"), RatatuiColor::Rgb(255, 0, 0));
        assert_eq!(parse_color("#0F0"), RatatuiColor::Rgb(0, 255, 0));
        assert_eq!(parse_color("#00F"), RatatuiColor::Rgb(0, 0, 255));
    }

    #[test]
    fn test_parse_aliases() {
        // Test our custom aliases with unique values
        assert_eq!(parse_color("lightred"), RatatuiColor::Rgb(255, 128, 128));
        assert_eq!(parse_color("lightmagenta"), RatatuiColor::Rgb(255, 128, 255));
    }

    #[test]
    fn test_invalid_inputs() {
        // Test invalid inputs
        assert_eq!(parse_color(""), RatatuiColor::Rgb(0, 0, 0));
        assert_eq!(parse_color("nonexistentcolor"), RatatuiColor::Rgb(0, 0, 0));
        assert_eq!(parse_color("#XYZ"), RatatuiColor::Rgb(0, 0, 0)); // Invalid hex
        assert_eq!(parse_color("#12"), RatatuiColor::Rgb(0, 0, 0));  // Invalid hex length
    }

    #[test]
    fn test_hex_validation() {
        // Test hex validation
        assert_eq!(parse_color("#GGG"), RatatuiColor::Rgb(0, 0, 0)); // Invalid hex chars
        assert_eq!(parse_color("#12345"), RatatuiColor::Rgb(0, 0, 0)); // Wrong length
    }
}
