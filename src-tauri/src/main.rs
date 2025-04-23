// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

/// Configurable App Launcher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Choices for the launcher (path to a file). Defaults to standard app launcher
    #[arg(short, long)]
    choices: Option<String>,

    /// Run a daemon in the background to make subsequent launches faster
    #[arg(short, long, default_value_t = false)]
    daemon: bool,
}

fn main() {
    let args = Args::parse();

    if args.daemon {
        todo!("Daemon mode is not implemented yet");
    }

    launcher_lib::run()
}
