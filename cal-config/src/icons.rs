//! Indexation of application icons at launch to replace them by their paths

use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

/// Index desktop icons paths by name
pub fn load_icons(theme: &Option<String>) -> HashMap<String, String> {
    // NOTE: icon paths contain their resolution in their values.
    // SVGs have the maximum resolution possible.
    // The tuples represent (resolution, is_fallback, path)
    let mut icon_paths: HashMap<String, (usize, bool, String)> = HashMap::new();
    let icon_dirs = &["icons/hicolor", "pixmaps"];

    let xdg_data_dirs = env::var("XDG_DATA_DIRS");
    match xdg_data_dirs {
        Ok(value) => {
            for dir in value.split(':') {
                if let Some(theme) = &theme {
                    let mut base_path = PathBuf::from(dir);
                    base_path.push("icons");
                    base_path.push(theme);
                    process_icons_dir(&base_path, false, &mut icon_paths);
                }

                let path = PathBuf::from(dir);
                icon_dirs.iter().for_each(|icon_dir| {
                    process_icons_dir(&path.join(icon_dir), true, &mut icon_paths);
                });
            }
        }
        Err(_) => {
            eprintln!("$XDG_DATA_DIRS not set, defaulting to /usr/share/icons");

            if let Some(theme) = theme {
                let mut base_path = PathBuf::from("/usr/share/icons");
                base_path.push(theme);
                process_icons_dir(&base_path, false, &mut icon_paths);
            }

            let base_path = PathBuf::from("/usr/share");
            icon_dirs.iter().for_each(|icon_dir| {
                process_icons_dir(&base_path.join(icon_dir), true, &mut icon_paths);
            });
        }
    }

    icon_paths.into_iter().map(|(k, v)| (k, v.2)).collect()
}

fn process_icons_dir(
    path: &Path,
    is_fallback: bool,
    mut icon_paths: &mut HashMap<String, (usize, bool, String)>,
) {
    if path.exists() {
        if path.iter().last() == Some(OsStr::new("pixmaps")) {
            insert_icons_with_weight(path, 0, is_fallback, icon_paths);
            return;
        }

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
                            insert_icons_with_weight(
                                &path,
                                usize::MAX,
                                is_fallback,
                                &mut icon_paths,
                            );
                        }
                        _ => {
                            let resolution = last_dir
                                .split('x')
                                .next()
                                .and_then(|s| s.parse::<usize>().ok())
                                .unwrap_or(0);
                            insert_icons_with_weight(
                                &path,
                                resolution,
                                is_fallback,
                                &mut icon_paths,
                            );
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
    is_fallback: bool,
    icon_paths: &mut HashMap<String, (usize, bool, String)>,
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
                                if (v.0 < weight && is_fallback <= v.1) || (v.1 > is_fallback) {
                                    v.0 = weight;
                                    v.1 = is_fallback;
                                    v.2 = entry.path().to_string_lossy().to_string();
                                }
                            })
                            .or_insert((
                                weight,
                                is_fallback,
                                entry.path().to_string_lossy().to_string(),
                            ));
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
