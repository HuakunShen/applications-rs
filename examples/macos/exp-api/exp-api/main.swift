import Foundation
import Cocoa


// let workspace = NSWorkspace.shared

let allBundles = Bundle.allBundles
let allFrameworks = Bundle.allFrameworks

allBundles.forEach { bundle in
    print(bundle.bundlePath)
}
