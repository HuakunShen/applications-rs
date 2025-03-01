use image::{ImageBuffer, Rgba};
use std::ffi::OsString;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use winapi::shared::windef::HICON;
use winapi::um::shellapi::ExtractIconW;
use winapi::um::wingdi::{CreateDIBSection, GetDIBits, BITMAP, BITMAPINFO, BI_RGB, DIB_RGB_COLORS};
use winapi::um::winuser::{DestroyIcon, GetIconInfo, ICONINFO};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug)]
struct AppInfo {
    name: String,
    path: Option<PathBuf>,
    icon_path: Option<PathBuf>,
}

fn get_installed_apps() -> io::Result<Vec<AppInfo>> {
    let mut apps = Vec::new();

    // System-wide installations (64-bit and 32-bit)
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

    // Check 64-bit applications
    if let Ok(uninstall_key) =
        hklm.open_subkey_with_flags(uninstall_path, KEY_READ | KEY_WOW64_64KEY)
    {
        get_apps_from_key(&uninstall_key, &mut apps)?;
    }

    // Check 32-bit applications
    if let Ok(uninstall_key) =
        hklm.open_subkey_with_flags(uninstall_path, KEY_READ | KEY_WOW64_32KEY)
    {
        get_apps_from_key(&uninstall_key, &mut apps)?;
    }

    // Per-user installations
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hkcu_uninstall_path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";
    if let Ok(uninstall_key) = hkcu.open_subkey(hkcu_uninstall_path) {
        get_apps_from_key(&uninstall_key, &mut apps)?;
    }

    Ok(apps)
}

fn get_apps_from_key(key: &RegKey, apps: &mut Vec<AppInfo>) -> io::Result<()> {
    for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
        let subkey = match key.open_subkey(&subkey_name) {
            Ok(s) => s,
            Err(_) => continue, // Skip if we can't open the subkey
        };

        // Read DisplayName
        let display_name = match subkey.get_value::<OsString, &str>("DisplayName") {
            Ok(name) => name.to_string_lossy().into_owned(),
            Err(_) => continue, // Skip if no DisplayName
        };

        // Read InstallLocation or UninstallString
        let path = subkey
            .get_value::<OsString, &str>("InstallLocation")
            .or_else(|_| subkey.get_value::<OsString, &str>("UninstallString"))
            .ok()
            .map(PathBuf::from);

        // Read DisplayIcon for the icon path
        let icon_path = subkey
            .get_value::<OsString, &str>("DisplayIcon")
            .ok()
            .map(PathBuf::from);

        apps.push(AppInfo {
            name: display_name,
            path,
            icon_path,
        });
    }
    Ok(())
}

fn load_icon(path: &PathBuf) -> Option<HICON> {
    let wide_path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    unsafe {
        let icon = ExtractIconW(std::ptr::null_mut(), wide_path.as_ptr(), 0);
        if !icon.is_null() {
            Some(icon)
        } else {
            None
        }
    }
}

fn save_icon_to_disk(icon: HICON, output_path: &PathBuf) -> io::Result<()> {
    unsafe {
        let mut icon_info: ICONINFO = std::mem::zeroed();
        if GetIconInfo(icon, &mut icon_info) == 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get icon info",
            ));
        }

        let mut bitmap: BITMAP = std::mem::zeroed();
        if winapi::um::wingdi::GetObjectW(
            icon_info.hbmColor as *mut _,
            std::mem::size_of::<BITMAP>() as _,
            &mut bitmap as *mut _ as *mut _,
        ) == 0
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get bitmap info",
            ));
        }

        let width = bitmap.bmWidth;
        let height = bitmap.bmHeight;
        let mut pixels = vec![0u8; (width * height * 4) as usize];

        let bmi = BITMAPINFO {
            bmiHeader: winapi::um::wingdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as _,
                biWidth: width,
                biHeight: -height, // Negative height to ensure top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [std::mem::zeroed(); 1],
        };

        let hdc = winapi::um::winuser::GetDC(std::ptr::null_mut());
        if GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height as _,
            pixels.as_mut_ptr() as *mut _,
            &bmi as *const _ as *mut _,
            DIB_RGB_COLORS,
        ) == 0
        {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get DIBits"));
        }

        // Convert BGRA to RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // Swap R and B channels
        }

        // Save as PNG using the `image` crate
        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, pixels)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create image buffer"))?;
        img.save(output_path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Clean up
        winapi::um::winuser::ReleaseDC(std::ptr::null_mut(), hdc);
        DestroyIcon(icon);
    }
    Ok(())
}

fn sanitize_path(path: &str) -> String {
    // Remove quotes and any command line arguments
    path.trim_matches('"')
        .split_whitespace()
        .next()
        .unwrap_or(path)
        .to_string()
}

fn sanitize_filename(name: &str) -> String {
    // Replace invalid filename characters with underscores
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '_' })
        .collect()
}

fn main() -> io::Result<()> {
    // Create images directory if it doesn't exist
    std::fs::create_dir_all("./images")?;

    let apps = get_installed_apps()?;
    println!("Installed Applications:");
    for (i, app) in apps.iter().enumerate() {
        println!("{:3}. - {}: {:?}", i + 1, app.name, app.path);

        // Try to get icon from different sources
        if let Some(icon_path) = &app.icon_path {
            let sanitized_path = sanitize_path(&icon_path.to_string_lossy());
            let path = PathBuf::from(&sanitized_path);
            
            // Try loading the icon
            if let Some(icon) = load_icon(&path) {
                let safe_filename = sanitize_filename(&app.name);
                let output_path = PathBuf::from(format!("./images/{}.png", safe_filename));
                
                if let Err(e) = save_icon_to_disk(icon, &output_path) {
                    println!("Failed to save icon for {}: {}", app.name, e);
                    
                    // Try alternate path if available (exe file)
                    if let Some(install_path) = &app.path {
                        let exe_path = sanitize_path(&install_path.to_string_lossy());
                        if let Some(alt_icon) = load_icon(&PathBuf::from(exe_path)) {
                            if save_icon_to_disk(alt_icon, &output_path).is_ok() {
                                println!("Successfully saved icon from alternate path for {}", app.name);
                                continue;
                            }
                        }
                    }
                } else {
                    println!("Saved icon for {} to {:?}", app.name, output_path);
                }
            } else {
                // Try alternate path if icon loading failed
                if let Some(install_path) = &app.path {
                    let exe_path = sanitize_path(&install_path.to_string_lossy());
                    if let Some(alt_icon) = load_icon(&PathBuf::from(exe_path)) {
                        let safe_filename = sanitize_filename(&app.name);
                        let output_path = PathBuf::from(format!("./images/{}.png", safe_filename));
                        if save_icon_to_disk(alt_icon, &output_path).is_ok() {
                            println!("Successfully saved icon from alternate path for {}", app.name);
                            continue;
                        }
                    }
                }
                println!("Failed to load icon for {}", app.name);
            }
        } else if let Some(install_path) = &app.path {
            // Try to get icon from the installation path if no icon path is available
            let exe_path = sanitize_path(&install_path.to_string_lossy());
            if let Some(icon) = load_icon(&PathBuf::from(exe_path)) {
                let safe_filename = sanitize_filename(&app.name);
                let output_path = PathBuf::from(format!("./images/{}.png", safe_filename));
                if save_icon_to_disk(icon, &output_path).is_ok() {
                    println!("Successfully saved icon from installation path for {}", app.name);
                    continue;
                }
            }
            println!("No icon path found for {}", app.name);
        } else {
            println!("No icon path found for {}", app.name);
        }
    }
    Ok(())
}
