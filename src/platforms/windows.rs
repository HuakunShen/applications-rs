use crate::common::App;
use std::path::PathBuf;
// use walkdir::WalskDir;
use anyhow::Result;
use lnk::ShellLink;
// use serde_derive::Deserialize;
// use serde_derive::Serialize;
// use std::ffi::OsString;
// use std::os::windows::ffi::OsStringExt;
use std::process::Command;
use walkdir::WalkDir;
// use winapi::um::winuser::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW};

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PowerShellLnkParseResult {
//     #[serde(rename = "IconLocation")]
//     pub icon_location: String,
//     #[serde(rename = "Description")]
//     pub description: String,
//     #[serde(rename = "WorkingDirectory")]
//     pub working_directory: String,
//     #[serde(rename = "Arguments")]
//     pub arguments: String,
//     #[serde(rename = "Hotkey")]
//     pub hotkey: String,
//     #[serde(rename = "WindowStyle")]
//     pub window_style: i64,
//     #[serde(rename = "TargetPath")]
//     pub target_path: String,
// }

// fn run_powershell_script(script: &str) -> Result<String> {
//     let output = Command::new("powershell")
//         .arg("-Command")
//         .arg(script)
//         .output()?;
//     let output = String::from_utf8(output.stdout)?;
//     Ok(output)
// }

// pub fn parse_lnk_with_powershell(lnk_path: PathBuf) -> anyhow::Result<PowerShellLnkParseResult> {
//     let script = format!(
//         r#"
//         "#,
//         lnk_path.to_str().unwrap()
//     );
//     let output = run_powershell_script(&script)?;
//     let result: PowerShellLnkParseResult = serde_json::from_str(&output)?;
//     Ok(result)
// }

fn parse_lnk(path: PathBuf) -> Option<App> {
    let shortcut = ShellLink::open(&path).unwrap();
    let exe: Option<PathBuf> = match shortcut.link_info() {
        Some(info) => match info.local_base_path() {
            Some(path) => Some(PathBuf::from(path)),
            None => None,
        },
        None => None,
    };
    let work_dir = match shortcut.working_dir() {
        Some(dir) => PathBuf::from(dir),
        None => {
            // if exe is not None, use the exe's parent directory
            match &exe {
                Some(exe) => exe.parent().unwrap().to_path_buf(),
                None => return None,
            }
        }
    };
    let icon_path: Option<PathBuf> = shortcut.icon_location().as_ref().map(PathBuf::from);

    Some(App {
        name: path.file_stem().unwrap().to_str().unwrap().to_string(),
        icon_path,
        app_path_exe: exe,
        app_desktop_path: work_dir,
    })
}

pub fn open_file_with(file_path: PathBuf, app: App) {
    let mut command = Command::new(app.app_path_exe.unwrap());
    command.arg(file_path);
    command
        .spawn()
        .expect("Failed to open file with the specified application.");
}

pub fn get_frontmost_application() -> Result<App> {
    todo!();
    // unsafe {
    //     let hwnd = GetForegroundWindow();
    //     let mut buffer = vec![0u16; GetWindowTextLengthW(hwnd) as usize + 1];
    //     let len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
    //     if len > 0 {
    //         let window_title = OsString::from_wide(&buffer[..len as usize]);
    //         let app = App {
    //             name: window_title.to_string_lossy().into_owned(),
    //             icon_path: None,
    //             app_path_exe: None,
    //             app_desktop_path: None,
    //         };
    //         Ok(app)
    //     } else {
    //         Err(anyhow::anyhow!("Failed to get frontmost application."))
    //     }
    // }
}

pub fn get_all_apps() -> Result<Vec<App>> {
    let start_menu = PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs");
    // get appdata roaming path
    let appdata = PathBuf::from(std::env::var("APPDATA").unwrap());
    // let start_menu2 = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs", appdata);
    let start_menu2 = appdata.join("Microsoft\\Windows\\Start Menu\\Programs");

    let scan_targets = vec![start_menu, start_menu2];
    let mut apps = vec![];
    for target in scan_targets {
        for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "lnk" {
                        if let Some(app) = parse_lnk(path.to_path_buf()) {
                            apps.push(app);
                        }
                    }
                }
            }
        }
    }
    Ok(apps)
}

pub fn get_running_apps() -> Vec<App> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lnk() {
        let path =
            PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk");
        let app = parse_lnk(path);
        assert!(app.is_some());
    }

    #[test]
    fn test_get_all_apps() {
        let apps = get_all_apps().unwrap();
        // println!("{:#?}", apps.len());
        assert!(!apps.is_empty());
    }

    // #[test]
    // fn test_parse_lnk_with_powershell() {
    //     let path =
    //         PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk");
    //     let result = parse_lnk_with_powershell(path).unwrap();
    //     println!("{:#?}", result);
    // }
}
