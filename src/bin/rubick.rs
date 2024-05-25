use std::path::PathBuf;
// use mslink::ShellLink;
use applications::App;
use lnk::ShellLink;
use walkdir::WalkDir;

fn parse_lnk(path: PathBuf) -> Option<App> {
    let shortcut = ShellLink::open(&path).unwrap();
    let icon = shortcut.icon_location();
    let rel_path = shortcut.relative_path();
    let work_dir = shortcut.working_dir();
    let link_info = shortcut.link_info();
    // let path = link_info.as_ref().unwrap().local_base_path();
    let name = shortcut.name();
    // shortcut.working_dir();
    println!("name: {:#?}", name);
    println!("rel_path: {:#?}", rel_path);
    println!("work_dir: {:#?}", work_dir);
    println!("shortcut: {:#?}", shortcut.icon_location());
    println!("icon::: {:#?}", shortcut.icon_location());

    // println!("{:#?}", shortcut.arguments());
    // println!("{:#?}", link_info);
    let exe: Option<PathBuf> = match link_info {
        Some(info) => {
            // let path = info.local_base_path();
            // println!("{:#?}", path);
            match info.local_base_path() {
                Some(path) => Some(PathBuf::from(path)),
                None => None,
            }
        }
        None => None,
    };
    let work_dir = match work_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            // if exe is not None, use the exe's parent directory
            match &exe {
                Some(exe) => exe.parent().unwrap().to_path_buf(),
                None => return None,
            }
            // if exe.is_some() {
            //     exe.unwrap().parent().unwrap().to_path_buf()
            // } else {
            //     // if exe is None, use the path of the lnk file
            //     return None
            //     // path.parent().unwrap().to_path_buf()
            // }
        }
    };
    // println!("{:#?}", path);
    // get filename stem
    // let icon_path: Option<PathBuf> = if let Some(icon) = icon {
    //     Some(PathBuf::from(icon))
    // } else {
    //     None
    // };
    let icon_path: Option<PathBuf> = icon.as_ref().map(PathBuf::from);

    Some(App {
        name: path.file_stem().unwrap().to_str().unwrap().to_string(),
        icon_path,
        app_path_exe: exe,
        app_desktop_path: work_dir,
    })
    // println!("{:#?}", shortcut);
}

fn main() {
    let start_menu = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs";
    let devclean_ui_lnk = PathBuf::from(
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\devclean-ui\\devclean-ui.lnk",
    );
    let docker_lnk =
        PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk");
    let vs_path = PathBuf::from(
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Visual Studio 2022.lnk",
    );
    let task_manager_lnk_path = PathBuf::from(
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\System Tools\\Task Manager.lnk",
    );
    let capcut_lnk = PathBuf::from("C:\\Users\\shenh\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\CapCut\\CapCut.lnk");
    let app = parse_lnk(vs_path);
    println!("{:#?}", app);

    // let target_dir = "C:\\path\\to\\your\\directory";

    // for entry in WalkDir::new(start_menu).into_iter().filter_map(|e| e.ok()) {
    //     let path = entry.path();
    //     if path.is_file() {
    //         if let Some(extension) = path.extension() {
    //             if extension == "lnk" {
    //                 println!("{}", path.display());
    //             }
    //         }
    //     }
    // }
    // // let lnk = parselnk::Lnk::from(docker_lnk).unwrap();

    // let start_menu = std::env::var("APPDATA").unwrap() + "\\Microsoft\\Windows\\Start Menu\\Programs";
    // list all files in the start menu
    // println!("start_menu: {}", start_menu);
    // for entry in std::fs::read_dir(start_menu).unwrap() {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path.is_file() {
    //         let filename = path.file_name().unwrap().to_str().unwrap();
    //         if filename.ends_with(".lnk") {
    //             // println!("{}", filename);
    //             let shortcut = lnk::ShellLink::open(path).unwrap();
    //             let icon = shortcut.icon_location();
    //
    //         }
    //     } else if path.is_dir() {
    //
    //     } else {
    //         println!("unknown file type: {:?}", path);
    //     }
    //     // let name = path.file_name().unwrap().to_str().unwrap();
    //     // println!("{}", name);
    //
    // }
}
// C:\Windows\Installer\{89C3B1AD-04F9-4A43-940D-51E26BC47942}\ProductIcon
// C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE
// C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\devenv.exe
// C:\ProgramData\Microsoft\Windows\Start Menu\Programs
