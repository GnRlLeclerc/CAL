use cal_config::Config;
use tauri::{ipc::Channel, State};

/// Load the App Launcher entries via a channel
#[tauri::command]
fn subscribe_config(initial_config: State<'_, Config>, channel: Channel<Config>) {
    channel.send(initial_config.inner().clone()).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(config: Config) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(config)
        .invoke_handler(tauri::generate_handler![subscribe_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
