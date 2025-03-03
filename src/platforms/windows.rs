use crate::common::{App, SearchPath};
use crate::utils::image::{RustImage, RustImageData};
use crate::AppTrait;
use anyhow::Ok;
use parselnk::string_data;
use parselnk::Lnk;
use std::path::PathBuf;
use windows_icons::get_icon_by_path;
// use walkdir::WalskDir;
use anyhow::Result;
use lnk::ShellLink;
use serde_derive::Deserialize;
use serde_derive::Serialize;
// use std::ffi::OsString;
// use std::os::windows::ffi::OsStringExt;
use std::process::Command;
use walkdir::WalkDir;
// use winapi::um::winuser::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW};
use image;
use std::collections::HashSet;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerShellLnkParseResult {
    #[serde(rename = "IconLocation")]
    pub icon_location: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: String,
    #[serde(rename = "Arguments")]
    pub arguments: String,
    #[serde(rename = "Hotkey")]
    pub hotkey: String,
    #[serde(rename = "WindowStyle")]
    pub window_style: i64,
    #[serde(rename = "TargetPath")]
    pub target_path: String,
}

// fn run_powershell_script(script: &str) -> Result<String> {
//     let output = Command::new("powershell")
//         .arg("-Command")
//         .arg(script)
//         .output()?;
//     let output = String::from_utf8(output.stdout)?;
//     Ok(output)
// }

pub fn parse_lnk_with_powershell_1(lnk_path: PathBuf) -> anyhow::Result<PowerShellLnkParseResult> {
    let lnk_path = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk";

    let script = format!(
        r#"
        function Get-Shortcut {{
            param (
                [string]$Path
            )
            
            $shell = New-Object -ComObject WScript.Shell
            $shortcut = $shell.CreateShortcut($Path)
            
            $properties = @{{
                TargetPath = $shortcut.TargetPath
                Arguments  = $shortcut.Arguments
                Description = $shortcut.Description
                Hotkey = $shortcut.Hotkey
                IconLocation = $shortcut.IconLocation
                WindowStyle = $shortcut.WindowStyle
                WorkingDirectory = $shortcut.WorkingDirectory
            }}
            
            return [PSCustomObject]$properties
        }}

        Get-Shortcut -Path "{}" | ConvertTo-Json
    "#,
        lnk_path
    );

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(script)
        .output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    // let result: PowerShellLnkParseResult = serde_json::from_str(&output).unwrap();

    let json: PowerShellLnkParseResult = serde_json::from_str(&output.to_string())?;
    Ok(json)
}

pub fn parse_lnk_with_powershell_2(lnk_path: PathBuf) -> anyhow::Result<App> {
    let parsed_json = parse_lnk_with_powershell_1(lnk_path)?;
    let target_path = PathBuf::from(parsed_json.target_path);
    let desktop_path = if parsed_json.working_directory.len() == 0 {
        PathBuf::from(parsed_json.working_directory)
    } else {
        target_path.parent().unwrap().to_path_buf()
    };
    let icon_path = if parsed_json.icon_location.len() == 0 {
        None
    } else {
        Some(PathBuf::from(parsed_json.icon_location))
    };
    let name = if parsed_json.description.len() == 0 {
        target_path
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        let desc = parsed_json.description.clone();
        if desc.starts_with("Runs ") {
            // edge case for Tauri apps
            desc[5..].to_string()
        } else {
            desc
        }
    };
    let app = App {
        name: name,
        icon_path: icon_path,
        app_path_exe: Some(target_path),
        app_desktop_path: desktop_path,
    };
    Ok(app)
}

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

/// Windows have path like this "%windir%\\system32\\mstsc.exe"
/// This function will translate the path to the real path
fn translate_path_alias(path: PathBuf) -> PathBuf {
    let mut path_str = path.to_string_lossy().to_string();

    // Common Windows environment variables
    let env_vars = vec![
        "%windir%",
        "%SystemRoot%",
        "%ProgramFiles%",
        "%ProgramFiles(x86)%",
        "%ProgramData%",
        "%USERPROFILE%",
        "%APPDATA%",
        "%LOCALAPPDATA%",
        "%PUBLIC%",
        "%SystemDrive%",
    ];

    for var in env_vars {
        if path_str.starts_with(var) {
            let env_name = var.trim_matches('%');
            if let std::result::Result::Ok(value) = std::env::var(env_name) {
                path_str = path_str.replace(var, &value);
                return PathBuf::from(path_str);
            }
        }
    }

    path
}

fn strip_extended_prefix(path: PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy();
    if path_str.starts_with("\\\\?\\") {
        PathBuf::from(&path_str[4..])
    } else {
        path
    }
}

fn parse_lnk2(path: PathBuf) -> Option<App> {
    let Some(lnk) = Lnk::try_from(path.as_path()).ok() else {
        return None;
    };

    let icon = lnk.string_data.icon_location.clone().map(|icon| {
        if icon.to_string_lossy().starts_with("%") {
            translate_path_alias(PathBuf::from(icon))
        } else {
            icon
        }
    });
    let mut app_exe_path: Option<PathBuf> = lnk.string_data.relative_path.clone();

    if lnk.string_data.relative_path.is_none() {
        if let Some(icon_path) = icon.clone() {
            // Clone here before using
            let icon_path = PathBuf::from(icon_path);
            // if icon_path ends with .exe, then it is the app_exe_path

            if let Some(ext) = icon_path.extension() {
                if ext == "exe" {
                    app_exe_path = Some(translate_path_alias(icon_path));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    let abs_path = path.parent().unwrap().join(app_exe_path.clone().unwrap());
    let abs_path = std::fs::canonicalize(abs_path);
    let exe_path = if abs_path.is_ok() {
        strip_extended_prefix(abs_path.unwrap())
    } else {
        return None;
    };
    // let exe_path: PathBuf = match abs_path {
    //     Ok(path) => path.into(),
    //     Err(_) => return None,
    // };

    let work_dir = lnk.string_data.working_dir;
    let work_dir = match work_dir {
        Some(dir) => {
            if dir.to_string_lossy().starts_with("%") {
                translate_path_alias(PathBuf::from(dir))
            } else {
                dir
            }
        }
        None => exe_path.parent().unwrap().to_path_buf(),
    };

    // lnk.string_data.name_string could be wrong, e.g. GitKraken has "Unleash the"
    // let name = match lnk.string_data.name_string {
    //     Some(name) => name,
    //     None => path.file_stem().unwrap().to_str().unwrap().to_string(),
    // };
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    // if name == "CLion" {
    //     println!("{:#?}", path.clone());
    // }
    Some(App {
        name,
        icon_path: icon,
        app_path_exe: Some(exe_path),
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

pub fn get_all_apps(extra_search_paths: &Vec<SearchPath>) -> Result<Vec<App>> {
    // Create a HashSet of search paths starting with the default Windows paths
    let mut search_paths: HashSet<SearchPath> = HashSet::new();

    // Add default Windows paths with unlimited depth
    let appdata_path = format!(
        "{}\\Microsoft\\Windows\\Start Menu\\Programs",
        std::env::var("APPDATA").unwrap()
    );
    let default_paths = vec![
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs",
        &appdata_path,
    ];

    for path in default_paths {
        search_paths.insert(SearchPath::new(PathBuf::from(path), u8::MAX));
    }

    // Add extra search paths
    for path in extra_search_paths {
        search_paths.insert(path.clone());
    }

    let mut apps = vec![];
    for search_path in search_paths {
        if !search_path.path.exists() {
            continue;
        }

        for entry in WalkDir::new(search_path.path)
            .max_depth(search_path.depth as usize)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "lnk" {
                        let result = App::from_path(path.to_path_buf());
                        if result.is_ok() {
                            apps.push(result.unwrap());
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

impl AppTrait for App {
    fn load_icon(&self) -> Result<RustImageData> {
        let icon_path = match &self.icon_path {
            Some(path) => Some(path.clone()),
            None => self.app_path_exe.clone(),
        };
        match icon_path {
            Some(path) => {
                let icon_path_str = path.to_string_lossy();
                let icon = get_icon_by_path(&icon_path_str)
                    .map_err(|e| anyhow::anyhow!("Failed to get icon: {}", e))?;
                Ok(RustImageData::from_dynamic_image(
                    image::DynamicImage::ImageRgba8(icon),
                ))
            }
            None => Err(anyhow::anyhow!("No icon path found for the app")),
        }
    }

    fn from_path(path: PathBuf) -> Result<Self> {
        if let Some(extension) = path.extension() {
            if extension == "lnk" {
                if let Some(app) = parse_lnk2(path.clone()) {
                    return Ok(app);
                }
            }
        }
        Err(anyhow::anyhow!(
            "Failed to create App from path: {:?}",
            path
        ))
    }
}

pub fn load_icon(path: PathBuf) -> Result<RustImageData> {
    let icon_path_str = path.to_string_lossy();
    let icon = get_icon_by_path(&icon_path_str)
        .map_err(|e| anyhow::anyhow!("Failed to get icon: {}", e))?;
    Ok(RustImageData::from_dynamic_image(image::DynamicImage::ImageRgba8(icon)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_lnk() {
    //     let path =
    //         PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk");
    //     let app = parse_lnk(path);
    //     assert!(app.is_some());
    // }

    #[test]
    fn test_get_all_apps() {
        let extra_search_paths = Vec::new();
        let apps = get_all_apps(&extra_search_paths).unwrap();
        println!("{:#?}", apps);
        println!("{:#?}", apps.len());
        assert!(!apps.is_empty());
    }

    #[test]
    fn test_path_alias() {
        let path = PathBuf::from("%windir%\\system32\\mstsc.exe");
        let path = translate_path_alias(path);
        assert_eq!(
            path.to_string_lossy().to_lowercase(),
            "c:\\windows\\system32\\mstsc.exe"
        );
    }

    // #[test]
    // fn test_parse_lnk_with_powershell() {
    //     let path =
    //         PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk");
    //     let result = parse_lnk_with_powershell_1(path).unwrap();
    //     println!("{:#?}", result);
    // }
}
