use crate::common::App;
use anyhow::Result;
use core_foundation::{bundle::CFBundle, url::CFURL};
use glob::glob;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacSystemProfilerAppList {
    #[serde(rename = "SPApplicationsDataType")]
    pub spapplications_data_type: Vec<MacSystemProfilterAppInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacSystemProfilterAppInfo {
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "arch_kind")]
    pub arch_kind: String,
    pub last_modified: String,
    #[serde(rename = "obtained_from")]
    pub obtained_from: String,
    pub path: String,
    #[serde(rename = "signed_by")]
    pub signed_by: Option<Vec<String>>,
    pub version: Option<String>,
    pub info: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CFBundlePrimaryIcon {
    #[serde(rename = "CFBundleIconName")]
    cf_bundle_icon_name: Option<String>,
    #[serde(rename = "CFBundleIconFiles")]
    cf_bundle_icon_files: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CFBundleIcons {
    #[serde(rename = "CFBundlePrimaryIcon")]
    cf_bundle_primary_icon: Option<CFBundlePrimaryIcon>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InfoPlist {
    #[serde(rename = "CFBundleIconFile")]
    cf_bundle_icon_file: Option<String>,
    #[serde(rename = "CFBundleIcons")]
    cf_bundle_icons: Option<CFBundleIcons>,
    #[serde(rename = "CFBundleIcons~ipad")]
    cf_bundle_icons_ipad: Option<CFBundleIcons>,
    #[serde(rename = "CFBundleExecutable")]
    cf_bundle_executable: Option<String>,
    #[serde(rename = "CFBundleIconName")]
    cf_bundle_icon_name: Option<String>,
    #[serde(rename = "CFBundleIdentifier")]
    cf_bundle_identifier: Option<String>,
    #[serde(rename = "CFBundleInfoDictionaryVersion")]
    cf_bundle_info_dictionary_version: Option<String>,
    #[serde(rename = "CFBundleName")]
    cf_bundle_name: Option<String>,
    #[serde(rename = "CFBundlePackageType")]
    cf_bundle_package_type: Option<String>,
    #[serde(rename = "CFBundleShortVersionString")]
    cf_bundle_short_version_string: Option<String>,
    #[serde(rename = "CFBundleVersion")]
    cf_bundle_version: Option<String>,
    #[serde(rename = "CFBundleDisplayName")]
    cf_bundle_display_name: Option<String>,
}

impl InfoPlist {
    pub fn from_value(value: &plist::Value) -> Result<InfoPlist> {
        let info_plist = plist::from_value(value).unwrap();
        Ok(info_plist)
    }

    pub fn from_file(path: &PathBuf) -> Result<InfoPlist> {
        match plist::from_file(path) {
            Ok(info_plist) => Ok(info_plist),
            Err(_) => match plist::Value::from_file(path) {
                // using plist::Value is a workaround for the error "duplicate key: CFBundleShortVersionString"
                Ok(value) => Ok(InfoPlist::from_value(&value).unwrap()),
                Err(err) => Err(anyhow::Error::msg(format!(
                    "Fail to parse plist: {}",
                    err.to_string()
                ))),
            },
        }
    }

    pub fn from_string(s: &str) -> Result<InfoPlist> {
        Ok(plist::from_bytes(s.as_bytes()).expect("failed to read info.plist"))
    }
}

/// system_profiler command on mac is the simplest way I found to get a list of apps
/// This function runs the command and returns the stdout
pub fn run_system_profiler_to_get_app_list() -> Result<String> {
    let output = std::process::Command::new("system_profiler")
        .arg("SPApplicationsDataType")
        .arg("-json")
        .output()?;
    Ok(std::str::from_utf8(&output.stdout).unwrap().to_string())
}

pub fn run_mdfind_to_get_app_list() -> Result<Vec<String>> {
    let output = std::process::Command::new("mdfind")
        .arg("kMDItemKind == 'Application'")
        .output()?;
    let output = String::from_utf8(output.stdout)?;
    let lines = output.split("\n").map(|line| line.to_string()).collect();
    Ok(lines)
}

/// Mac App folder is very complicated, I made this struct with some helper functions to make it easier to work with
pub struct MacAppPath(PathBuf);

impl MacAppPath {
    pub fn new(path: PathBuf) -> Self {
        MacAppPath(path)
    }

    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    /// Check if the path is an app
    /// 1. It has to exist
    /// 2. It has to have a Info.plist file
    pub fn is_app(&self) -> bool {
        self.exists() && self.has_info_plist()
    }

    /// Check if the path has a Wrapper folder
    /// iOS apps can run on Apple Silicon Macs, but these apps have different structures
    /// iOS apps are wrapped in a Wrapper folder
    /// For normal Mac apps, this function will always return false
    /// because Mac apps don't have a Wrapper folder
    /// For iOS apps, this function will return true if the Wrapper folder exists
    pub fn has_wrapper(&self) -> bool {
        match self.get_wrapper_path() {
            Some(path) => path.exists(),
            None => false,
        }
    }

    /// Get the path to the Wrapper folder
    /// iPad apps are wrapped in a Wrapper folder
    pub fn get_wrapper_path(&self) -> Option<PathBuf> {
        match self.0.join("Wrapper") {
            path if path.exists() => Some(path),
            _ => None,
        }
    }

    /// Get the path to the first inner .app folder in the Wrapper, if it exists
    /// iPad apps are wrapped in a Wrapper folder
    /// Here we assume there is only one inner .app folder, otherwise the logic will get too complicated
    pub fn get_app_path_in_wrapper(&self) -> Option<PathBuf> {
        let wrapper_path = self.get_wrapper_path()?;
        let wrapper_path_str = wrapper_path.to_str()?;
        // search for .app in the wrapper
        let glob_path = format!("{}/*.app", wrapper_path_str);
        if let Some(e) = glob(&glob_path).expect("Failed to read glob pattern").next() {
            return Some(e.unwrap());
        }
        None
    }

    pub fn get_bundle(&self) -> CFBundle {
        CFBundle::new(CFURL::from_path(&self.0, true).expect("Fail to create CFURL"))
            .expect("Fail to create CFBundle")
    }

    pub fn get_executable_path_with_bundle(&self) -> Option<PathBuf> {
        let bundle = self.get_bundle();
        match bundle.executable_url() {
            Some(url) => url.to_path(),
            None => None,
        }
    }

    pub fn get_executable_path(&self) -> Option<PathBuf> {
        let plist_path = self.get_info_plist_path()?;
        match InfoPlist::from_file(&plist_path) {
            Ok(info_plist) => match info_plist.cf_bundle_executable {
                Some(executable) => Some(PathBuf::from(executable)),
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn has_info_plist(&self) -> bool {
        self.get_info_plist_path().is_some()
    }

    pub fn get_info_plist_path(&self) -> Option<PathBuf> {
        if self.has_wrapper() {
            let app_path_in_wrapper = self.get_app_path_in_wrapper()?;
            let path = app_path_in_wrapper.join("Info.plist"); // iOS apps doesn't have Contents folder
            match path.exists() {
                true => Some(path),
                false => None,
            }
        } else {
            let path = self.0.join("Contents").join("Info.plist");
            match path.exists() {
                true => Some(path),
                false => None,
            }
        }
    }

    /// Convert the MacAppPath to an App struct
    /// This function will return None if the path is not an app
    pub fn to_app(&self) -> Option<App> {
        if !self.is_app() {
            return None;
        }
        let info_plist_path = self.get_info_plist_path()?;
        let info_plist = InfoPlist::from_file(&info_plist_path).ok()?;
        // let bundle_name = info_plist.cf_bundle_name;
        // let bundle_display_name = info_plist.cf_bundle_display_name;
        // let name = if bundle_name.is_some() {
        //     bundle_name.unwrap()
        // } else if bundle_display_name.is_some() {
        //     bundle_display_name.unwrap()
        // } else {
        //     return None;
        // };
        // use path filename without .app extension
        let name = self.0.file_stem()?.to_str()?.to_string();
        let is_ios_app = self.has_wrapper();
        let icon_file_name = if is_ios_app {
            let icons = info_plist.cf_bundle_icons;
            match icons {
                Some(icons) => {
                    let primary_icon = icons.cf_bundle_primary_icon;
                    match primary_icon {
                        Some(icon) => {
                            let icon_files = icon.cf_bundle_icon_files;
                            match icon_files {
                                Some(icon_files) => {
                                    let first_icon_file: Option<String> =
                                        icon_files.first().map(|s| s.to_string());
                                    first_icon_file
                                }
                                None => None,
                            }
                        }
                        None => None,
                    }
                }
                None => None,
            }
        } else {
            info_plist.cf_bundle_icon_file
        };
        let contents_path = self.0.join("Contents");
        let resources_path = contents_path.join("Resources");
        let macos_path = contents_path.join("MacOS");

        let icon_path = match icon_file_name {
            Some(icon_file_name) => {
                // if icon_file_name doesn't have an extension, add .icns
                let icon_file_name = if icon_file_name.ends_with(".icns") {
                    icon_file_name
                } else {
                    format!("{}.icns", icon_file_name)
                };
                let icon_path = resources_path.join(icon_file_name);
                if icon_path.exists() {
                    Some(icon_path)
                } else {
                    None
                }
            }
            None => None,
        };
        let app_path_exe = match info_plist.cf_bundle_executable {
            Some(executable) => {
                let app_path_exe = macos_path.join(executable);
                if app_path_exe.exists() {
                    Some(app_path_exe)
                } else {
                    None
                }
            }
            None => None,
        };
        Some(App {
            name,
            icon_path,
            app_path_exe,
            app_desktop_path: self.0.clone(),
        })
    }
}

mod tests {
    use super::*;

    /// This test is to make sure all the fields in the InfoPlist struct are deserialized correctly
    #[test]
    fn test_path_is_app() {
        let output = run_system_profiler_to_get_app_list().unwrap();
        // parse output string in json format to MacSystemProfilerAppList
        let app_list_json = serde_json::from_str::<MacSystemProfilerAppList>(&output);
        assert!(app_list_json.is_ok());
        let app_list_json = app_list_json.unwrap();
        app_list_json
            .spapplications_data_type
            .iter()
            .for_each(|app| {
                let path = PathBuf::from(&app.path);
                let mac_app_path = MacAppPath::new(path.clone());
                if !mac_app_path.is_app() {
                    println!("Path is not an app: {:?}", path);
                }
            });
    }

    // this test may only run on my computer, with 2 special ipad apps
    #[test]
    fn test_get_app_path_in_wrapper() {
        let mac_app_path = MacAppPath::new(PathBuf::from("/Applications/Shadowrocket.app"));
        if !mac_app_path.exists() {
            return;
        }
        let app_path_in_wrapper = mac_app_path.get_app_path_in_wrapper();
        assert_eq!(
            app_path_in_wrapper.unwrap(),
            PathBuf::from(format!(
                "/Applications/Shadowrocket.app/Wrapper/Shadowrocket.app"
            ))
        );
        let mac_app_path = MacAppPath::new(PathBuf::from("/Applications/全民K歌.app/"));
        if !mac_app_path.exists() {
            return;
        }
        let app_path_in_wrapper = mac_app_path.get_app_path_in_wrapper();
        assert_eq!(
            app_path_in_wrapper.unwrap(),
            PathBuf::from(format!("/Applications/全民K歌.app/Wrapper/QQKSong.app"))
        );
    }

    /// Load all apps on the system and check if the Info.plist file can be loaded
    #[test]
    fn test_load_info_plist() {
        let output = run_system_profiler_to_get_app_list().unwrap();
        // parse output string in json format to MacSystemProfilerAppList
        let app_list_json = serde_json::from_str::<MacSystemProfilerAppList>(&output);
        assert!(app_list_json.is_ok());
        let app_list_json = app_list_json.unwrap();
        app_list_json
            .spapplications_data_type
            .iter()
            .for_each(|app| {
                let path = PathBuf::from(&app.path);
                let mac_app_path = MacAppPath::new(path.clone());
                let plist_path = mac_app_path.get_info_plist_path();
                if plist_path.is_none() {
                    return;
                }
                let plist_path = plist_path.unwrap();
                let info_plist =
                    InfoPlist::from_file(&plist_path).expect("failed to load info.plist");
                // let _ = info_plist.cf_bundle_executable.unwrap();
                // println!("Bundle Name: {:?}", info_plist.cf_bundle_name);
                // println!("Bundle Display Name: {:?}\n", info_plist.cf_bundle_display_name);
            });
    }

    #[test]
    fn test_to_app() {
        // "/Applications/Parallels Desktop.app/Contents/Info.plist"
        let mac_app_path = MacAppPath::new(PathBuf::from("/Applications/Discord.app"));
        let app = mac_app_path.to_app();
        println!("App: {:?}", app);
    }
}
