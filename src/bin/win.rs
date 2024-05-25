use std::collections::HashMap;
use winreg::enums::*;
use winreg::RegKey;

fn get_installed_apps() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut apps = HashMap::new();

    // Paths to the Uninstall registry keys
    let uninstall_keys = [
        (HKEY_LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\Uninstall"),
        (HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Uninstall"),
    ];

    for (hkey, path) in &uninstall_keys {
        let reg_key = RegKey::predef(*hkey).open_subkey(path)?;

        for subkey_name in reg_key.enum_keys().flatten() {
            if let Ok(subkey) = reg_key.open_subkey(&subkey_name) {
                if let Ok(display_name) = subkey.get_value::<String, &str>("DisplayName") {
                    if let Ok(display_version) = subkey.get_value::<String, &str>("DisplayVersion") {
                        apps.insert(display_name, display_version);
                    } else {
                        apps.insert(display_name, String::from("Unknown Version"));
                    }
                }
            }
        }
    }

    Ok(apps)
}

fn main() {
    match get_installed_apps() {
        Ok(apps) => {
            for (name, version) in &apps {
                println!("{}: {}", name, version);
            }
            println!("{} apps found", apps.capacity());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
