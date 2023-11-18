use crate::common::App;
use std::path::PathBuf;
use walkdir::WalkDir;

#[cfg(target_os = "macos")]
fn find_ios_app_icon(app_path: PathBuf) -> Option<PathBuf> {
    // find all png files in the app_path, search for AppIcon ignore case in the pngs
    let mut all_icons: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(app_path.clone()) {
        if entry.is_err() {
            return None;
        }
        let entry = entry.unwrap();
        if entry.path().extension().is_none() {
            continue;
        }
        if entry.path().extension().unwrap() == "png" {
            all_icons.push(entry.path().to_path_buf());
        }
    }
    return if all_icons.len() == 0 {
        None
    } else if all_icons.len() == 1 {
        Some(all_icons[0].clone())
    } else {
        // more than one png found, search for keyword AppIcon, ignore case
        // filter to get png with AppIcon in name, ignore case
        // sort all_icons by path length, shortest first
        all_icons.sort_by(|a, b| a.to_str().unwrap().len().cmp(&b.to_str().unwrap().len()));
        let filtered_all_icons = all_icons
            .iter()
            .filter(|&x| {
                x.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("appicon")
            })
            .collect::<Vec<_>>();
        if filtered_all_icons.len() == 1 {
            Some(filtered_all_icons[0].clone())
        } else if filtered_all_icons.len() == 0 {
            Some(all_icons[0].clone())
        } else {
            // filtered_all_icons more than 1, use the one with shortest length
            Some(filtered_all_icons[0].clone())
        }
    };
}

#[cfg(target_os = "macos")]
pub fn find_app_icns(app_path: PathBuf) -> Option<PathBuf> {
    // default location: Contents/Resources/AppIcon.icns
    let contents_path = app_path.join("Contents");
    if !contents_path.exists() {
        // this may be a ios app, look for png app icon
        return find_ios_app_icon(app_path);
    }
    let resources_path = contents_path.join("Resources");
    let default_icns_path = resources_path.join("AppIcon.icns");
    if default_icns_path.exists() {
        return Some(default_icns_path);
    }
    let mut all_icons: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(contents_path.clone()) {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();
        if entry.path().extension().is_none() {
            continue;
        }
        if entry.path().extension().unwrap() == "icns" {
            all_icons.push(entry.path().to_path_buf());
        }
    }
    if all_icons.len() == 1 {
        return Some(all_icons[0].clone());
    } else if all_icons.len() == 0 {
        return None;
    } else {
        // more than one icon found
        // search for appicon in name, ignore case
        // sort all_icons by path length, shortest first
        all_icons.sort_by(|a, b| a.to_str().unwrap().len().cmp(&b.to_str().unwrap().len()));
        let filtered_all_icons = all_icons
            .iter()
            .filter(|&x| {
                x.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("appicon")
            })
            .collect::<Vec<_>>();
        if filtered_all_icons.len() == 1 {
            Some(filtered_all_icons[0].clone())
        } else if filtered_all_icons.len() == 0 {
            Some(all_icons[0].clone())
        } else {
            // filtered_all_icons more than 1, use the one with shortest length
            Some(filtered_all_icons[0].clone())
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_apps() -> Vec<App> {
    let applications_folder = PathBuf::from("/Applications");
    // iterate this folder
    // for each .app file, create an App struct
    // return a vector of App structs
    // list all files in applications_folder
    let mut apps: Vec<App> = Vec::new();
    for entry in applications_folder
        .read_dir()
        .expect("Unable to read directory")
    {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().is_none() {
                continue;
            }
            if path.extension().unwrap() == "app" {
                // search for .icns in Contents/Resources
                let app = App {
                    name: path.file_name().unwrap().to_string_lossy().into_owned(),
                    icon_path: find_app_icns(path.clone()),
                    app_path_exe: path.clone(),
                    app_desktop_path: path.clone(),
                };
                apps.push(app);
            }
        }
    }
    apps
}

/// On Mac, the `open` command has a optional `-a` flag to specify the app to open the file with.
/// For example, opening `main.rs` with VSCode: `open -a "Visual Studio Code" main.rs`, where "Visual Studio Code.app" is the app folder name.
/// The `.app` can be included or discarded in the `open` command.
#[cfg(target_os = "macos")]
pub fn open_file_with(file_path: PathBuf, app_path: PathBuf) {
    let mut command = std::process::Command::new("open");
    command.arg("-a");
    command.arg(app_path);
    command.arg(file_path);
    let output = command.output().expect("failed to execute process");
    println!("output: {:?}", output);
}

// generate test
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{find_ios_app_icon, get_apps};

    #[test]
    fn it_works() {
        let apps = get_apps();
        println!("Apps: {:?}", apps);
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
