//! CAL entries, and how to parse them

use serde::{Deserialize, Serialize};

use crate::freedesktop::freedesktop_entries;

/// An entry in the app launcher
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Entry {
    /// Entry name
    pub name: String,
    /// Entry command (ran if the entry is selected)
    pub command: String,
    /// Optional entry picture
    pub icon: Option<String>,
    /// Optional entry description
    pub description: Option<String>,
    /// Keywords for filtering
    pub keywords: Option<Vec<String>>,
}

/// Parse CAL entries from CSV
pub fn from_csv() -> Vec<Entry> {
    todo!("CAL entries from CSV")
}

/// Parse CAL entries from Freedesktop ones
pub fn from_freedesktop() -> Vec<Entry> {
    freedesktop_entries()
        .into_iter()
        .map(|(_, entry)| entry)
        .collect()
}
