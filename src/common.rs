//! Common Data Structures

use std::path::PathBuf;

use crate::utils::image::RustImageData;

#[derive(Debug)]
pub struct App {
    pub name: String,
    pub icon_path: Option<PathBuf>,
    pub app_path_exe: Option<PathBuf>, // Path to the .app file for mac, or Exec for Linux, or .exe for Windows
    pub app_desktop_path: PathBuf,     // Path to the .desktop file for Linux, .app for Mac
}

pub trait AppTrait {
    fn load_icon(&self) -> Option<RustImageData>;
}
