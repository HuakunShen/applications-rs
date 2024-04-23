use crate::common::{App, PlatformContext, PlatformTrait};
use std::path::PathBuf;
use toml;
use walkdir::WalskDir;

#[cfg(target_os = "windows")]
pub fn get_apps() -> Vec<App> {
    vec![]
}

#[cfg(target_os = "windows")]
pub fn open_file_with(file_path: PathBuf, app_path: PathBuf) {}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{get_apps, open_file_with};

    #[test]
    fn test_get_apps() {
        let apps = get_apps();
        for app in apps {
            println!("App: {:?}", app);
        }
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
