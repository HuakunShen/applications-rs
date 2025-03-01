use applications::{api, AppInfo, AppInfoContext};
use std::path::PathBuf;

fn main() {
    let mut ctx = AppInfoContext::new(vec![]);
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);
}
