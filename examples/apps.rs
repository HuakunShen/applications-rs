use applications::utils::image::RustImage;
use applications::{AppInfo, AppInfoContext, AppTrait};

fn main() {
    std::fs::create_dir_all("./icons").unwrap();
    let mut ctx = AppInfoContext::new();
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);
    let mut failed_count = 0;
    for app in apps {
        println!("App: {:#?}", app);
        let icon_result = app.load_icon();
        let icon = match icon_result {
            Ok(icon) => icon,
            Err(e) => {
                println!("Failed to load icon for {}: {}", app.name, e);
                failed_count += 1;
                continue;
            }
        };
        if let Err(e) = icon.save_to_path(&format!("./icons/{}.png", app.name)) {
            println!("Failed to save icon for {}: {}", app.name, e);
            failed_count += 1;
            continue;
        }
    }
    println!("Total failed to get/save icons: {}", failed_count);
}
