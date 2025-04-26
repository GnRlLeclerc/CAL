//! Color scheme configuration

use serde::{Deserialize, Serialize};

/// Color scheme of the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    /// Background color
    #[serde(default = "default_background")]
    pub background: String,

    /// Background color when hovering
    #[serde(default = "default_hover")]
    pub hover: String,

    /// Background color when selected (on click)
    #[serde(default = "default_selected")]
    pub selected: String,

    /// Text color
    #[serde(default = "default_text")]
    pub text: String,

    /// Text color when selected
    #[serde(default = "default_text_selected")]
    pub text_selected: String,

    /// Dim text color (for description)
    #[serde(default = "default_text_dim")]
    pub text_dim: String,
}

// Default color theme (Atom One Dark)

fn default_background() -> String {
    "#282c34".into()
}

fn default_hover() -> String {
    "#3e4451".into()
}

fn default_selected() -> String {
    "#4b5263".into()
}

fn default_text() -> String {
    "#abb2bf".into()
}

fn default_text_selected() -> String {
    "#ffffff".into()
}

fn default_text_dim() -> String {
    "#5c6370".into()
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            background: default_background(),
            hover: default_hover(),
            selected: default_selected(),
            text: default_text(),
            text_selected: default_text_selected(),
            text_dim: default_text_dim(),
        }
    }
}

/// Parse colors from TOML
pub fn from_toml<'a>(config: &'a str) -> Result<Colors, toml::de::Error> {
    toml::from_str(config)
}

/// Parse colors from JSON
pub fn from_json(config: &str) -> serde_json::Result<Colors> {
    serde_json::from_str(config)
}
