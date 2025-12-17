# U Language

**Modern, Safe, and Lightweight Systems Programming Language**

[![Version](https://img.shields.io/badge/version-1.3.0-blue.svg)](https://github.com/webcien/u-lang/releases/tag/v1.3.0)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/webcien/u-lang)

---

## 🚀 What is U?

U is a **systems programming language** that combines the best features of modern languages while maintaining simplicity and performance. It offers:

- **Memory Safety** through a complete ownership system (like Rust, but simpler)
- **Native GUI** with a declarative DSL (like Flutter/SwiftUI)
- **Actor-based Concurrency** without data races (like Erlang/Pony)
- **Modern Package Manager** for ecosystem growth (like Cargo)
- **Cross-Platform** compilation to Linux, Windows, macOS, and WebAssembly

U compiles to **clean C code** and uses **Zig** as a cross-compilation backend, resulting in:
- ⚡ **Zero runtime overhead**
- 🎯 **Predictable performance**
- 📦 **Tiny binaries** (no runtime, no GC)
- 🌍 **True cross-platform** support

---

## ✨ Key Features

### 1. Complete Ownership System

U implements a **7-rule ownership system** that guarantees memory safety without garbage collection:

```u
// Explicit deep copies with .clone()
let s1 = "hello";
let s2 = s1.clone(); // s2 is independent

// Multiple immutable references
let v = Vec_new<i32>();
let r1 = &v; // Immutable borrow
let r2 = &v; // OK: Multiple immutable borrows allowed

// Compile-time error on use-after-move
let x = Vec_new<i32>();
let y = x;  // x is moved
// let z = x;  // ERROR: use of moved value
```

**No explicit lifetimes. No borrow checker complexity. Just simple, safe code.**

### 2. Declarative GUI DSL

Build native user interfaces with a clean, declarative syntax:

```u
ui my_app {
    Container {
        width: 600,
        height: 400,
        background: rgb(230, 240, 255),
        child: Column {
            children: [
                Text { text: "Hello, U Language!", size: 24 },
                Button { 
                    text: "Click Me", 
                    onClick: handle_click 
                },
                TextField { placeholder: "Enter text..." }
            ]
        }
    }
}
```

**Compiles to Skia rendering calls. Runs on desktop and mobile.**

### 3. Actor-Based Concurrency

Safe, scalable concurrency without locks or data races:

```u
actor Counter {
    let mut count: i32 = 0;
    
    fn increment() {
        count = count + 1;
    }
    
    fn get_count() -> i32 {
        return count;
    }
}

fn main() {
    let counter = spawn Counter;
    send counter.increment();
    let result = await counter.get_count();
    return 0;
}
```

**Micro-runtime of only 5.3 KB. Zero overhead message passing.**

### 4. Modern Package Manager

Manage dependencies with a Cargo-like package manager:

```bash
# Create a new package
ul init my-package

# Install dependencies
ul install u-std

# Build your project
ul build --release

# Publish to registry
ul publish
```

**Package manifest (`ul.toml`):**
```toml
[package]
name = "my-package"
version = "1.0.0"

[dependencies]
u-std = "1.0.0"
u-gui = { version = "1.3.0", features = ["skia"] }
```

### 5. Cross-Platform Compilation

Compile to any platform from any platform using Zig:

```bash
# Compile for Linux
ul build --target x86_64-linux

# Compile for Windows
ul build --target x86_64-windows

# Compile for macOS
ul build --target x86_64-macos

# Compile for WebAssembly
ul build --target wasm32-wasi
```

---

## 📦 Installation

### Prerequisites

- **Rust** (for building the compiler)
- **Zig** (for cross-compilation)
- **Git**

### Build from Source

```bash
git clone https://github.com/webcien/u-lang.git
cd u-lang/compiler
cargo build --release
```

The compiler binary will be at `target/release/ul`.

### Add to PATH

```bash
export PATH="$PATH:/path/to/u-lang/compiler/target/release"
```

---

## 🎯 Quick Start

### Hello World

Create `hello.ul`:

```u
fn main() {
    unsafe {
        printf("Hello, U Language!\n");
    }
    return 0;
}

extern "C" {
    fn printf(format: ptr, ...);
}
```

Compile and run:

```bash
ul build hello.ul
./hello
```

### GUI Application

Create `gui_app.ul`:

```u
ui my_window {
    Container {
        width: 400,
        height: 300,
        background: rgb(255, 255, 255),
        child: Text {
            text: "Hello, GUI!",
            size: 24,
            color: rgb(0, 0, 0)
        }
    }
}

fn main() {
    unsafe {
        skia_init();
        let surface = skia_create_surface(400, 300);
        let canvas = skia_get_canvas(surface);
        render_ui_my_window(canvas);
        skia_save_png(surface, "output.png");
    }
    return 0;
}
```

Compile:

```bash
ul build gui_app.ul
```

---

## 📚 Documentation

- **[Language Guide](docs/)** - Complete language reference
- **[Standard Library](stdlib/)** - API documentation
- **[ul.toml Specification](docs/UL_TOML_SPEC.md)** - Package manifest format
- **[Examples](examples/)** - Sample programs

---

## 🏗️ Project Structure

```
u-lang/
├── compiler/          # U Language compiler (Rust)
│   └── src/
│       ├── lexer.rs           # Tokenization
│       ├── parser.rs          # AST generation
│       ├── type_checker.rs    # Type validation
│       ├── ownership_checker.rs  # Ownership validation
│       ├── concurrency_checker.rs # Concurrency validation
│       ├── optimizer.rs       # Code optimization
│       ├── package_manager.rs # Package management
│       └── codegen/
│           └── c.rs           # C code generation
├── runtime/           # Runtime libraries (C)
│   ├── actor_runtime.c     # Actor system (5.3 KB)
│   ├── event_loop_sdl2.c   # Event loop
│   ├── layout.c            # Flexbox layout
│   └── skia_real.c         # Skia integration
├── stdlib/            # Standard library (U)
│   ├── clone.ul       # Clone trait
│   ├── option.ul      # Option<T>
│   ├── result.ul      # Result<T, E>
│   ├── vec.ul         # Vec<T>
│   └── hashmap.ul     # HashMap<K, V>
├── examples/          # Example programs
├── docs/              # Documentation
└── tests/             # Test suite
```

---

## 🛠️ Tooling

### Compiler Commands

| Command | Description |
|:---|:---|
| `ul build <file>` | Compile a U source file |
| `ul build --release` | Compile with optimizations |
| `ul build --target <triple>` | Cross-compile to target platform |
| `ul fmt <file>` | Format source code |
| `ul lint <file>` | Lint source code |

### Package Manager Commands

| Command | Description |
|:---|:---|
| `ul init <name>` | Create a new package |
| `ul install <package>` | Install a dependency |
| `ul publish` | Publish package to registry |
| `ul update` | Update dependencies |

---

## 🎨 Standard Library

U provides a modern standard library with common data structures:

| Type | Description | File |
|:---|:---|:---|
| `Option<T>` | Optional value | `stdlib/option.ul` |
| `Result<T, E>` | Error handling | `stdlib/result.ul` |
| `Vec<T>` | Dynamic array | `stdlib/vec.ul` |
| `HashMap<K, V>` | Hash map | `stdlib/hashmap.ul` |
| `Clone` | Deep copy trait | `stdlib/clone.ul` |

---

## 🌟 Why U?

### vs. Rust
- ✅ **Simpler ownership** (no explicit lifetimes)
- ✅ **Native GUI DSL** (no external frameworks)
- ✅ **Smaller learning curve**
- ❌ No borrow checker complexity

### vs. Go
- ✅ **No garbage collection** (predictable performance)
- ✅ **Memory safety** (ownership system)
- ✅ **Native GUI** (no web-based UI)
- ❌ Smaller ecosystem (for now)

### vs. Zig
- ✅ **Memory safety** (ownership system)
- ✅ **Actor concurrency** (safe by default)
- ✅ **GUI DSL** (declarative UI)
- ❌ Uses Zig as backend (dependency)

### vs. C/C++
- ✅ **Memory safety** (no segfaults, no use-after-free)
- ✅ **Modern syntax** (type inference, traits)
- ✅ **Package manager** (dependency management)
- ✅ **Same performance** (compiles to C)

---

## 📈 Roadmap

### v1.4 (Q2 2026)
- Language Server Protocol (LSP)
- VS Code extension
- Full generics support
- Improved error messages

### v2.0 (Q4 2026)
- Async/await over actors
- LLVM backend (optional)
- Garbage collection (optional)
- WebAssembly improvements

---

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/webcien/u-lang.git
cd u-lang/compiler
cargo build
cargo test
```

---

## 📜 License

U Language is licensed under the **MIT License**.

See [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

U Language draws inspiration from:
- **Rust** - Ownership system and safety
- **Go** - Simplicity and tooling
- **Zig** - Cross-compilation and C interop
- **Pony** - Actor-based concurrency
- **Flutter/SwiftUI** - Declarative UI

Special thanks to the open-source community.

---

## 📞 Contact

- **Repository:** https://github.com/webcien/u-lang
- **Issues:** https://github.com/webcien/u-lang/issues
- **Discussions:** https://github.com/webcien/u-lang/discussions

---

## 🚀 Get Started

```bash
# Clone the repository
git clone https://github.com/webcien/u-lang.git

# Build the compiler
cd u-lang/compiler
cargo build --release

# Try an example
cd ../examples
../compiler/target/release/ul build hello.ul
./hello
```

**Welcome to U Language! 🎉**

---

**Copyright © 2025 Webcien and U contributors**
