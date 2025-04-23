//! CAL config and how to parse it

use serde::Deserialize;

use crate::Entry;

/// The CAL config
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The config entries
    pub entries: Vec<Entry>,
}

impl Default for Config {
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

impl Config {
    /// Merge another config into this one, taking precedence over existing fields
    pub fn merge(&mut self, other: Config) {
        // Override entries if not empty
        if !other.entries.is_empty() {
            self.entries = other.entries;
        }
    }
}

/// Parse a CAL config + entries from TOML
pub fn from_toml<'a>(config: &'a str) -> Result<Config, toml::de::Error> {
    toml::from_str(config)
}

/// Parse a CAL config + entries from JSON
pub fn from_json(config: &str) -> serde_json::Result<Config> {
    serde_json::from_str(config)
}
