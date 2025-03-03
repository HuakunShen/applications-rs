use crate::common::{App, AppTrait, SearchPath};
use crate::utils::image::{RustImage, RustImageData};
use crate::utils::mac::{
    run_mdfind_to_get_app_list, run_system_profiler_to_get_app_list, MacAppPath,
    MacSystemProfilerAppList, MacSystemProfilterAppInfo,
};
use anyhow::Result;
use cocoa::base::id;
use objc;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::{Path, PathBuf};
use tauri_icns::{IconFamily, IconType};
use walkdir::WalkDir;

#[deprecated]
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

#[deprecated]
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
        Some(all_icons[0].clone())
    } else if all_icons.len() == 0 {
        None
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

/// On Mac, the `open` command has a optional `-a` flag to specify the app to open the file with.
/// For example, opening `main.rs` with VSCode: `open -a "Visual Studio Code" main.rs`, where "Visual Studio Code.app" is the app folder name.
/// The `.app` can be included or discarded in the `open` command.
pub fn open_file_with(file_path: PathBuf, app: App) {
    let app_path = app.app_desktop_path; // on mac, desktop path is the .app path
    let mut command = std::process::Command::new("open");
    command.arg("-a");
    command.arg(app_path);
    command.arg(file_path);
    let output = command.output().expect("failed to execute process");
    println!("output: {:?}", output);
}

pub fn nsstring_to_string(nsstring: *mut Object) -> Result<String> {
    unsafe {
        let cstr: *const i8 = msg_send![nsstring, UTF8String];
        if !cstr.is_null() {
            Ok(std::ffi::CStr::from_ptr(cstr)
                .to_string_lossy()
                .into_owned())
        } else {
            Err(anyhow::Error::msg(
                "Fail to convert NSString to String".to_string(),
            ))
        }
    }
}

// fn path_to_app(path_str: String) -> Result<App> {
//     // convert path_str to PathBuf
//     let path = PathBuf::from(path_str);

//     let filename = path.file_name();

//     let filename = match filename {
//         Some(name) => name.to_str(),
//         None => {
//             return Err(Error::Generic("Fail to get filename".to_string()));
//         }
//     };
//     let filename = match filename {
//         Some(name) => name.to_string(),
//         None => {
//             return Err(Error::Generic("Fail to get filename".to_string()));
//         }
//     };
//     // if filename ends with .app, remove it
//     let filename = if filename.ends_with(".app") {
//         filename.trim_end_matches(".app").to_string()
//     } else {
//         filename
//     };
//     if !path.exists() {
//         return Err(Error::Generic("Path does not exist".to_string()));
//     }
//     let app = App {
//         name: filename,
//         icon_path: find_app_icns(path.clone()),
//         app_path_exe: path.clone(),
//         app_desktop_path: path.clone(),
//     };
//     Ok(app)
// }

pub fn get_frontmost_application() -> Result<App> {
    unsafe {
        let shared_workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let front_most_app: id = msg_send![shared_workspace, frontmostApplication];
        let bundle_url: id = msg_send![front_most_app, bundleURL];
        let path: id = msg_send![bundle_url, path];
        match nsstring_to_string(path) {
            Ok(path_str) => {
                let path = PathBuf::from(path_str);
                let app_path = MacAppPath::new(path.clone());
                match app_path.to_app() {
                    Some(app) => Ok(app),
                    None => Err(anyhow::Error::msg("Fail to get app")),
                }
            }
            Err(e) => Err(e),
        }
    }
}

/// This is a mac-only function
pub fn get_menu_bar_owning_application() -> Result<App> {
    unsafe {
        let shared_workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let menu_bar_owning_app: id = msg_send![shared_workspace, menuBarOwningApplication];
        let bundle_url: id = msg_send![menu_bar_owning_app, bundleURL];
        let path: id = msg_send![bundle_url, path];
        match nsstring_to_string(path) {
            Ok(path_str) => {
                let path = PathBuf::from(path_str);
                let app_path = MacAppPath::new(path.clone());
                match app_path.to_app() {
                    Some(app) => Ok(app),
                    None => Err(anyhow::Error::msg("Fail to get app".to_string())),
                }
            }
            Err(e) => Err(e),
        }
    }
}

pub fn get_all_apps_sys_profiler() -> Result<Vec<App>> {
    let output = run_system_profiler_to_get_app_list()?;
    // parse output string in json format to MacSystemProfilerAppList
    let app_list_json = serde_json::from_str::<MacSystemProfilerAppList>(&output);
    let app_list = match app_list_json {
        Ok(app_list) => app_list.spapplications_data_type,
        Err(e) => {
            return Err(anyhow::Error::msg(format!(
                "Fail to parse system_profiler output: {}",
                e
            )))
        }
    };
    let apps: Vec<App> = app_list
        .iter()
        .map(|app| app.to_owned().into())
        .filter_map(|x| x) // turn Vec<Option<App>> into Vec<App>
        .collect();
    Ok(apps)
}

pub fn get_all_apps_mdfind() -> Result<Vec<App>> {
    let apps_list = run_mdfind_to_get_app_list()?;
    Ok(apps_list
        .iter()
        .map(|app_path| MacAppPath::new(PathBuf::from(app_path)).to_app())
        .filter_map(|x| x)
        .collect())
}

/// Search apps in the given path iteratively by walking down the path, depth is the depth of the path
pub fn search_apps(path: PathBuf, depth: u8) -> Result<Vec<App>> {
    if depth == 0 {
        return Ok(vec![]);
    }

    let mut apps = Vec::new();
    // Set max_depth on WalkDir to limit traversal depth
    let walker = WalkDir::new(path).max_depth(depth.into());

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if path.is_dir() {
            // Check if the path has an extension and if it's an .app
            if let Some(ext) = path.extension() {
                if ext == "app" {
                    if let Ok(app) = App::from_path(path.to_path_buf()) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    Ok(apps)
}

pub fn get_all_apps(extra_search_paths: &Vec<SearchPath>) -> Result<Vec<App>> {
    let mut all_apps = get_all_apps_mdfind()?;
    let mut seen_paths = all_apps
        .iter()
        .map(|app| app.app_desktop_path.clone())
        .collect::<std::collections::HashSet<_>>();

    for path in extra_search_paths {
        let apps = search_apps(path.path.clone(), path.depth)?;
        for app in apps {
            if seen_paths.insert(app.app_desktop_path.clone()) {
                all_apps.push(app);
            }
        }
    }
    Ok(all_apps)
}

impl From<MacSystemProfilterAppInfo> for Option<App> {
    fn from(app_info: MacSystemProfilterAppInfo) -> Self {
        let app_path = MacAppPath::new(PathBuf::from(app_info.path));
        app_path.to_app()
    }
}

pub fn get_running_apps() -> Vec<App> {
    unsafe {
        let shared_workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let running_apps: id = msg_send![shared_workspace, runningApplications];
        let count: usize = msg_send![running_apps, count];
        let mut apps: Vec<App> = Vec::new();
        for i in 0..count {
            let app: id = msg_send![running_apps, objectAtIndex: i];
            let bundle_url: id = msg_send![app, bundleURL];
            let path: id = msg_send![bundle_url, path];
            let path_str = nsstring_to_string(path).unwrap();
            if let Ok(app) = App::from_path(PathBuf::from(path_str)) {
                apps.push(app);
            }
            // let app_path = MacAppPath::new(PathBuf::from(path_str));
            // match app_path.to_app() {
            //     Some(app) => apps.push(app),
            //     None => {}
            // }
        }
        apps
    }
}

/// path can be the path to .app folder or .icns file
pub fn load_icon(path: &Path) -> Result<RustImageData> {
    // check file type and file extension
    let file = File::open(path)
        .map_err(|e| anyhow::Error::msg(format!("Failed to open icon file: {}", e)))?;
    let file_type = file
        .metadata()
        .map_err(|e| anyhow::Error::msg(format!("Failed to get file metadata: {}", e)))?
        .file_type();
    let file_extension = path.extension().unwrap_or_default();
    if file_type.is_dir() {
        // it's a .app folder
        let app = App::from_path(path.to_path_buf())
            .map_err(|e| anyhow::Error::msg(format!("Failed to create App from path: {}", e)))?;
        app.load_icon()
    } else if file_extension == "icns" {
        let file = BufReader::new(file);
        let icon_family = IconFamily::read(file)
            .map_err(|e| anyhow::Error::msg(format!("Failed to read icon family: {}", e)))?;

        let mut largest_icon_type = IconType::Mask8_16x16;
        let mut largest_width = 0;
        for icon_type in icon_family.available_icons() {
            let icon_type_width = icon_type.pixel_width();
            if icon_type_width > largest_width {
                largest_width = icon_type_width;
                largest_icon_type = icon_type;
                if largest_width >= 64 {
                    // width 256 is large enough
                    break;
                }
            }
        }

        let largest_icon = icon_family.get_icon_with_type(largest_icon_type)?;
        let mut buffer: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut buffer);
        largest_icon
            .write_png(cursor)
            .map_err(|e| anyhow::Error::msg(format!("Failed to write PNG: {}", e)))?;

        let bytes: &[u8] = &buffer;
        RustImageData::from_bytes(bytes)
            .map_err(|e| anyhow::Error::msg(format!("Failed to create image from bytes: {}", e)))
        // Ok(RustImageData::from_dynamic_image(image::DynamicImage::ImageRgba8(icon)))
    } else {
        Err(anyhow::Error::msg("Failed to load icon"))
    }
}

impl AppTrait for App {
    fn load_icon(&self) -> Result<RustImageData> {
        if let Some(icon_path) = &self.icon_path {
            load_icon(icon_path)
        } else {
            Err(anyhow::Error::msg("No icon path available"))
        }
    }

    fn from_path(path: PathBuf) -> Result<Self> {
        MacAppPath::new(path.to_path_buf())
            .to_app()
            .ok_or(anyhow::Error::msg("Failed to create App from path"))
    }
}

// generate test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::mac::MacAppPath;
    use std::path::PathBuf;

    #[test]
    fn test_get_running_apps() {
        let apps = get_running_apps();
        println!("Apps: {:#?}", apps);
    }

    #[test]
    fn get_frontmost_application() {
        let app = super::get_frontmost_application();
        println!("Frontmost app: {:?}", app);
        let app = super::get_menu_bar_owning_application();
        println!("Menu bar owning app: {:?}", app);
    }

    #[test]
    fn get_all_apps() {
        let apps = super::get_all_apps(&vec![]).unwrap();
        assert!(apps.len() > 0);
    }

    #[test]
    fn find_info_plist() {
        let apps = super::get_all_apps(&vec![]).unwrap();
        for app in apps {
            let path = app.app_desktop_path;
            let mac_app_path = MacAppPath::new(path.clone());
            let info_plist_path = mac_app_path.get_info_plist_path();
            if info_plist_path.is_none() {
                println!("Info.plist not found: {:?}", path);
            }
        }
    }

    #[test]
    fn test_search_apps() {
        let apps = search_apps(PathBuf::from("/Applications"), 1).unwrap();
        println!("Apps: {:#?}", apps);
    }

    // #[test]
    // fn open_file_with_vscode() {
    //     let file_path = PathBuf::from("/Users/hacker/Desktop");
    //     let app_path = PathBuf::from("/Applications/Visual Studio Code.app");
    //     let app = MacAppPath::new(app_path).to_app().unwrap();
    //     super::open_file_with(file_path, app);
    // }
}
