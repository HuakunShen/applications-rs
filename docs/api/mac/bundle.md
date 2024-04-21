# Bundle

A bundle is a directory that contains a special directory structure that allows you to distribute your application in a single file. Bundles are used on macOS to distribute applications, frameworks, plugins, and other types of executables.


Here are some sample code in swift and its equivalent rust. It's possible to get app info like icon path from bundle. The `infoDictionary` has lots of useful information about the app, but getting the content of it is too difficult in rust. I'd rather parse `Info.plist` file into a struct.

```swift
import Foundation
import Cocoa


// Specify the path to the .app folder
// let appPath = "/Applications/Visual Studio Code.app"
// let appPath = "/Applications/Shadowrocket.app"
let appPath = "/Users/hacker/Library/HTTPStorages/com.apple.ctcategories.service"

// Create a URL object from the path
let appURL = URL(fileURLWithPath: appPath)
// convert url to string
let appURLString = appURL.absoluteString
// Initialize the Bundle with the URL
if let appBundle = Bundle(url: appURL) {
    print("Bundle was successfully created for: \(appPath)")
    // You can now access bundle properties
    print("Bundle Identifier: \(appBundle.bundleIdentifier ?? "Unknown")")
    print("Executable Path: \(appBundle.executablePath ?? "Unknown")")
    print("Bundle Path: \(appBundle.bundlePath ?? "Unknown")")
    print("description: \(appBundle.description ?? "Unknown")")
    print("resourcePath: \(appBundle.resourcePath ?? "Unknown")")
    print("resourcePath: \(appBundle.className ?? "Unknown")")
    // appBundle.infoDictionary?.forEach { key, value in
    //     print("\(key): \(value)")
    // }
    // convert inforDictionary to json
    let jsonData = try JSONSerialization.data(withJSONObject: appBundle.infoDictionary ?? [:], options: .prettyPrinted)
    let jsonString = String(data: jsonData, encoding: .utf8)
    print("infoDictionary: \(jsonString ?? "Unknown")")
} else {
    print("Failed to create Bundle for: \(appPath)")
}
```

```rust
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
```
