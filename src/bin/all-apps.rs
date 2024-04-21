// use gio::traits::AppInfoExt as _;
use gtk::{prelude::IconThemeExt as _, IconLookupFlags};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    display_name: String,
    icon: Option<String>,
    command: Option<String>,
}

// pub fn get_all_applications() -> Vec<AppInfo> {
//     let icon_theme = gtk::IconTheme::default().expect("default icon theme");

//     let mut info = gio::AppInfo::all()
//         .iter()
//         .map(|info| AppInfo {
//             display_name: info.display_name().to_string(),
//             icon: info
//                 .icon()
//                 .map(|s| icon_theme.lookup_by_gicon(&s, 48, IconLookupFlags::empty()))
//                 .flatten()
//                 .map(|icon| icon.filename())
//                 .flatten()
//                 .map(|path| path.to_string_lossy().to_string()),
//             command: info
//                 .commandline()
//                 .map(|path| path.to_string_lossy().to_string()),
//         })
//         .collect::<Vec<_>>();
//     info.sort_by(|a, b| a.display_name.cmp(&b.display_name));
//     info.dedup_by(|a, b| a.display_name == b.display_name);
//     info
// }

// pub fn launch_application(command: String) {
//     let app_info =
//         gio::AppInfo::create_from_commandline(&command, None, gio::AppInfoCreateFlags::empty())
//             .expect("create app info from command");
//     app_info
//         .launch(&[], None::<&gio::AppLaunchContext>)
//         .expect("launch app");
// }


fn main() {
    // let all_app_info = gio::AppInfo:;
    // gtk::init().expect("gtk init");
    // let icon_theme = gtk::IconTheme::default().expect("default icon theme");
    // println!("Icon Theme: {:#?}", icon_theme);
    // println!("Length: {}", all_app_info.len());
    // all_app_info.iter().for_each(|app_info| {
    //     println!("{:#?}", app_info);
    // });
    // let apps = get_all_applications();
    // for app in apps {
    //     println!("{:#?}", app);
    // }
}