# Changelog

All notable changes to the U programming language will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-17

### 🎉 Initial Release

This is the first stable release of the U programming language, marking a significant milestone in the development of a modern, safe, and lightweight systems programming language with native GUI capabilities.

### Added

#### Core Language Features
- **Complete Compiler Pipeline:** Lexer, parser, type checker, and C code generator fully functional
- **Type System:** Strong static typing with type inference
- **Memory Safety:** Explicit `unsafe` blocks for low-level operations
- **Functions:** First-class functions with return types and parameters
- **Control Flow:** `if`, `while`, `for`, `match` statements
- **Variables:** Mutable (`var`) and immutable (`let`) bindings
- **Operators:** Arithmetic, logical, comparison, and bitwise operators
- **Comments:** Single-line (`//`) and multi-line (`/* */`) comments

#### FFI (Foreign Function Interface)
- **C Interoperability:** Full support for calling C functions via `extern "C"` blocks
- **Variadic Functions:** Support for C variadic functions (e.g., `printf`)
- **Pointer Types:** `ptr` type for raw pointer manipulation
- **Type Mapping:** Automatic mapping between U types and C types

#### GUI DSL (Domain-Specific Language)
- **Declarative UI Syntax:** Flutter/SwiftUI-inspired syntax for building user interfaces
- **Widget System:** Comprehensive set of built-in widgets
  - `Container`: Layout container with background color
  - `Text`: Text rendering with font size and color
  - `Button`: Interactive button with onClick events
  - `TextField`: Text input with placeholder
  - `Image`: Image display (placeholder in POC)
  - `Row`: Horizontal layout container
  - `Column`: Vertical layout container
  - `ScrollView`: Scrollable content (base implementation)
- **Event Handling:** `onClick`, `onHover`, `onChange` event properties
- **Theming:** Theme system with standardized colors and sizes (`stdlib/theme.ul`)
- **Code Generation:** Automatic translation of UI DSL to Skia rendering calls

#### Runtime Systems
- **Skia Integration:** Graphics rendering via Skia library (POC wrapper included)
- **Event Loop:** Event dispatch system for handling user input (`runtime/event_loop.c`)
- **Layout Engine:** Flexbox-based layout system (`runtime/layout.c`)
  - Row and column layouts
  - Flex grow/shrink/basis properties
  - Gap, padding, and margin support
  - Justify content and align items

#### Mobile Support
- **Android NDK:** Build scripts and integration guide for Android arm64
- **iOS SDK:** Build instructions for iOS arm64 with Xcode
- **Cross-Platform Runtime:** Unified runtime that compiles for desktop and mobile

#### Standard Library
- **Core Functions:** Basic I/O and utility functions
- **Theme Module:** Standardized UI theme (`stdlib/theme.ul`)

#### Developer Tools
- **CLI Compiler:** `ul` command-line tool with `build` subcommand
- **Error Messages:** Clear error reporting with line numbers
- **Code Generation:** Optimized C code output
- **Build Flags:** `--no-link` for C-only output

#### Documentation
- **Language Guide:** Comprehensive documentation (`U_LANG_v1.0_DOCUMENTATION.md`)
- **DSL Specification:** UI DSL syntax reference (`docs/DSL_SPECIFICATION_v0.9.md`)
- **Mobile Architecture:** Integration guide for Android/iOS (`docs/MOBILE_ARCHITECTURE_v0.9.md`)
- **Skia Integration:** Production integration guide (`docs/SKIA_INTEGRATION.md`)
- **Examples:** Multiple example programs demonstrating language features
  - `hello.ul`: Basic hello world
  - `ffi_printf.ul`: FFI demonstration
  - `ui_minimal.ul`: Minimal UI example
  - `widgets_demo.ul`: Widget showcase
  - `todo_app.ul`: Complete application demo

### Technical Details

**Compiler:**
- Written in Rust 2021 edition
- Optimized release builds with LTO
- Zero-cost abstractions
- Comprehensive test suite

**Code Generation:**
- Generates clean, readable C code
- Minimal runtime overhead
- Direct Skia API calls (no abstraction penalty)
- Supports both static and dynamic linking

**Platform Support:**
- Linux x86_64 (primary)
- Android arm64 (via NDK)
- iOS arm64 (via Xcode)
- macOS (untested but should work)

**Dependencies:**
- Rust toolchain (for compiler)
- C compiler (GCC or Clang)
- Skia library (for GUI applications)

### Known Limitations

- **Skia POC:** Current runtime uses a proof-of-concept Skia wrapper; production use requires real Skia integration
- **Event Loop:** Basic event system implemented; full event loop requires platform-specific integration
- **Layout:** Flexbox layout engine is functional but not fully optimized
- **Mobile:** Build scripts provided but require manual integration into native projects
- **Standard Library:** Minimal stdlib; will be expanded in future releases

### Breaking Changes

N/A (initial release)

### Deprecated

N/A (initial release)

### Security

- All pointer operations require explicit `unsafe` blocks
- Type system prevents common memory errors
- No null pointer dereferences in safe code

### Performance

- Compiles to native C code
- Zero-cost abstractions
- Direct Skia API calls
- Optimized release builds

### Contributors

- U Language Team
- Webcien Organization

### License

MIT License - Copyright (c) 2025 Webcien and U contributors

---

## [Unreleased]

### Planned for v1.1.0
- Full Skia integration (replace POC wrapper)
- Platform-specific event loops (X11, Wayland, Cocoa, Win32)
- Expanded standard library
- Package manager
- Language server protocol (LSP) support
- Syntax highlighting for popular editors

### Planned for v2.0.0
- Generics
- Async/await
- Advanced type system features
- Garbage collection option
- WebAssembly target

---

[1.0.0]: https://github.com/webcien/u-lang/releases/tag/v1.0.0
