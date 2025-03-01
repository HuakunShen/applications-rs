//! This applications crate is a library crate that can be used to get a list of installed applications on your system.
//! Support MacOS, and Linux. Windows support will be added soon.
//!
//! # Examples
//!
//! ```ignore
//! let mut ctx = crate::common::AppInfoContext::new();
//! ctx.refresh_apps().unwrap();        // this will block the thread
//! let apps = ctx.get_all_apps();
//! assert!(apps.len() > 0);
//! ```
//!
//! ```ignore
//! use applications::{AppInfoContext, AppInfo};
//!
//! fn main() {
//!     let mut ctx = AppInfoContext::new();
//!     ctx.refresh_apps().unwrap(); // must refresh apps before getting them
//!     let apps = ctx.get_all_apps();
//!     println!("Apps: {:#?}", apps);
//!     let frontmost_app = ctx.get_frontmost_application().unwrap();
//!     println!("Frontmost App: {:#?}", frontmost_app);
//!     let running_apps = ctx.get_running_apps();
//!     println!("Running Apps: {:#?}", running_apps);
//! }
//! ```

pub mod api;
pub mod common;
// difference platforms may have different implementation and signatures for each function, so platforms will not be public
mod platforms;
pub mod prelude;
pub mod utils;

pub use common::{App, AppInfo, AppInfoContext, AppTrait};

#[cfg(test)]
mod tests {
    use crate::AppInfo;

    #[test]
    fn get_all_apps() {
        let mut ctx = crate::common::AppInfoContext::new(vec![]);
        ctx.refresh_apps().unwrap();
        let apps = ctx.get_all_apps();
        assert!(apps.len() > 0);
    }
}
