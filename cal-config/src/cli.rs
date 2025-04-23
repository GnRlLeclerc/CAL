//! CLI options for launching CAL

use crate::{
    Config,
    config::{from_json, from_toml},
};
use clap::Parser;
use std::path::PathBuf;
use std::{fs::read_to_string, path::Path};

/// Configurable App Launcher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to another config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

/// Generate the config from CLI args and config files
pub fn process_cli_config() -> Config {
    let mut config = Config::default();
    let args = Args::parse();

    if let Some(project_dirs) = directories_next::ProjectDirs::from("com", "GnRlLeclerc", "cal") {
        let cfg_dir = project_dirs.config_dir();

        vec!["config.toml", "config.json"]
            .iter()
            .map(|f| cfg_dir.join(f))
            .filter(|p| p.exists())
            .for_each(|p| {
                config.merge(read_config_from_path(&p));
            });
    }

    if let Some(cfg_path) = args.config {
        config.merge(read_config_from_path(&cfg_path));
    }

    config
}

/// Read a file from a path
fn read_from_path<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    read_to_string(path).expect("Failed to read config file")
}

/// Read a config file from a path (util function)
fn read_config_from_path<P>(path: P) -> Config
where
    P: AsRef<Path>,
{
    let ext = path
        .as_ref()
        .extension()
        .expect(&format!(
            "Config file {} must have an extension",
            path.as_ref().display(),
        ))
        .to_str()
        .expect("Config file extension must be a string");

    match ext {
        "toml" => from_toml(&read_from_path(path)).unwrap(),
        "json" => from_json(&read_from_path(path)).unwrap(),
        "yaml" | "yml" => panic!("YAML config files are not supported yet"),
        _ => {
            panic!(
                "Unsupported config file extension {} for file {}",
                ext,
                path.as_ref().display()
            );
        }
    }
}
