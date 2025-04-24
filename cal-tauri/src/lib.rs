use cal_config::Config;
use tauri::{ipc::Channel, State};

/// Load the App Launcher entries via a channel
#[tauri::command]
fn subscribe_config(initial_config: State<'_, Config>, channel: Channel<Config>) {
    channel.send(initial_config.inner().clone()).unwrap()
}

/// Run a command
#[tauri::command]
fn run_command(command: String) {
    let command = command.split_whitespace().collect::<Vec<_>>();

    std::process::Command::new(command[0])
        .args(command[1..].iter())
        .spawn()
        .expect("failed to execute command");
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
