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

impl PlatformContext {
    pub fn new() -> Self {
        PlatformContext { cache_apps: vec![] }
    }

    pub async fn init(&mut self) -> Result<()> {
        self.refresh_apps()?;
        Ok(())
    }
}

impl PlatformTrait for PlatformContext {
    fn refresh_apps(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_all_apps(&self) -> Vec<App> {
        todo!()
    }

    fn open_file_with(&self, file_path: PathBuf, app: App) {
        todo!()
    }

    fn get_running_apps(&self) -> Vec<App> {
        todo!()
    }

    fn get_frontmost_application(&self) -> Result<App> {
        todo!()
    }
}

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
