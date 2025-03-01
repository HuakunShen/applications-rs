use crate::common::{App, AppInfo, AppInfoContext};
use anyhow::Result;
use ini::ini;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Clone, Default, Eq, Hash, Serialize, Deserialize)]
pub struct AppIcon {
    name: String,
    path: PathBuf,
    dimensions: Option<u16>,
}

pub fn brute_force_find_entry(
    desktop_file_path: &Path,
    entry_names: Vec<&str>,
) -> Result<Option<String>> {
    let file = std::fs::File::open(desktop_file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                for entry_name in entry_names.iter() {
                    if line.starts_with(entry_name) {
                        // let entry = line.split("=").last().unwrap();
                        let entry = line[entry_name.len() + 1..line.len()].trim();
                        return Ok(Some(entry.to_string()));
                    }
                }
            }
            Err(_e) => {}
        }
    }
    Ok(None)
}

/// in case the icon in .desktop file cannot be parsed, use this function to manually find the icon
/// example /usr/share/applications/microsoft-edge.desktop icon cannot be parsed with ini crate
pub fn brute_force_find_icon(desktop_file_path: &Path) -> Result<Option<String>> {
    // read the desktop file into lines and find the icon line
    brute_force_find_entry(desktop_file_path, vec!["Icon", "icon"])
}

pub fn brute_force_find_exec(desktop_file_path: &Path) -> Result<Option<String>> {
    brute_force_find_entry(desktop_file_path, vec!["Exec", "exec"])
}


/// clean exec path by removing placeholder "%"" args
/// like %u, %U, %F
fn clean_exec_path(exec: &str) -> String {
    let cleaned: Vec<&str> = exec
        .split_whitespace()
        .take_while(|s| !s.starts_with('%')) // Take everything up to first % parameter
        .collect();

    cleaned.join(" ")
}

fn clean_app_path(path: &str) -> String {
    // Compile the regex to match %word patterns
    let re = Regex::new(r"%\w+").unwrap();
    let command = re.replace_all(path, "").trim().to_string();

    // Replace multiple whitespaces with a single space
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let command = re_whitespace.replace_all(&command, " ").to_string();

    command
}

/// return a tuple, first element is the app, second element is a boolean indicating if the desktop file has display
/// Some apps like url handlers don't have display
/// The display indicator is not reliable, default to true. It's false iff the desktop file contains `nodisplay=true`
pub fn parse_desktop_file(desktop_file_path: PathBuf) -> (App, bool) {
    let mut app = App::default();
    app.app_desktop_path = desktop_file_path.clone();
    let desktop_file_path_str = desktop_file_path.to_str().unwrap();
    let map = ini!(desktop_file_path_str);
    let desktop_entry_exists = map.contains_key("desktop entry");
    let mut display = true;
    if desktop_entry_exists {
        let desktop_entry = map["desktop entry"].clone();
        if desktop_entry.contains_key("nodisplay") {
            // I don't want apps like a url handler that doesn't have GUI
            let nodisplay = desktop_entry["nodisplay"].clone();
            match nodisplay {
                Some(nodisplay) => {
                    if nodisplay == "true" {
                        display = false;
                    }
                }
                None => {}
            }
        }

        let raw_exec = desktop_entry
            .get("exec")
            .cloned()
            // try to find it by brute if not found
            .or_else(|| brute_force_find_exec(&desktop_file_path).ok())
            .flatten();

        if let Some(exec) = raw_exec {
            app.app_path_exe = Some(PathBuf::from(clean_exec_path(&exec)));
        }

        if desktop_entry.contains_key("icon") {
            let icon = desktop_entry["icon"].clone();
            app.icon_path = Some(PathBuf::from(icon.unwrap()));
        } else {
            match brute_force_find_icon(&desktop_file_path) {
                Ok(icon) => {
                    app.icon_path = icon.map(|icon| PathBuf::from(icon));
                }
                Err(_) => {}
            };
        }
        if desktop_entry.contains_key("name") {
            let name = desktop_entry["name"].clone();
            app.name = name.unwrap();
        }
    }
    return (app, display);
}

pub fn get_all_apps() -> Result<Vec<App>> {
    // read XDG_DATA_DIRS env var
    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
    let xdg_data_dirs: Vec<&str> = xdg_data_dirs.split(':').collect();
    // make a string sett from xdg_data_dirs
    let mut search_dirs: HashSet<&str> = xdg_data_dirs.iter().cloned().collect();
    search_dirs.insert("/usr/share/applications");
    // get home dir of current user
    let home_dir = std::env::var("HOME").unwrap();
    let home_path = PathBuf::from(home_dir);
    let local_share_apps = home_path.join(".local/share/applications");
    search_dirs.insert(local_share_apps.to_str().unwrap());
    search_dirs.insert("/usr/share/xsessions");
    search_dirs.insert("/etc/xdg/autostart");
    search_dirs.insert("/var/lib/snapd/desktop/applications");
    let icons_db = find_all_app_icons()?;
    // for each dir, search for .desktop files
    let mut apps: HashSet<App> = HashSet::new();
    for dir in search_dirs {
        let dir = PathBuf::from(dir);
        if !dir.exists() {
            continue;
        }
        for entry in WalkDir::new(dir.clone()) {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().is_none() {
                continue;
            }

            if path.extension().unwrap() == "desktop" && path.is_file() {
                let (mut app, has_display) = parse_desktop_file(path.to_path_buf());
                // fill icon path if .desktop file contains only icon name
                if !has_display {
                    continue;
                }
                if app.icon_path.is_some() {
                    let icon_path = app.icon_path.clone().unwrap();
                    if !icon_path.exists() {
                        // let icon_name = icon_path.file_name().unwrap().to_str().unwrap();
                        if let Some(icons) = icons_db.get(icon_path.to_str().unwrap()) {
                            if let Some(icon) = icons.first() {
                                app.icon_path = Some(icon.path.clone());
                            }
                        } else {
                            app.icon_path = None;
                        }
                    }
                }
                apps.insert(app);
            }
        }
    }
    Ok(apps.iter().cloned().collect())
}

pub fn find_all_app_icons() -> Result<HashMap<String, Vec<AppIcon>>> {
    let hicolor_path: PathBuf = PathBuf::from("/usr/share/icons");
    let search_dirs = vec![hicolor_path];
    // filter out search_dirs that do not exist
    let search_dirs: Vec<PathBuf> = search_dirs.into_iter().filter(|dir| dir.exists()).collect();

    let mut set = HashSet::new();

    for dir in search_dirs {
        let dir = PathBuf::from(dir);
        if !dir.exists() {
            continue;
        }

        for entry in WalkDir::new(dir.clone()) {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            let path = entry.path();
            match path.extension() {
                Some(ext) => {
                    if ext == "png" {
                        let path_str = path.to_string_lossy().to_string();
                        let split: Vec<&str> = path_str.split("/").collect();
                        let dim_str = if split.len() < 6 {
                            None
                        } else {
                            split[5].split("x").last()
                        };
                        let dim = match dim_str {
                            Some(dim) => match dim.parse::<u16>() {
                                Ok(dim) => Some(dim),
                                Err(_) => None,
                            },
                            None => None,
                        };
                        set.insert(AppIcon {
                            name: path.file_name().unwrap().to_str().unwrap().to_string(),
                            path: path.to_path_buf(),
                            dimensions: dim, // dimensions,
                        });
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }
    let mut map: HashMap<String, Vec<AppIcon>> = HashMap::new();
    for icon in set {
        let name = icon.name.clone();
        let name = &name[0..name.len() - 4]; // remove .png
        if map.contains_key(name) {
            map.get_mut(name).unwrap().push(icon);
        } else {
            map.insert(name.to_string(), vec![icon]);
        }
    }
    // sort icons by dimensions
    for (_, icons) in map.iter_mut() {
        icons.sort_by(|a, b| {
            if a.dimensions.is_none() && b.dimensions.is_none() {
                return std::cmp::Ordering::Equal;
            }
            if a.dimensions.is_none() {
                return std::cmp::Ordering::Greater;
            }
            if b.dimensions.is_none() {
                return std::cmp::Ordering::Less;
            }
            b.dimensions.unwrap().cmp(&a.dimensions.unwrap())
        });
    }
    Ok(map)
}

pub fn open_file_with(file_path: PathBuf, app: App) {
    let exe_path = app.app_path_exe.unwrap();
    let exec_path_str = exe_path.to_str().unwrap();
    let file_path_str = file_path.to_str().unwrap();
    let output = std::process::Command::new(exec_path_str)
        .arg(file_path_str)
        .output()
        .expect("failed to execute process");
}

pub fn get_running_apps() -> Vec<App> {
    todo!()
}

/// TODO: this is not working yet, xprop gives the current app name, but we need to locate its .desktop file if possible
/// If I need to compare app name with app apps, then this function should be moved to AppInfoContext where there is a `cached_apps`
pub fn get_frontmost_application() -> Result<App> {
    let output = std::process::Command::new("xprop")
        .arg("-root")
        .arg("_NET_ACTIVE_WINDOW")
        .output()
        .expect("failed to execute process");

    let output = std::str::from_utf8(&output.stdout).unwrap();
    let id = output.split_whitespace().last().unwrap();

    let output = std::process::Command::new("xprop")
        .arg("-id")
        .arg(id)
        .arg("WM_CLASS")
        .output()
        .expect("failed to execute process");

    let output = std::str::from_utf8(&output.stdout).unwrap();
    let app_name = output.split('"').nth(1).unwrap();

    let apps = get_all_apps()?;
    for app in apps {
        if app.name == app_name {
            return Ok(app);
        }
    }

    Err(anyhow::Error::msg("No matching app found".to_string()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::process::Command;
    use std::str;

    use super::*;

    #[test]
    fn test_clean_app_path() {
        assert_eq!(clean_app_path("code %f").to_string(), "code");
        assert_eq!(clean_app_path("code %f %F").to_string(), "code");
        assert_eq!(clean_app_path("\"/home/hacker/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea\" %u").to_string(), "\"/home/hacker/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea\"");
    }

    #[test]
    fn test_get_apps() {
        let apps = get_all_apps().unwrap();
        // println!("App: {:#?}", apps);
        assert!(apps.len() > 0);
        // iterate through apps and find the onces whose name contains "terminal"
        for app in apps {
            if app.name.to_lowercase().contains("code") {
                println!("App: {:#?}", app);
            }
        }
    }

    #[test]
    fn test_find_all_app_icons() {
        let start = std::time::Instant::now();
        let icons_icons = find_all_app_icons().unwrap();
        let elapsed = start.elapsed();
        assert!(icons_icons.len() > 0);
        println!("Elapsed: {:?}", elapsed);
    }
}
