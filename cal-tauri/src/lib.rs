use std::{env, process::Stdio};

use cal_config::{Config, Entry};
use cal_daemon::read_message_async;
use tauri::{ipc::Channel, Manager, RunEvent, State, WindowEvent};
use tokio::{net::UnixListener, sync::Mutex};

pub enum AppState {
    /// One run only: contains the full config to send to the frontend
    Once(Config),
    /// Daemon mode: contains the cached desktop entries for default usage of the launcher
    Daemon(UnixListener, Vec<Entry>),
}

/// Load the App Launcher entries via a channel
/// The frontend should connect to the rust backend only once, using this endpoint.
#[tauri::command]
async fn subscribe_config(
    state: State<'_, Mutex<AppState>>,
    channel: Channel<&Config>,
) -> Result<(), ()> {
    match &*state.lock().await {
        AppState::Once(config) => channel.send(config).unwrap(),
        AppState::Daemon(listener, entries) => {
            println!("Daemon mode: listening for commands...");
            while let Ok((mut stream, _)) = listener.accept().await {
                let config: Option<Config> = read_message_async(&mut stream).await.ok();
                if let Some(mut config) = config {
                    if config.entries.is_empty() {
                        config.entries = entries.clone();
                    }
                    config.daemon = true; // Signal to the frontend that it must remain open

                    channel.send(&config).unwrap();
                }
            }
        }
    }
    Ok(())
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
pub fn run(state: AppState, daemon: bool) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![subscribe_config, run_command])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |a, event| {
            // In daemon mode, cleanup the socket before exit,
            // and always hide the webview window instead of closing it.
            match (daemon, event) {
                (true, RunEvent::ExitRequested { .. }) => {
                    // Remove the socket
                    let _ = std::fs::remove_file(cal_daemon::SOCKET);
                }
                (
                    true,
                    RunEvent::WindowEvent {
                        label,
                        event: WindowEvent::CloseRequested { api, .. },
                        ..
                    },
                ) => {
                    api.prevent_close();
                    a.get_webview_window(&label).unwrap().hide().unwrap();
                }
                _ => {}
            }
        });
}
