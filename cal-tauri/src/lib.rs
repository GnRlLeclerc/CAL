use std::{env, process::Stdio};

use cal_config::{colors::Colors, Config};
use tauri::{ipc::Channel, State};

/// Load the App Launcher entries via a channel
#[tauri::command]
fn subscribe_config(initial_config: State<'_, Config>, channel: Channel<Config>) {
    channel.send(initial_config.inner().clone()).unwrap()
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
pub fn run(config: Config) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(config)
        .invoke_handler(tauri::generate_handler![subscribe_config, run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
