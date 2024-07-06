use applications::{AppInfo, AppInfoContext};

fn main() {
    let mut ctx = AppInfoContext::new();
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    // println!("Apps: {:#?}", apps);

    // write apps to a json file
    let apps_json = serde_json::to_string_pretty(&apps).unwrap();
    std::fs::write("apps.json", apps_json).unwrap();

    // The following two methods are only available on macOS
    // let frontmost_app = ctx.get_frontmost_application().unwrap();
    // println!("Frontmost App: {:#?}", frontmost_app);

    // let running_apps = ctx.get_running_apps();
    // println!("Running Apps: {:#?}", running_apps);
}
