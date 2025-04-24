//! Configurable App Launcher entries

pub mod cli;
pub mod config;
pub mod entries;
pub mod freedesktop;

pub use config::Config;
pub use entries::Entry;
