use std::{env, process::Stdio};

use cal_config::{Config, Entry};
use tauri::{ipc::Channel, State};

pub enum AppState {
    /// One run only: contains the full config to send to the frontend
    Once(Config),
    /// Daemon mode: contains the cached desktop entries for default usage of the launcher
    Daemon(Vec<Entry>),
}

/// Load the App Launcher entries via a channel
/// The frontend should connect to the rust backend only once, using this endpoint.
#[tauri::command]
fn subscribe_config(state: State<'_, AppState>, channel: Channel<&Config>) {
    match state.inner() {
        AppState::Once(config) => channel.send(config).unwrap(),
        AppState::Daemon(entries) => {
            // TODO: keep the channel open, and send stuff when a new config is received.
            // Also send the entries when the ones in the received config are empty
            todo!("Daemon mode not implemented yet");
        }
    }
}

/// Run a command
#[tauri::command]
fn run_command(command: String, terminal: bool) {
    let mut command = command
        .split_whitespace()
        .filter(|&arg| match arg {
            "%U" | "%u" | "%F" | "%f" | "@@" | "@@u" => false,
            _ => true,
        })
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>();

    if terminal {
        for arg in get_default_terminal().into_iter().rev() {
            command.insert(0, arg);
        }
    }

    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let _ = std::process::Command::new(&command[0])
        .args(command[1..].iter())
        .env("SHLVL", "0")
        .current_dir(home_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn get_default_terminal() -> Vec<String> {
    if let Ok(terminal) = env::var("TERMINAL") {
        return vec![terminal];
    }

    // Check for DE-specific terminals
    if let Ok(desktop_environment) = env::var("XDG_CURRENT_DESKTOP") {
        match desktop_environment.as_str() {
            "GNOME" => return vec!["kgx".into(), "-e".into()],
            "KDE" => return vec!["konsole".into()],
            "XFCE" => return vec!["xfce4-terminal".into()],
            "LXQt" => return vec!["lxterminal".into()],
            _ => {}
        }
    }

    // Default to xterm if no terminal is found
    vec!["xterm".into()]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(state: AppState) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![subscribe_config, run_command,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
