use ratatui::style::Color as RatatuiColor;

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    // Remove '#' if present
    let hex = hex.trim_start_matches('#');
    
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

pub fn parse_color(color_str: &str) -> RatatuiColor {
    // Check for hex color format first
    if color_str.starts_with('#') {
        if let Some((r, g, b)) = hex_to_rgb(color_str) {
            return RatatuiColor::Rgb(r, g, b);
        }
    }
    
    // Convert to lowercase for case-insensitive matching
    let color_key = color_str.to_lowercase();
    
    // Define common CSS colors
    match color_key.as_str() {
        "black" => RatatuiColor::Rgb(0, 0, 0),
        "silver" => RatatuiColor::Rgb(192, 192, 192),
        "gray" | "grey" => RatatuiColor::Rgb(128, 128, 128),
        "white" => RatatuiColor::Rgb(255, 255, 255),
        "maroon" => RatatuiColor::Rgb(128, 0, 0),
        "red" => RatatuiColor::Rgb(255, 0, 0),
        "purple" => RatatuiColor::Rgb(128, 0, 128),
        "fuchsia" | "magenta" => RatatuiColor::Rgb(255, 0, 255),
        "green" => RatatuiColor::Rgb(0, 128, 0),
        "lime" => RatatuiColor::Rgb(0, 255, 0),
        "olive" => RatatuiColor::Rgb(128, 128, 0),
        "yellow" => RatatuiColor::Rgb(255, 255, 0),
        "navy" => RatatuiColor::Rgb(0, 0, 128),
        "blue" => RatatuiColor::Rgb(0, 0, 255),
        "teal" => RatatuiColor::Rgb(0, 128, 128),
        "aqua" | "cyan" => RatatuiColor::Rgb(0, 255, 255),
        "orange" => RatatuiColor::Rgb(255, 165, 0),
        "aliceblue" => RatatuiColor::Rgb(240, 248, 255),
        "antiquewhite" => RatatuiColor::Rgb(250, 235, 215),
        "aquamarine" => RatatuiColor::Rgb(127, 255, 212),
        "azure" => RatatuiColor::Rgb(240, 255, 255),
        "beige" => RatatuiColor::Rgb(245, 245, 220),
        "bisque" => RatatuiColor::Rgb(255, 228, 196),
        "blanchedalmond" => RatatuiColor::Rgb(255, 235, 205),
        "blueviolet" => RatatuiColor::Rgb(138, 43, 226),
        "brown" => RatatuiColor::Rgb(165, 42, 42),
        "burlywood" => RatatuiColor::Rgb(222, 184, 135),
        "cadetblue" => RatatuiColor::Rgb(95, 158, 160),
        "chartreuse" => RatatuiColor::Rgb(127, 255, 0),
        "chocolate" => RatatuiColor::Rgb(210, 105, 30),
        "coral" => RatatuiColor::Rgb(255, 127, 80),
        "cornflowerblue" => RatatuiColor::Rgb(100, 149, 237),
        "cornsilk" => RatatuiColor::Rgb(255, 248, 220),
        "crimson" => RatatuiColor::Rgb(220, 20, 60),
        "darkblue" => RatatuiColor::Rgb(0, 0, 139),
        "darkcyan" => RatatuiColor::Rgb(0, 139, 139),
        "darkgoldenrod" => RatatuiColor::Rgb(184, 134, 11),
        "darkgray" | "darkgrey" => RatatuiColor::Rgb(169, 169, 169),
        "darkgreen" => RatatuiColor::Rgb(0, 100, 0),
        "darkkhaki" => RatatuiColor::Rgb(189, 183, 107),
        "darkmagenta" => RatatuiColor::Rgb(139, 0, 139),
        "darkolivegreen" => RatatuiColor::Rgb(85, 107, 47),
        "darkorange" => RatatuiColor::Rgb(255, 140, 0),
        "darkorchid" => RatatuiColor::Rgb(153, 50, 204),
        "darkred" => RatatuiColor::Rgb(139, 0, 0),
        "darksalmon" => RatatuiColor::Rgb(233, 150, 122),
        "darkseagreen" => RatatuiColor::Rgb(143, 188, 143),
        "darkslateblue" => RatatuiColor::Rgb(72, 61, 139),
        "darkslategray" | "darkslategrey" => RatatuiColor::Rgb(47, 79, 79),
        "darkturquoise" => RatatuiColor::Rgb(0, 206, 209),
        "darkviolet" => RatatuiColor::Rgb(148, 0, 211),
        "deeppink" => RatatuiColor::Rgb(255, 20, 147),
        "deepskyblue" => RatatuiColor::Rgb(0, 191, 255),
        "dimgray" | "dimgrey" => RatatuiColor::Rgb(105, 105, 105),
        "dodgerblue" => RatatuiColor::Rgb(30, 144, 255),
        "firebrick" => RatatuiColor::Rgb(178, 34, 34),
        "floralwhite" => RatatuiColor::Rgb(255, 250, 240),
        "forestgreen" => RatatuiColor::Rgb(34, 139, 34),
        "gainsboro" => RatatuiColor::Rgb(220, 220, 220),
        "ghostwhite" => RatatuiColor::Rgb(248, 248, 255),
        "gold" => RatatuiColor::Rgb(255, 215, 0),
        "goldenrod" => RatatuiColor::Rgb(218, 165, 32),
        "greenyellow" => RatatuiColor::Rgb(173, 255, 47),
        "honeydew" => RatatuiColor::Rgb(240, 255, 240),
        "hotpink" => RatatuiColor::Rgb(255, 105, 180),
        "indianred" => RatatuiColor::Rgb(205, 92, 92),
        "indigo" => RatatuiColor::Rgb(75, 0, 130),
        "ivory" => RatatuiColor::Rgb(255, 255, 240),
        "khaki" => RatatuiColor::Rgb(240, 230, 140),
        "lavender" => RatatuiColor::Rgb(230, 230, 250),
        "lavenderblush" => RatatuiColor::Rgb(255, 240, 245),
        "lawngreen" => RatatuiColor::Rgb(124, 252, 0),
        "lemonchiffon" => RatatuiColor::Rgb(255, 250, 205),
        "lightblue" => RatatuiColor::Rgb(173, 216, 230),
        "lightcoral" => RatatuiColor::Rgb(240, 128, 128),
        "lightcyan" => RatatuiColor::Rgb(224, 255, 255),
        "lightgoldenrodyellow" => RatatuiColor::Rgb(250, 250, 210),
        "lightgray" | "lightgrey" => RatatuiColor::Rgb(211, 211, 211),
        "lightgreen" => RatatuiColor::Rgb(144, 238, 144),
        "lightpink" => RatatuiColor::Rgb(255, 182, 193),
        "lightsalmon" => RatatuiColor::Rgb(255, 160, 122),
        "lightseagreen" => RatatuiColor::Rgb(32, 178, 170),
        "lightskyblue" => RatatuiColor::Rgb(135, 206, 250),
        "lightslategray" | "lightslategrey" => RatatuiColor::Rgb(119, 136, 153),
        "lightsteelblue" => RatatuiColor::Rgb(176, 196, 222),
        "lightyellow" => RatatuiColor::Rgb(255, 255, 224),
        "limegreen" => RatatuiColor::Rgb(50, 205, 50),
        "linen" => RatatuiColor::Rgb(250, 240, 230),
        "mediumaquamarine" => RatatuiColor::Rgb(102, 205, 170),
        "mediumblue" => RatatuiColor::Rgb(0, 0, 205),
        "mediumorchid" => RatatuiColor::Rgb(186, 85, 211),
        "mediumpurple" => RatatuiColor::Rgb(147, 112, 219),
        "mediumseagreen" => RatatuiColor::Rgb(60, 179, 113),
        "mediumslateblue" => RatatuiColor::Rgb(123, 104, 238),
        "mediumspringgreen" => RatatuiColor::Rgb(0, 250, 154),
        "mediumturquoise" => RatatuiColor::Rgb(72, 209, 204),
        "mediumvioletred" => RatatuiColor::Rgb(199, 21, 133),
        "midnightblue" => RatatuiColor::Rgb(25, 25, 112),
        "mintcream" => RatatuiColor::Rgb(245, 255, 250),
        "mistyrose" => RatatuiColor::Rgb(255, 228, 225),
        "moccasin" => RatatuiColor::Rgb(255, 228, 181),
        "navajowhite" => RatatuiColor::Rgb(255, 222, 173),
        "oldlace" => RatatuiColor::Rgb(253, 245, 230),
        "olivedrab" => RatatuiColor::Rgb(107, 142, 35),
        "orangered" => RatatuiColor::Rgb(255, 69, 0),
        "orchid" => RatatuiColor::Rgb(218, 112, 214),
        "palegoldenrod" => RatatuiColor::Rgb(238, 232, 170),
        "palegreen" => RatatuiColor::Rgb(152, 251, 152),
        "paleturquoise" => RatatuiColor::Rgb(175, 238, 238),
        "palevioletred" => RatatuiColor::Rgb(219, 112, 147),
        "papayawhip" => RatatuiColor::Rgb(255, 239, 213),
        "peachpuff" => RatatuiColor::Rgb(255, 218, 185),
        "peru" => RatatuiColor::Rgb(205, 133, 63),
        "pink" => RatatuiColor::Rgb(255, 192, 203),
        "plum" => RatatuiColor::Rgb(221, 160, 221),
        "powderblue" => RatatuiColor::Rgb(176, 224, 230),
        "rosybrown" => RatatuiColor::Rgb(188, 143, 143),
        "royalblue" => RatatuiColor::Rgb(65, 105, 225),
        "saddlebrown" => RatatuiColor::Rgb(139, 69, 19),
        "salmon" => RatatuiColor::Rgb(250, 128, 114),
        "sandybrown" => RatatuiColor::Rgb(244, 164, 96),
        "seagreen" => RatatuiColor::Rgb(46, 139, 87),
        "seashell" => RatatuiColor::Rgb(255, 245, 238),
        "sienna" => RatatuiColor::Rgb(160, 82, 45),
        "skyblue" => RatatuiColor::Rgb(135, 206, 235),
        "slateblue" => RatatuiColor::Rgb(106, 90, 205),
        "slategray" | "slategrey" => RatatuiColor::Rgb(112, 128, 144),
        "snow" => RatatuiColor::Rgb(255, 250, 250),
        "springgreen" => RatatuiColor::Rgb(0, 255, 127),
        "steelblue" => RatatuiColor::Rgb(70, 130, 180),
        "tan" => RatatuiColor::Rgb(210, 180, 140),
        "thistle" => RatatuiColor::Rgb(216, 191, 216),
        "tomato" => RatatuiColor::Rgb(255, 99, 71),
        "turquoise" => RatatuiColor::Rgb(64, 224, 208),
        "violet" => RatatuiColor::Rgb(238, 130, 238),
        "wheat" => RatatuiColor::Rgb(245, 222, 179),
        "whitesmoke" => RatatuiColor::Rgb(245, 245, 245),
        "yellowgreen" => RatatuiColor::Rgb(154, 205, 50),
        // Our custom aliases
        "lightred" => RatatuiColor::Rgb(255, 182, 193),    // Same as lightpink
        "lightmagenta" => RatatuiColor::Rgb(255, 182, 193),// Same as lightpink
        _ => RatatuiColor::Rgb(0, 0, 0), // Default to black for unknown colors
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
        // Test our custom aliases
        assert_eq!(parse_color("lightred"), RatatuiColor::Rgb(255, 182, 193));
        assert_eq!(parse_color("darkgray"), RatatuiColor::Rgb(169, 169, 169));
    }

    #[test]
    fn test_unknown_color() {
        // Test fallback for unknown colors
        assert_eq!(parse_color("nonexistentcolor"), RatatuiColor::Rgb(0, 0, 0));
        assert_eq!(parse_color("#XYZ"), RatatuiColor::Rgb(0, 0, 0)); // Invalid hex
        assert_eq!(parse_color("#12"), RatatuiColor::Rgb(0, 0, 0));  // Invalid hex length
    }
}
