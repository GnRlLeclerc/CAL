// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cal_config::{cli::process_cli_config, entries::from_freedesktop};
use launcher_lib::AppState;

fn main() {
    let mut config = process_cli_config();

    // If the app is launched as a daemon, or once with no entries,
    // it means that the desktop entries should be used (costly to load)
    if config.daemon || config.entries.is_empty() {
        config.entries = from_freedesktop(&config.icon_theme);
    }

    let state = match config.daemon {
        true => AppState::Daemon(config.entries),
        false => AppState::Once(config),
    };

    launcher_lib::run(state)
}
