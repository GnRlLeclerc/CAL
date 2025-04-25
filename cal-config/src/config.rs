//! CAL config and how to parse it

use serde::{Deserialize, Serialize};

use crate::{Entry, colors::Colors};

/// The CAL config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Daemon mode (does not display anything, runs a daemon)
    #[serde(default)]
    pub daemon: bool,

    /// Icon theme name
    pub icon_theme: Option<String>,

    /// The config entries
    #[serde(default)]
    pub entries: Vec<Entry>,

    /// Colors (can be optionally provided through the config)
    #[serde(default)]
    pub colors: Colors,
}

/// Parse a CAL config + entries from TOML
pub fn from_toml<'a>(config: &'a str) -> Result<Config, toml::de::Error> {
    toml::from_str(config)
}

/// Parse a CAL config + entries from JSON
pub fn from_json(config: &str) -> serde_json::Result<Config> {
    serde_json::from_str(config)
}
