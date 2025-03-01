use windows_icons::get_icon_by_path;

fn main() {
    let icon_path = "C:\\Users\\shenh\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe";
    let icon = get_icon_by_path(icon_path).unwrap();
    icon.save("icon.png").unwrap();
}
