use applications::{get_apps, open_file_with};

fn main() {
    let apps = get_apps();
    for app in apps {
        println!("App: {:?}", app);
    }
}
