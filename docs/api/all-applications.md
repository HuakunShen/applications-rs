# Get All Applications

## Mac

Here is related swift code

https://github.com/ospfranco/sol/blob/f6764a510b092a36b69d9e794c64d368b8daf628/macos/sol-macOS/lib/ApplicationSearcher.swift#L19

```swift
let resourceKeys: [URLResourceKey] = [
    .isExecutableKey,
    .isApplicationKey,
]
let resourceValues = try url.resourceValues(forKeys: Set(resourceKeys))
```
`isExecutableKey` and `isApplicationKey` are `URLResourceKey` used to get the information about a path, but how to do this in Rust?

Here is a GitHub search on all rust code https://github.com/search?q=isApplicationKey+language%3Arust&type=code

`cidre` seems like a good library with lots of rusty apple APIs, see 
https://github.com/yury/cidre/blob/49944af547df347a0c572453b405500c7ef3f932/cidre/src/ns/url.rs#L160

But `cidre` doesn't have many stars on GitHub or downloads on crates.io

Another search result is `core-foundation` crate, see

https://github.com/servo/core-foundation-rs/blob/199badc40ffd926ddec37954571ffec48d5e0f77/core-foundation-sys/src/url.rs#L98

https://docs.rs/core-foundation/0.9.4/core_foundation/?search=isApplicationKey

`isApplicationKey` is defined as `kCFURLIsApplicationKey` as `extern "C"`, but how to use it? I couldn't find any examples on GitHub. 



## Icon

> After getting the application path, how to get the app icon? NSRunningApplication has a property `icon` which returns the icon of the application, but not for other apps.

