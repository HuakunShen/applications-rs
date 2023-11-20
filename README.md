# apps-rs

> This crate is used to read available desktop applications on different platforms.

## Platforms

- [x] Mac
- [x] Linux
- [ ] Windows

## Usage

```rust
use std::path::PathBuf;
use applications::{get_apps, open_file_with};

fn main() {
    let apps = get_apps();
    for app in apps {
        println!("{:#?}", app);
    }

    let file_path = PathBuf::from("/User/username/Desktop/app/main.rs");
    let app_path = PathBuf::from("/Applications/Visual Studio Code.app");
    open_file_with(file_path, app_path);
}
```

## How?

> How and where to search for available desktop applications on each platform?

### Linux

Desktop applications are specified in files that ends with `.desktop`. `echo $XDG_DATA_DIRS` to see a list of paths where these desktop files could reside in.

The `.desktop` files are in toml format. Parse them with [toml](https://crates.io/crates/toml) crate. 

The `Exec` can be used to launch the app, and `Icon` field contains the app icon.

### MacOS

The simplest way is to search in `/Applications` folder. The app icon is in `.icns` format. 
Apple silicon macs can now run iOS apps. iOS app icons are in `.png` format.

`system_profiler` command can be used to get installed applications.

`system_profiler SPApplicationsDataType` is the command to use to get a full list of applications. The output format is not a standard format, greping is needed.

https://github.com/BurntSushi/ripgrep may be a good choice.

### Windows

https://crates.io/crates/winreg could be useful. Ask chatgpt for sample code.


## Libraries

- https://crates.io/crates/icns: Read and write icns files, convert into PNG format.


