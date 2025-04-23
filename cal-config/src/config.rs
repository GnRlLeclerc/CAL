//! CAL config and how to parse it

use serde::Deserialize;

use crate::Entry;

/// The CAL config
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The config entries
    pub entries: Vec<Entry>,
}

/// Parse a CAL config + entries from TOML
pub fn from_toml<'a>(config: &'a str) -> Result<Config, toml::de::Error> {
    toml::from_str(config)
}

/// Parse a CAL config + entries from JSON
pub fn from_json(config: &str) -> serde_json::Result<Config> {
    serde_json::from_str(config)
}
