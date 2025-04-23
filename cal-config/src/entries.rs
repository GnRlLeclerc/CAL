//! CAL entries, and how to parse them

use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use serde::Deserialize;

/// An entry in the app launcher
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Entry name
    pub name: String,
    /// Entry command (ran if the entry is selected)
    pub command: String,
    /// Optional entry picture
    pub picture: Option<String>,
    /// Optional entry description
    pub description: Option<String>,
}

/// Parse CAL entries from CSV
pub fn from_csv(_config: &str) -> Vec<Entry> {
    todo!("CAL entries from CSV")
}

/// Parse CAL entries from freedesktop files
///
// * `use_locale` - if true, use the locale translation of entries names & descriptions.
pub fn from_freedesktop(use_locale: bool) -> Vec<Entry> {
    let locales = match use_locale {
        true => get_languages_from_env(),
        false => vec![],
    };

    Iter::new(default_paths())
        .entries(Some(&locales))
        .filter_map(|entry| {
            Some(Entry {
                name: entry.name(&locales)?.to_string(),
                description: entry.comment(&locales).map(|s| s.to_string()),
                command: entry.exec()?.to_string(),
                picture: entry.icon().map(|s| s.to_string()),
            })
        })
        .collect()
}
