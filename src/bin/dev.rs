use std::path::PathBuf;
use applications::{api, AppInfo, AppInfoContext};

fn main() {
    let mut ctx = AppInfoContext::new();
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);
}
