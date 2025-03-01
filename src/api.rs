use crate::common::{App, AppInfo, AppInfoContext, SearchPath};
use crate::platforms::{get_all_apps, get_frontmost_application, get_running_apps, open_file_with};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{self, Arc, Mutex};
use std::thread;

impl AppInfoContext {
    pub fn new(extra_search_paths: Vec<SearchPath>) -> Self {
        AppInfoContext {
            cached_apps: Arc::new(Mutex::new(vec![])),
            refreshing: Arc::new(AtomicBool::new(false)),
            extra_search_paths,
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
        let apps = get_all_apps(&self.extra_search_paths)?;
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
        let mut ctx = AppInfoContext::new(vec![]);
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

    #[test]
    fn get_all_apps() {
        let mut ctx = AppInfoContext::new(vec![]);
        ctx.refresh_apps().unwrap();
        let apps = ctx.get_all_apps();
        println!("Apps Length: {:#?}", apps.len());
        assert!(apps.len() > 0);
    }
}
