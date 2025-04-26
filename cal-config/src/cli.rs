//! CLI options for launching CAL

use crate::{
    Config,
    colors::{self, Colors},
    config::{DisplayMode, from_json, from_toml},
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

    /// Launch the daemon
    #[clap(short, long)]
    daemon: bool,

    /// Icon theme name
    #[arg(short, long)]
    icon_theme: Option<String>,

    /// Search text placeholder
    #[clap(short, long)]
    placeholder: Option<String>,

    /// Menu display mode
    #[clap(short, long)]
    mode: Option<DisplayMode>,
}

/// Generate the config from CLI args and config files
pub fn process_cli_config() -> Config {
    let args = Args::parse();
    let project_dirs = directories_next::ProjectDirs::from("com", "GnRlLeclerc", "cal");
    let cfg_dir = project_dirs.as_ref().map(|d| d.config_dir());

    // In order of priority
    // 1. Parse the config from the path specified in CLI args
    // 2. Parse the config from any of the valid configuration files
    // 3. Use the default config
    let mut config = args
        .config
        // 1. CLI config path
        .map(|cfg_path| read_config_from_path(&cfg_path))
        // 2. Config from default paths
        .or_else(|| {
            cfg_dir.and_then(|cfg_dir| {
                vec![cfg_dir.join("config.toml"), cfg_dir.join("config.json")]
                    .iter()
                    .filter(|p| p.exists())
                    .next()
                    .map(|p| read_config_from_path(&p))
            })
        })
        // 3. Use the default config
        .unwrap_or_else(|| from_toml("").unwrap());

    // Parse the colors from colors.toml or colors.json, overriding the base config if found
    let colors = cfg_dir.and_then(|cfg_dir| {
        vec![cfg_dir.join("colors.toml"), cfg_dir.join("colors.json")]
            .iter()
            .filter(|p| p.exists())
            .next()
            .map(|p| read_colors_from_path(&p))
    });

    // Override the config with CLI args
    config.icon_theme = args.icon_theme.or(config.icon_theme);
    config.daemon = args.daemon;
    config.placeholder = args.placeholder.or(config.placeholder);
    config.mode = args.mode.unwrap_or(config.mode);
    config.colors = colors.unwrap_or(config.colors);

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

fn read_colors_from_path<P>(path: P) -> Colors
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
        "toml" => colors::from_toml(&read_from_path(path)).unwrap(),
        "json" => colors::from_json(&read_from_path(path)).unwrap(),
        _ => {
            panic!(
                "Unsupported config file extension {} for file {}",
                ext,
                path.as_ref().display()
            );
        }
    }
}
