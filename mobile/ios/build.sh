#!/bin/bash
# U Language - iOS SDK Build Script
# MIT License — Copyright (c) 2025 Webcien and U contributors

set -e

echo "=== U Language iOS SDK Build ==="

# Configuration
XCODE_PATH="/Applications/Xcode.app"
SDK="iphoneos"
ARCH="arm64"
MIN_IOS_VERSION="13.0"
OUTPUT_DIR="build/ios"

# Check Xcode
if [ ! -d "$XCODE_PATH" ]; then
    echo "WARNING: Xcode not found at $XCODE_PATH"
    echo "This script requires macOS with Xcode installed"
    echo "Generating build instructions instead..."
    
    cat > "$OUTPUT_DIR/BUILD_INSTRUCTIONS.md" << 'EOF'
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
EOF

    echo "✓ Build instructions generated: $OUTPUT_DIR/BUILD_INSTRUCTIONS.md"
    exit 0
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "Using Xcode: $XCODE_PATH"
echo "SDK: $SDK"
echo "Architecture: $ARCH"
echo "Min iOS Version: $MIN_IOS_VERSION"

# Compile U runtime for iOS
echo "Compiling U runtime..."
xcrun -sdk $SDK clang \
    -c ../../runtime/skia_glue.c \
    -o "$OUTPUT_DIR/skia_glue.o" \
    -arch $ARCH \
    -mios-version-min=$MIN_IOS_VERSION \
    -O2

echo "Creating static library..."
libtool -static \
    -o "$OUTPUT_DIR/libu_runtime.a" \
    "$OUTPUT_DIR/skia_glue.o"

echo "✓ iOS build complete: $OUTPUT_DIR/libu_runtime.a"
echo ""
echo "INTEGRATION STEPS:"
echo "1. Add libu_runtime.a to your Xcode project"
echo "2. Link against Skia framework"
echo "3. Initialize U runtime in AppDelegate"
