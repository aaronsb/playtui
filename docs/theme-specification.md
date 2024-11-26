# Theme Specification

## Overview

This document specifies the theming system for PlayTUI. The system is designed to be simple, maintainable, and user-customizable while providing consistent styling across the application.

## Features

- Serialization/deserialization support via serde
- Color support via ratatui's palette feature
- Support for both hex colors (#RRGGBB) and HTML color names
- Global theme application across all elements
- Simplified element targeting

## Theme Structure

```rust
// Theme structure (serializable)
pub struct Theme {
    pub metadata: ThemeMetadata,
    pub colors: Colors,
    pub styles: Styles,
}

pub struct ThemeMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
}

pub struct Colors {
    // Primary colors
    pub primary: Color,      // Main accent color
    pub secondary: Color,    // Secondary accent color
    pub background: Color,   // Default background
    pub foreground: Color,   // Default text color

    // State colors
    pub active: Color,       // Currently selected/active items
    pub inactive: Color,     // Inactive/disabled elements
    pub playing: Color,      // Currently playing item
    pub error: Color,        // Error states
}

pub struct Styles {
    // Border styles
    pub border_focused: Style,
    pub border_unfocused: Style,
    
    // Text styles
    pub text_normal: Style,
    pub text_bold: Style,
    pub text_dim: Style,
    
    // Interactive element styles
    pub button: Style,           // Default button style
    pub button_active: Style,    // Style for active buttons (e.g. pause when playing)
    pub button_shadow: Style,    // Shadow effect for buttons
    pub record_button_active: Style, // Special style for active record button
    pub list_item: Style,
    pub list_selected: Style,
    
    // Player-specific styles
    pub playing_item: Style,     // Currently playing track
    pub progress_bar: Style,     // Playback progress
    pub volume_indicator: Style, // Volume level indicator
    pub tab_active: Style,      // Active tab
    pub tab_inactive: Style,    // Inactive tab
}
```

## Color Definition

Colors can be specified in two formats:
1. Hex format: `"#RRGGBB"` (e.g., `"#D2691E"`)
2. HTML color names: (e.g., `"Chocolate"`)

### Example Theme JSON

```json
{
  "metadata": {
    "name": "Forest",
    "author": "PlayTUI Team",
    "version": "1.0"
  },
  "colors": {
    "primary": "#2E8B57",
    "secondary": "SeaGreen",
    "background": "#1A1A1A",
    "foreground": "#F5F5F5",
    "active": "#98FB98",
    "inactive": "#696969",
    "playing": "#00FF7F",
    "error": "#FF6347"
  },
  "styles": {
    "border_focused": {
      "fg": "#98FB98",
      "modifiers": ["BOLD"]
    },
    "border_unfocused": {
      "fg": "#2E8B57"
    },
    "text_normal": {
      "fg": "#F5F5F5"
    },
    "text_bold": {
      "fg": "#F5F5F5",
      "modifiers": ["BOLD"]
    },
    "text_dim": {
      "fg": "#696969"
    },
    "button": {
      "fg": "#2E8B57",
      "modifiers": ["BOLD"]
    },
    "button_active": {
      "fg": "#98FB98",
      "modifiers": ["BOLD"]
    },
    "button_shadow": {
      "fg": "#1A1A1A",
      "bg": "#2E8B57",
      "modifiers": []
    },
    "record_button_active": {
      "fg": "#FF6347",
      "modifiers": ["BOLD"]
    },
    "list_item": {
      "fg": "#F5F5F5"
    },
    "list_selected": {
      "fg": "#98FB98",
      "modifiers": ["BOLD"]
    },
    "playing_item": {
      "fg": "#00FF7F",
      "modifiers": ["BOLD"]
    },
    "progress_bar": {
      "fg": "#98FB98",
      "bg": "#1A1A1A"
    },
    "volume_indicator": {
      "fg": "#00FF7F",
      "modifiers": ["BOLD"]
    },
    "tab_active": {
      "fg": "#1A1A1A",
      "bg": "#2E8B57",
      "modifiers": ["BOLD"]
    },
    "tab_inactive": {
      "fg": "#2E8B57"
    }
  }
}
```

## Implementation Notes

1. **Color Parsing**
   - Implement a color parser that supports both hex and HTML color names
   - Maintain a static dictionary of HTML color names to RGB values
   - Use the `palette` feature for color manipulation

2. **Serialization**
   - Use serde for theme serialization/deserialization
   - Support both JSON and TOML formats for theme files

3. **Application**
   - Themes are applied globally to all elements
   - Components should reference theme styles rather than defining their own
   - Use macros to reduce boilerplate in theme application

4. **Default Theme**
   - Provide a default theme that serves as a fallback
   - Default theme should use safe, readable colors

## Best Practices

1. **Color Usage**
   - Use semantic color names in the theme definition
   - Maintain good contrast ratios for accessibility
   - Consider color blindness when choosing default colors

2. **Style Consistency**
   - Reuse existing styles when possible
   - Maintain consistent styling for similar elements
   - Use modifiers sparingly and consistently

3. **Theme Creation**
   - Document all color and style choices
   - Test themes with all application states
   - Validate themes against accessibility guidelines
