mod color;
mod style;
mod types;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use ratatui::style::{Color, Style};

pub use types::{ThemeMetadata, Colors, Controls, Styles, StyleConfig};
pub use color::parse_color;
pub use style::{parse_modifiers, style_from_config};

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    pub colors: Colors,
    pub styles: Styles,
    pub controls: Controls,
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

    pub fn get_style(&self, style_name: &str) -> Style {
        style::get_style(self, style_name)
    }

    pub fn get_color(&self, color_name: &str) -> Option<Color> {
        color::get_color(self, color_name)
    }
}
