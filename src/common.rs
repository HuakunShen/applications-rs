//! Common Data Structures
use crate::prelude::*;
use crate::utils::image::RustImageData;
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub icon_path: Option<PathBuf>,
    pub app_path_exe: Option<PathBuf>, // Path to the .app file for mac, or Exec for Linux, or .exe for Windows
    pub app_desktop_path: PathBuf,     // Path to the .desktop file for Linux, .app for Mac
}

pub trait AppTrait {
    fn load_icon(&self) -> Option<RustImageData>;
}

pub trait AppInfo {
    /// It could take a few seconds to retrieve all apps, so a cache needs to be maintained
    /// This method is used to refresh the cache
    fn refresh_apps(&mut self) -> Result<()>;
    fn get_all_apps(&self) -> Vec<App>;
    fn open_file_with(&self, file_path: PathBuf, app: App);
    fn get_running_apps(&self) -> Vec<App>;
    fn get_frontmost_application(&self) -> Result<App>;
    fn is_refreshing(&self) -> bool;
    fn empty_cache(&mut self);
}

#[derive(Debug, Clone)]
pub struct AppInfoContext {
    pub cached_apps: Arc<Mutex<Vec<App>>>,
    pub refreshing: Arc<AtomicBool>,
}
