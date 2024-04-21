//! This applications crate is a library crate that can be used to get a list of installed applications on your system.
//! Support MacOS, Windows, and Linux.
//! It returns app folder, app icon, and app name.
//! ```rust
//! use std::path::PathBuf;
//! #[derive(Debug)]
//! pub struct App {
//!     pub name: String,
//!     pub icon_path: Option<PathBuf>,
//!     pub app_path_exe: PathBuf, // Path to the .app file for mac, or Exec for Linux, or .exe for Windows
//!     pub app_desktop_path: PathBuf, // Path to the .desktop file for Linux, .app for Mac
//! }
//! ```
//!
//! For example, to get a list of installed applications on MacOS:
//!
//! ```rust
//! use applications::{get_apps};
//!
//! fn main() {
//!     let apps = get_apps();
//!     // print the list nicely into separate lines
//!     for app in apps {
//!         println!("{:#?}", app);
//!     }
//! }
//! ```
//!
//! On Mac to open a file with a specific app:
//! ```rust
//! use std::path::PathBuf;
//! use applications::{get_apps, open_file_with};
//! let file_path = PathBuf::from("/Users/hacker/Desktop/app/main.rs");
//! let app_path = PathBuf::from("/Applications/Visual Studio Code.app");
//!
//! open_file_with(file_path, app_path);
//! ```
//!
pub mod error;
pub mod prelude;
pub mod utils;
pub mod platforms;
pub mod common;
pub use platforms::*;