use parselnk::Lnk;
use std::{convert::TryFrom, path::PathBuf};

fn main() {
    // let lnk_path = std::path::Path::new(r"c:\users\me\desktop\slack.lnk");
    // let lnk = parselnk::Lnk::from(lnk_path).unwrap();
    // let path = std::path::Path::new("c:\\users\\me\\shortcut.lnk");
    let lnk_path =
        PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk");
    let lnk_path = PathBuf::from("C:\\Users\\shenh\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\CapCut\\CapCut.lnk");
    let lnk_path = PathBuf::from(
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\devclean-ui\\devclean-ui.lnk",
    );
    let lnk_path = PathBuf::from(
        "C:\\Users\\shenh\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Lark.lnk",
    );
    let lnk = Lnk::try_from(lnk_path.as_path()).unwrap();
    println!("{:#?}", lnk);
    // println!("{:#?}", lnk.link_info);
    println!("working dir: {:#?}", lnk.working_dir());
    let ext_data = lnk.extra_data;
    // println!("Header: {:#?}", lnk.header);
    println!("Icon Location: {:?}", lnk.string_data.icon_location);
    println!("Icon Name String: {:?}", lnk.string_data.name_string);
    println!("Icon Rel Path: {:?}", lnk.string_data.relative_path);
    let abs_path = lnk_path
        .clone()
        .parent()
        .unwrap()
        .join(lnk.string_data.relative_path.unwrap());
    let abs_path = std::fs::canonicalize(abs_path).unwrap();
    println!("Icon Abs Path: {:?}", abs_path);
    println!("Exists: {}", abs_path.exists());
    // println!("Icon Rel Path: {:?}", lnk.string_data.);
    println!("Icon Working Dir: {:?}", lnk.string_data.working_dir);

    // ext_data.
    // println!("{:#?}", ext_data);
}
