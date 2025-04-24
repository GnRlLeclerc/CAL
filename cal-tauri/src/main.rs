// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cal_config::cli::process_cli_config;

fn main() {
    let config = process_cli_config();
    launcher_lib::run(config)
}
