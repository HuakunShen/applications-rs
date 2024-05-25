use crate::common::App;
use std::path::PathBuf;
// use walkdir::WalskDir;
use anyhow::Result;

pub fn get_apps() -> Vec<App> {
    vec![]
}

pub fn open_file_with(file_path: PathBuf, app: App) {}

pub fn get_frontmost_application() -> Result<App> {
    todo!()
}

pub fn get_all_apps() -> Result<Vec<App>> {
    Ok(vec![])
}

pub fn get_running_apps() -> Vec<App> {
    vec![]
}
