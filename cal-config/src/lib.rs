//! Configurable App Launcher entries

pub mod cli;
pub mod config;
pub mod entries;
mod freedesktop;
mod icons;

pub use config::Config;
pub use entries::Entry;
