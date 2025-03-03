use applications::{common::SearchPath, AppInfo, AppInfoContext, AppTrait};

fn main() {
    let mut ctx = AppInfoContext::new(vec![SearchPath::new(
        std::path::PathBuf::from("C:\\Users\\shenh\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Scoop Apps"),
        1,
    )]);
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);
}
