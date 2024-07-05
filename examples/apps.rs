use applications::{AppInfo, AppInfoContext};

fn main() {
    let mut ctx = AppInfoContext::new();
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);

    // The following two methods are only available on macOS
    let frontmost_app = ctx.get_frontmost_application().unwrap();
    println!("Frontmost App: {:#?}", frontmost_app);

    let running_apps = ctx.get_running_apps();
    println!("Running Apps: {:#?}", running_apps);
}
