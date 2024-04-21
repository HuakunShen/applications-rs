use core_foundation::base::{TCFType, ToVoid};
use core_foundation::bundle::CFBundle;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::CFURL;

fn main() {
    unsafe {
        let app_path = "/Applications/Shadowrocket.app";
        // let app_url = CFURL::from_string(app_path, None);
        let bundle = CFBundle::new(CFURL::from_path(&app_path, true).expect("msg")).unwrap();
        let executable = bundle.executable_url().unwrap().to_path().unwrap();
        println!("executable: {:?}", executable);
        let bundle_url = bundle.bundle_url().unwrap().to_path().unwrap();
        println!("bundle_url: {:?}", bundle_url);
        let bundle_resource_url = bundle.bundle_resources_url().unwrap().to_path().unwrap();
        println!("bundle_resource_url: {:?}", bundle_resource_url);
        let resource_path = bundle.resources_path().unwrap();
        println!("resource_path: {:?}", resource_path);
        let cf_info_dict = bundle.info_dictionary();
        println!("{:?}", cf_info_dict);
        let key = CFString::from_static_string("CFBundleName");
        // let bundle_name = cf_info_dict.get(key);
        let bundle_name = cf_info_dict
            .find(key)
            .and_then(|v| v.downcast::<CFString>())
            .unwrap();
        let bundle_name = bundle_name.to_string();
        println!("{:?}", bundle_name);

        let cf_bundle_icons = cf_info_dict.get(CFString::new("CFBundleIcons"));
        println!("{:?}", cf_bundle_icons);
    }
}
