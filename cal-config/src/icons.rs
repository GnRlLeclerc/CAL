//! Indexation of application icons at launch to replace them by their paths

use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

/// Index desktop icons paths by name
pub fn load_icons() -> HashMap<String, String> {
    // NOTE: icon paths contain their resolution in their values.
    // SVGs have the maximum resolution possible.
    let mut icon_paths: HashMap<String, (usize, String)> = HashMap::new();

    let xdg_data_dirs = env::var("XDG_DATA_DIRS");
    match xdg_data_dirs {
        Ok(value) => {
            for dir in value.split(':') {
                let mut path = PathBuf::from(dir);
                path.push("icons/hicolor");
                process_icons_dir(&path, &mut icon_paths);
            }
        }
        Err(_) => {
            let base_path = "/usr/share/icons/hicolor";
            eprintln!("$XDG_DATA_DIRS not set, defaulting to {}", base_path);
            let path = Path::new(base_path);
            process_icons_dir(&path, &mut icon_paths);
        }
    }

    icon_paths.into_iter().map(|(k, v)| (k, v.1)).collect()
}

fn process_icons_dir(path: &Path, mut icon_paths: &mut HashMap<String, (usize, String)>) {
    if path.exists() {
        match path.read_dir() {
            Ok(entries) => entries
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .for_each(|entry| {
                    // Analyze each directory for the resolution
                    let path = entry.path();
                    let last_dir = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("unknown");

                    match last_dir {
                        "symbolic" => return,
                        "scalable" => {
                            insert_icons_with_weight(&path, usize::MAX, &mut icon_paths);
                        }
                        _ => {
                            let resolution = last_dir
                                .split('x')
                                .next()
                                .and_then(|s| s.parse::<usize>().ok())
                                .unwrap_or(0);
                            insert_icons_with_weight(&path, resolution, &mut icon_paths);
                        }
                    }
                }),
            Err(_) => {
                eprintln!("Error reading directory: {}", path.display());
            }
        }
    }
}

/// Recursively iterate over a path to load all icons.
/// These icons correspond to a specific resolution.
/// If an icon is already in the map, it is only overwritten if the resolution is higher.
fn insert_icons_with_weight(
    path: &Path,
    weight: usize,
    icon_paths: &mut HashMap<String, (usize, String)>,
) {
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    // Get the filename to get the icon it belongs to
                    if let Some(icon) = basename(entry.file_name()) {
                        icon_paths
                            .entry(icon.to_string())
                            .and_modify(|v| {
                                if v.0 < weight {
                                    v.0 = weight;
                                    v.1 = entry.path().to_string_lossy().to_string();
                                }
                            })
                            .or_insert((weight, entry.path().to_string_lossy().to_string()));
                    }
                }
            }
            Err(_) => eprintln!("Error reading directory: {}", path.display()),
        }
    }
}

/// Returns the basename of a file.
fn basename(filename: &OsStr) -> Option<&str> {
    filename.to_str().map(|s| {
        if let Some(last_dot_index) = s.rfind('.') {
            &s[..last_dot_index]
        } else {
            s
        }
    })
}
