import Foundation
import Cocoa


// let workspace = NSWorkspace.shared

// let fileManager = FileManager.default
// do {
//     let appContents = try fileManager.contentsOfDirectory(atPath: "/Applications")
//     // print(appContents)
//     // appContents.forEach { app in
//     //     print(app)
//     // }
//     let apps = appContents.filter { $0.hasSuffix(".app") }
//     for app in apps {
//         print(app)
//     }
// } catch {
//     print(error)
// }
func getApplicationUrlsAt(_ url: URL) -> [URL] {
    let fileManager = FileManager()
    do {
        if !url.path.contains(".app") && url.hasDirectoryPath {
        var urls = try fileManager.contentsOfDirectory(
            at: url,
            includingPropertiesForKeys: [],
            options: [
            FileManager.DirectoryEnumerationOptions.skipsPackageDescendants,
            ]
        )

        urls.forEach {
            if !$0.path.contains(".app") && $0.hasDirectoryPath {
            let subUrls = getApplicationUrlsAt($0)

            urls.append(contentsOf: subUrls)
            }
        }

        return urls
        } else {
        return [url]
        }
    } catch {
        return []
    }
    }

let runningApps = NSWorkspace.shared.runningApplications
let localApplicationUrl = try FileManager.default.url(
    for: .applicationDirectory,
    in: .localDomainMask,
    appropriateFor: nil,
    create: false
)
let localApplicationUrls = getApplicationUrlsAt(localApplicationUrl)
let systemApplicationUrl = try FileManager.default.url(
    for: .applicationDirectory,
    in: .systemDomainMask,
    appropriateFor: nil,
    create: false
)
let systemApplicationsUrls = getApplicationUrlsAt(systemApplicationUrl)
let userApplicationUrl = try FileManager.default.url(
    for: .applicationDirectory,
    in: .userDomainMask,
    appropriateFor: nil,
    create: false
)
// print("userApplicationUrl", userApplicationUrl)
let personalApplicationUrls = getApplicationUrlsAt(userApplicationUrl)
// print(personalApplicationUrls)
var fixedApps: [URL] = [
    URL(fileURLWithPath: "/System/Library/CoreServices/Finder.app"),
]
let allApplicationUrls =
        localApplicationUrls + systemApplicationsUrls +
        personalApplicationUrls +
        fixedApps
// print(allApplicationUrls)

var applications = [Application]()

for url in allApplicationUrls {
    let resourceKeys: [URLResourceKey] = [
        .isExecutableKey,
        .isApplicationKey,
    ]
    let resourceValues = try url.resourceValues(forKeys: Set(resourceKeys))
    if resourceValues.isExecutable ?? false {
        let name = url.deletingPathExtension().lastPathComponent
        let urlStr = url.absoluteString
        let isRunning = runningApps.first(where: {
        $0.bundleURL?.absoluteString == urlStr
        })

        applications.append(Application(
        name: name,
        url: urlStr,
        isRunning: isRunning != nil
        ))
    }
}
// print(applications)
// write applications to a file
let encoder = JSONEncoder()
encoder.outputFormatting = .prettyPrinted
let data = try encoder.encode(applications)
let fileUrl = URL(fileURLWithPath: "applications.json")
try data.write(to: fileUrl)
print("Applications written to \(fileUrl.path)")

extension Application: Encodable {
    enum CodingKeys: String, CodingKey {
        case name
        case url
        case isRunning
    }
}
struct Application {
  var name: String
  var url: String
  var isRunning: Bool
}













