# Skia Integration Guide for U Language v1.0

**Status:** Production Ready  
**Date:** 2025-12-17  
**Author:** U Language Team

## Overview

This document provides comprehensive instructions for integrating the real Skia graphics library with the U language compiler and runtime. The current implementation uses a proof-of-concept wrapper (`skia_glue.c`) that simulates Skia functionality. For production use, you must link against the actual Skia library.

## Prerequisites

Before integrating Skia, ensure you have the following:

- **Build Tools:** Python 3, GN, Ninja
- **Compiler:** Clang or GCC with C++17 support
- **Disk Space:** ~10 GB for Skia source and build artifacts
- **Time:** 30-60 minutes for initial Skia build

## Step 1: Download and Build Skia

### Clone Skia Repository

```bash
cd /opt
git clone https://skia.googlesource.com/skia.git
cd skia
python3 tools/git-sync-deps
```

### Configure Build

For **Linux x86_64** (desktop):

```bash
bin/gn gen out/Release --args='
  is_official_build=true
  is_component_build=false
  skia_use_system_libjpeg_turbo=false
  skia_use_system_libpng=false
  skia_use_system_zlib=false
  skia_enable_gpu=true
  skia_use_gl=true
'
```

For **Android arm64**:

```bash
bin/gn gen out/Android --args='
  target_os="android"
  target_cpu="arm64"
  ndk="/path/to/android-ndk"
  is_official_build=true
'
```

For **iOS arm64**:

```bash
bin/gn gen out/iOS --args='
  target_os="ios"
  target_cpu="arm64"
  is_official_build=true
  skia_use_metal=true
'
```

### Build Skia

```bash
ninja -C out/Release
```

This produces `out/Release/libskia.a` (static library) or `out/Release/libskia.so` (shared library).

## Step 2: Update U Runtime

Replace the POC implementation in `runtime/skia_glue.c` with real Skia API calls.

### Include Skia Headers

```c
#include "include/core/SkCanvas.h"
#include "include/core/SkSurface.h"
#include "include/core/SkPaint.h"
#include "include/core/SkImage.h"
#include "include/core/SkData.h"
#include "include/encode/SkPngEncoder.h"
```

### Update Function Implementations

**Example: Surface Creation**

```c
SkSurface* u_skia_surface_create(int width, int height) {
    SkImageInfo info = SkImageInfo::MakeN32Premul(width, height);
    sk_sp<SkSurface> surface = SkSurface::MakeRaster(info);
    return surface.release();
}
```

**Example: Drawing Rectangle**

```c
void u_skia_canvas_draw_rect(SkCanvas* canvas, int left, int top, 
                               int width, int height, SkPaint* paint) {
    SkRect rect = SkRect::MakeXYWH(left, top, width, height);
    canvas->drawRect(rect, *paint);
}
```

**Example: Save PNG**

```c
int u_skia_surface_save_png(SkSurface* surface, const char* path) {
    sk_sp<SkImage> image = surface->makeImageSnapshot();
    sk_sp<SkData> data = SkPngEncoder::Encode(nullptr, image.get(), {});
    
    FILE* f = fopen(path, "wb");
    if (!f) return 0;
    
    fwrite(data->data(), 1, data->size(), f);
    fclose(f);
    return 1;
}
```

## Step 3: Update U Compiler

Modify the compiler to link against Skia when generating binaries.

### Update `compiler/src/main.rs`

Add Skia library paths to the linker invocation:

```rust
let skia_lib_path = "/opt/skia/out/Release";
let skia_include_path = "/opt/skia";

let link_args = format!(
    "-L{} -lskia -lstdc++ -lpthread -lfontconfig -lfreetype",
    skia_lib_path
);
```

### Update Build Script

In `compiler/build.rs`, add:

```rust
println!("cargo:rustc-link-search=/opt/skia/out/Release");
println!("cargo:rustc-link-lib=skia");
```

## Step 4: Compile U Runtime with Skia

```bash
cd runtime
g++ -c skia_glue.c -o skia_glue.o \
    -I/opt/skia \
    -std=c++17 \
    -O2

ar rcs libu_runtime.a skia_glue.o
```

## Step 5: Link U Programs

When compiling a U program:

```bash
ul build my_app.ul

# Internally, the compiler runs:
gcc -o my_app my_app.c \
    runtime/libu_runtime.a \
    /opt/skia/out/Release/libskia.a \
    -lstdc++ -lpthread -lfontconfig -lfreetype -lGL
```

## Step 6: Verify Integration

Create a test program:

```u
extern "C" {
    fn skia_init();
    fn skia_create_surface(width: i32, height: i32) -> ptr;
    fn skia_get_canvas(surface: ptr) -> ptr;
    fn skia_save_png(surface: ptr, filename: ptr);
}

ui test {
    Container {
        width: 200,
        height: 200,
        background: rgb(100, 150, 200)
    }
}

fn main() {
    unsafe {
        skia_init();
        let surface = skia_create_surface(200, 200);
        let canvas = skia_get_canvas(surface);
        render_ui_test(canvas);
        skia_save_png(surface, "test.png");
    }
    return 0;
}
```

Compile and run:

```bash
ul build test.ul
./test
```

Check that `test.png` is created with the correct rendering.

## Platform-Specific Notes

### Linux

- Requires: `libfontconfig-dev`, `libfreetype-dev`, `libgl1-mesa-dev`
- Install: `sudo apt install libfontconfig-dev libfreetype-dev libgl1-mesa-dev`

### macOS

- Use Metal backend for better performance
- Add `-framework CoreFoundation -framework CoreGraphics -framework CoreText`

### Android

- Link against `libskia.a` built for Android
- Include in `jniLibs/arm64-v8a/`
- Update `Android.mk` or `CMakeLists.txt`

### iOS

- Use Metal backend
- Add Skia as a framework in Xcode
- Link against `libskia.framework`

## Troubleshooting

**Issue:** Undefined symbols when linking  
**Solution:** Ensure all Skia dependencies are linked: `-lstdc++ -lpthread -lfontconfig -lfreetype`

**Issue:** Runtime crash in Skia functions  
**Solution:** Check that Skia was built with the same C++ standard library as your compiler

**Issue:** Missing fonts or text not rendering  
**Solution:** Install system fonts or bundle fonts with your application

## Performance Optimization

For production builds:

1. **Enable LTO:** Add `use_lto=true` to GN args
2. **Strip Symbols:** Use `strip` command on final binary
3. **Use GPU Acceleration:** Enable OpenGL or Metal backend
4. **Cache Surfaces:** Reuse SkSurface objects when possible

## Next Steps

- Implement text rendering with `SkFont` and `SkTextBlob`
- Add image loading with `SkImage::MakeFromEncoded`
- Integrate with platform window systems (X11, Wayland, Cocoa, Win32)
- Implement GPU-accelerated rendering with OpenGL/Metal/Vulkan

## References

- [Skia Official Documentation](https://skia.org/docs/)
- [Skia Build Instructions](https://skia.org/docs/user/build/)
- [Skia API Reference](https://api.skia.org/)
