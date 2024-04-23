import Foundation
import Cocoa

sleep(3)
let workspace = NSWorkspace.shared
let currentApplication = workspace.frontmostApplication
if let appName = currentApplication?.localizedName {
    print("The current application is \"\(appName)\"")
} else {
    print("Could not determine the current application")
}

if let appIcon = currentApplication?.icon {
    let bitmapRep = NSBitmapImageRep(data: appIcon.tiffRepresentation!)
    let pngData = bitmapRep?.representation(using: .png, properties: [:])
    let fileURL = URL(fileURLWithPath: "./appicon.png")
    do {
        try pngData?.write(to: fileURL, options: .atomic)
        print("Image saved successfully.")
    } catch {
        print("Error saving image: \(error)")
    }
} else {
    print("No icon found for the current application.")
}

