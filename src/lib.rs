//! This applications crate is a library crate that can be used to get a list of installed applications on your system.
//! Support MacOS, Windows, and Linux.

pub mod common;
pub mod error;
pub mod platforms;
pub mod prelude;
pub mod utils;
use prelude::*;
use std::{
    path::PathBuf,
    sync::{self, atomic::AtomicBool, Arc, Mutex},
    thread,
};

use common::{App, AppInfo, AppInfoContext};
pub use platforms::*;

impl AppInfoContext {
    pub fn new() -> Self {
        AppInfoContext {
            cached_apps: Arc::new(Mutex::new(vec![])),
            refreshing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn refresh_apps_in_background(&mut self) {
        let mut ctx = self.clone();
        if self.refreshing.load(sync::atomic::Ordering::Relaxed) {
            return;
        }
        self.refreshing.store(true, sync::atomic::Ordering::Relaxed);
        let refreshing = Arc::clone(&self.refreshing);
        thread::spawn(move || {
            ctx.refresh_apps().unwrap();
            refreshing.store(false, sync::atomic::Ordering::Relaxed);
        });
    }
}

impl AppInfo for AppInfoContext {
    /// Refresh cache of all apps, this is synchronous and could take a few seconds, especially on Mac
    fn refresh_apps(&mut self) -> Result<()> {
        self.refreshing.store(true, sync::atomic::Ordering::Relaxed);
        let apps = get_all_apps()?;
        *self.cached_apps.lock().unwrap() = apps;
        self.refreshing
            .store(false, sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn get_all_apps(&self) -> Vec<App> {
        self.cached_apps.lock().unwrap().clone()
    }

    fn open_file_with(&self, file_path: PathBuf, app: App) {
        open_file_with(file_path, app)
    }

    fn get_running_apps(&self) -> Vec<App> {
        get_running_apps()
    }

    fn get_frontmost_application(&self) -> Result<App> {
        get_frontmost_application()
    }

    fn is_refreshing(&self) -> bool {
        self.refreshing.load(sync::atomic::Ordering::Relaxed)
    }

    fn empty_cache(&mut self) {
        self.cached_apps.lock().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::common::{AppInfo, AppInfoContext};

    #[test]
    fn test_app_info() {
        let mut ctx = AppInfoContext::new();
        assert_eq!(ctx.get_all_apps().len(), 0);
        assert_eq!(ctx.is_refreshing(), false);
        ctx.refresh_apps().unwrap();
        assert_eq!(ctx.is_refreshing(), false);
        assert!(ctx.get_all_apps().len() > 0);
        ctx.empty_cache();
        assert_eq!(ctx.get_all_apps().len(), 0);
        assert_eq!(ctx.is_refreshing(), false);
        ctx.refresh_apps_in_background();
        assert_eq!(ctx.is_refreshing(), true);
        thread::sleep(Duration::from_secs(5));
        assert_eq!(ctx.is_refreshing(), false);
        assert!(ctx.get_all_apps().len() > 0);
    }
}
