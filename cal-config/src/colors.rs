//! Color scheme configuration

use serde::{Deserialize, Serialize};

/// Color scheme of the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    /// Background color
    pub background: String,
    /// Background color when hovering
    pub hover: String,
    /// Background color when selected (on click)
    pub selected: String,
    /// Text color
    pub text: String,
    /// Text color when selected
    pub text_selected: String,
    /// Dim text color (for description)
    pub text_dim: String,
}

/// Default implemtentation for Colors (Atom One Dark)
impl Default for Colors {
    fn default() -> Self {
        Self {
            background: "#282c34".into(),
            hover: "#3e4451".into(),
            selected: "#4b5263".into(),
            text: "#abb2bf".into(),
            text_selected: "#ffffff".into(),
            text_dim: "#5c6370".into(),
        }
    }
}
