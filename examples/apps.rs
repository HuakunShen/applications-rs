use std::path::PathBuf;

use applications::{AppInfo, AppInfoContext};
use windows_icons::get_icon_by_path;

fn sanitize_filename(name: &str) -> String {
    // Replace invalid filename characters with underscores
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn main() {
    std::fs::create_dir_all("./images").unwrap();
    let mut ctx = AppInfoContext::new();
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);
    let mut failed_count = 0;
    for app in apps {
        println!("App: {:#?}", app);
        let icon_path = match app.icon_path {
            Some(path) => path,
            None => app.app_path_exe.unwrap(),
        };
        println!("Icon Path: {:#?}", icon_path);
        if let Ok(icon) = get_icon_by_path(&icon_path.to_string_lossy()) {
            let safe_filename = sanitize_filename(&app.name);
            let output_path = PathBuf::from(format!("./images/{}.png", safe_filename));
            if let Err(e) = save_icon_to_disk(icon, &output_path) {
                println!("Failed to save icon for {}: {}", app.name, e);
                failed_count += 1;
            }
        } else {
            println!("Failed to get icon for {}", app.name);
            failed_count += 1;
        }
    }
    println!("Total failed to get/save icons: {}", failed_count);
    // write apps to a json file
    // let apps_json = serde_json::to_string_pretty(&apps).unwrap();
    // std::fs::write("apps.json", apps_json).unwrap();

    // The following two methods are only available on macOS
    // let frontmost_app = ctx.get_frontmost_application().unwrap();
    // println!("Frontmost App: {:#?}", frontmost_app);

    // let running_apps = ctx.get_running_apps();
    // println!("Running Apps: {:#?}", running_apps);
}

fn save_icon_to_disk(
    icon: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    output_path: &PathBuf,
) -> Result<(), image::ImageError> {
    icon.save(output_path)
}
