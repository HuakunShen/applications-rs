import Foundation
import Cocoa


// Specify the path to the .app folder
// let appPath = "/Applications/Visual Studio Code.app"
// let appPath = "/Applications/Shadowrocket.app"
// let appPath = "/Users/hacker/Library/HTTPStorages/com.apple.ctcategories.service"
// let appPath = "/Applications/Parallels Desktop.app"
let appPath = "/Users/hacker/Applications/Turing Complete.app"
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


