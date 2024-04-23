//! This applications crate is a library crate that can be used to get a list of installed applications on your system.
//! Support MacOS, and Linux. Windows support will be added soon.
//!
//! # Examples
//!
//! ```rust
//! let mut ctx = crate::common::AppInfoContext::new();
//! ctx.refresh_apps().unwrap();        // this will block the thread
//! let apps = ctx.get_all_apps();
//! assert!(apps.len() > 0);
//! ```
//!
//! ```rust
//! fn main() {
//!     let mut ctx = crate::common::AppInfoContext::new();
//!     // this runs a refresh thread in the background, recommended for Mac, whose system_profiler command takes a few seconds to run
//!     ctx.refresh_apps_in_background();
//!     assert_eq!(ctx.is_refreshing(), true);
//!     thread::sleep(Duration::from_secs(5)); // let's wait for the refresh to finish, otherwise everything ends when main thread quits
//!     assert_eq!(ctx.is_refreshing(), false);
//!     assert!(ctx.get_all_apps().len() > 0);
//! }
//! ```

pub mod api;
pub mod common;
pub mod error;
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
        let mut ctx = crate::common::AppInfoContext::new();
        ctx.refresh_apps().unwrap();
        let apps = ctx.get_all_apps();
        assert!(apps.len() > 0);
    }
}
