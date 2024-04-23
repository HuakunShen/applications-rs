import Foundation
import Cocoa


let workspace = NSWorkspace.shared
sleep(3)
workspace.runningApplications.forEach { app in
    print("")
    print(app.localizedName ?? "Unknown")
    print(app.bundleURL ?? "Unknown")
    print(app.bundleIdentifier ?? "Unknown")
    print("exe", app.executableURL ?? "Unknown")

        
}

// let allBundles = Bundle.allBundles
// let allFrameworks = Bundle.allFrameworks
// allBundles.forEach { bundle in
//     print(bundle.bundlePath)
// }