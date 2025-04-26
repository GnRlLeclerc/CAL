// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::unix::net::UnixStream;

use cal_config::{cli::process_cli_config, entries::from_freedesktop};
use cal_daemon::{send_message, SOCKET};
use launcher_lib::AppState;
use tokio::net::UnixListener;

/// Close the socket when interrupted (daemon only)
async fn wait_for_termination() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");

    let _ = std::fs::remove_file(SOCKET);

    std::process::exit(0);
}

fn main() {
    let mut config = process_cli_config();
    let daemon = config.daemon.clone();

    // Running as client
    if !config.daemon {
        if let Ok(mut stream) = UnixStream::connect(SOCKET) {
            send_message(&config, &mut stream).unwrap();
            return;
        }
        // Else, run the app in "once" mode, and load the desktop entries
    }

    // If the app is launched as a daemon, or once with no entries,
    // it means that the desktop entries should be used (costly to load)
    if config.daemon || config.entries.is_empty() {
        config.entries = from_freedesktop(&config.icon_theme);
    }

    let state = match config.daemon {
        true => {
            let listener = tauri::async_runtime::block_on(async {
                UnixListener::bind(SOCKET)
                    .expect("Socket already present, cannot start a second daemon")
            });

            AppState::Daemon(listener, config.entries)
        }
        false => AppState::Once(config),
    };

    if daemon {
        // Wait for termination signal
        tauri::async_runtime::spawn(wait_for_termination());
    }

    launcher_lib::run(state, daemon)
}
