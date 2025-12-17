#!/bin/bash
# U Language - Android NDK Build Script
# MIT License — Copyright (c) 2025 Webcien and U contributors

set -e

echo "=== U Language Android NDK Build ==="

# Configuration
NDK_PATH="${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/25.2.9519653}"
TARGET="aarch64-linux-android"
API_LEVEL="21"
OUTPUT_DIR="build/android"

# Check NDK
if [ ! -d "$NDK_PATH" ]; then
    echo "ERROR: Android NDK not found at $NDK_PATH"
    echo "Please set ANDROID_NDK_HOME or install NDK"
    exit 1
fi

echo "Using NDK: $NDK_PATH"
echo "Target: $TARGET"
echo "API Level: $API_LEVEL"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Compile U runtime for Android
echo "Compiling U runtime..."
$NDK_PATH/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android${API_LEVEL}-clang \
    -c ../../runtime/skia_glue.c \
    -o "$OUTPUT_DIR/skia_glue.o" \
    -fPIC \
    -O2

echo "Creating static library..."
$NDK_PATH/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar rcs \
    "$OUTPUT_DIR/libu_runtime.a" \
    "$OUTPUT_DIR/skia_glue.o"

echo "✓ Android build complete: $OUTPUT_DIR/libu_runtime.a"
echo ""
echo "INTEGRATION STEPS:"
echo "1. Copy libu_runtime.a to your Android project's jniLibs/arm64-v8a/"
echo "2. Add U runtime initialization in your MainActivity"
echo "3. Link against Skia for Android"
echo ""
echo "Example Android.mk:"
echo "  LOCAL_STATIC_LIBRARIES := u_runtime skia"
echo "  include \$(BUILD_SHARED_LIBRARY)"
