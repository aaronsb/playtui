use serde::{Deserialize, Serialize};

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
pub struct Controls {
    pub record: String,
    pub play: String,
    pub rewind: String,
    pub fast_forward: String,
    pub stop: String,
    pub pause: String,
    pub next: String,
    pub previous: String,
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
