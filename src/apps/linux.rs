use std::collections::HashSet;
use crate::common::App;
use std::path::PathBuf;
use walkdir::WalkDir;
use ini::ini;


#[cfg(target_os = "linux")]
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
            app.app_path_exe = PathBuf::from(exec.unwrap());
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

#[cfg(target_os = "linux")]
pub fn get_apps() -> Vec<App> {
    // read XDG_DATA_DIRS env var
    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
    let xdg_data_dirs: Vec<&str> = xdg_data_dirs.split(':').collect();
    // make a string sett from xdg_data_dirs
    let mut search_dirs: HashSet<&str> = xdg_data_dirs.iter().cloned().collect();
    search_dirs.insert("/usr/share/applications");
    // get home dir of current user
    let home_dir = std::env::var("HOME").unwrap();
    let mut home_path = PathBuf::from(home_dir);
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
            // read .desktop file, get Exec, name, Icon

            // toml::from_str()
            // read file content

            if path.extension().unwrap() == "desktop" {
                // let file_content = std::fs::read_to_string(path).unwrap();
                let app = parse_desktop_file(path.to_path_buf());
                apps.push(app);
                // let file_content = file_content.trim();
                // // let read_content = toml::from_str(file_content);
                // let parsed_toml: Result<Value, toml::de::Error> = toml::from_str(&file_content);
                // println!("Parsed toml: {:?}", parsed_toml.map_err(|err| err.to_string()));
                // if parsed_toml.is_err() {
                //     continue;
                // }
                // println!("Parsed toml: {:?}", parsed_toml);

                // let parsed_toml = parsed_toml.unwrap();
                // apps.push(App {
                //     name: path.file_name().unwrap().to_string_lossy().into_owned(),
                //     icon_path: None,
                //     app_path_exe: path.to_path_buf(),
                //     app_desktop_path: path.to_path_buf(),
                // });
            }
        }
    }
    apps
}

#[cfg(target_os = "linux")]
pub fn open_file_with(file_path: PathBuf, app_path: PathBuf) {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::parse_desktop_file;

    use super::{get_apps, open_file_with};

    #[test]
    fn it_works() {
        let apps = get_apps();
        println!("Apps: {:?}", apps);
    }

    #[test]
    fn test_get_apps() {
        let apps = get_apps();
        for app in apps {
            println!("App: {:?}", app);
        }
    }

    #[test]
    fn test_parse_desktop_file() {
        let app = parse_desktop_file(PathBuf::from("/var/lib/snapd/desktop/applications/gitkraken_gitkraken.desktop"));
        println!("App: {:?}", app);
    }

    // #[test]
    // fn ios_app() {
    //     let path = PathBuf::from("/Applications/Surge.app");
    //     let found = find_ios_app_icon(path);
    //     println!("Found: {:?}", found);
    // }

    // #[test]
    // fn open_file_with_vscode() {
    //     let file_path = PathBuf::from("/Users/hacker/Desktop/new_IQA.py");
    //     let app_path = PathBuf::from("/Applications/Visual Studio Code.app");
    //     super::open_file_with(file_path, app_path);
    // }
}
