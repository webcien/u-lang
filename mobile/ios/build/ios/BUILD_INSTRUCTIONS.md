# U Language iOS Build Instructions

## Prerequisites
- macOS with Xcode 14+
- iOS SDK 13.0+
- Skia library for iOS

## Build Steps

1. **Compile U Runtime for iOS:**
   ```bash
   xcrun -sdk iphoneos clang \
       -c ../../runtime/skia_glue.c \
       -o build/ios/skia_glue.o \
       -arch arm64 \
       -mios-version-min=13.0 \
       -O2
   ```

2. **Create Static Library:**
   ```bash
   libtool -static \
       -o build/ios/libu_runtime.a \
       build/ios/skia_glue.o
   ```

3. **Integration in Xcode:**
   - Add `libu_runtime.a` to your Xcode project
   - Link against Skia framework
   - Add U runtime initialization in AppDelegate

4. **Example Swift Integration:**
   ```swift
   import UIKit
   
   @main
   class AppDelegate: UIResponder, UIApplicationDelegate {
       func application(_ application: UIApplication, 
                       didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
           // Initialize U runtime
           u_runtime_init()
           return true
       }
   }
   ```

## Framework Structure
```
YourApp.app/
├── Frameworks/
│   ├── libskia.framework
│   └── libu_runtime.a
└── YourApp
```

## Code Signing
Remember to sign all frameworks and libraries with your Apple Developer certificate.
