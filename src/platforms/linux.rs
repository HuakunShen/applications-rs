use crate::common::{App, PlatformContext, PlatformTrait};
use ini::ini;
use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn parse_desktop_file(desktop_file_path: PathBuf) -> App {
    let mut app = App::default();
    app.app_desktop_path = desktop_file_path.clone();
    let desktop_file_path_str = desktop_file_path.to_str().unwrap();
    let map = ini!(desktop_file_path_str);
    let desktop_entry_exists = map.contains_key("desktop entry");
    if desktop_entry_exists {
        let desktop_entry = map["desktop entry"].clone();
        if desktop_entry.contains_key("exec") {
            let exec = desktop_entry["exec"].clone();
            app.app_path_exe = Some(PathBuf::from(exec.unwrap()));
        }
        if desktop_entry.contains_key("icon") {
            let icon = desktop_entry["icon"].clone();
            app.icon_path = Some(PathBuf::from(icon.unwrap()));
        }
        if desktop_entry.contains_key("name") {
            let name = desktop_entry["name"].clone();
            app.name = name.unwrap();
        }
    }
    return app;
}

pub fn get_all_apps() -> Vec<App> {
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
    // for each dir, search for .desktop files
    let mut apps: Vec<App> = Vec::new();
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

            if path.extension().unwrap() == "desktop" {
                let app = parse_desktop_file(path.to_path_buf());
                apps.push(app);
            }
        }
    }
    apps
}

pub fn open_file_with(file_path: PathBuf, exec_path: PathBuf) {
    let exec_path_str = exec_path.to_str().unwrap();
    let file_path_str = file_path.to_str().unwrap();
    let output = std::process::Command::new(exec_path_str)
        .arg(file_path_str)
        .output()
        .expect("failed to execute process");
    println!("Output: {:?}", output);
}

#[cfg(test)]
mod tests {
    use crate::parse_desktop_file;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_get_apps() {
        let apps = get_all_apps();
        // println!("App: {:#?}", apps);
        assert!(apps.len() > 0);
    }

    #[test]
    fn test_parse_desktop_file() {
        let app = parse_desktop_file(PathBuf::from(
            "/var/lib/snapd/desktop/applications/gitkraken_gitkraken.desktop",
        ));
        println!("App: {:#?}", app);
    }

    // #[test]
    // fn ios_app() {
    //     let path = PathBuf::from("/Applications/Surge.app");
    //     let found = find_ios_app_icon(path);
    //     println!("Found: {:?}", found);
    // }

    // #[test]
    // fn open_file_with_vscode() {
    //     let file_path = PathBuf::from("/home/huakun/Desktop/CCC");
    //     let app_path = PathBuf::from("code");
    //     open_file_with(file_path, app_path);
    // }
}
