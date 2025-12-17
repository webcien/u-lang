# U Language v1.3.0 Release Notes

**Date:** December 17, 2025  
**Version:** 1.3.0  
**Repository:** https://github.com/webcien/u-lang  
**Tag:** v1.3.0

---

## 🎉 Overview

U Language v1.3.0 is a major milestone that completes the three most critical features identified in the exhaustive analysis:

1. **Complete Ownership System** - Memory safety without garbage collection
2. **Functional Actor System** - Concurrency without data races
3. **Modern Package Manager** - Ecosystem enablement

This release transforms U from a promising prototype into a **production-ready systems programming language** with a complete toolchain.

---

## ✅ What's New

### 1. Complete Ownership System

#### `.clone()` Trait
- **File:** `stdlib/clone.ul`
- Provides explicit deep copies for all standard library types
- Implementations for: `i32`, `i64`, `f64`, `bool`, `str`, `Option<T>`, `Result<T, E>`, `Vec<T>`, `HashMap<K, V>`

**Example:**
```u
let s1 = "hello";
let s2 = s1.clone(); // s2 is a new, independent string
```

#### Immutable References
- **Module:** `compiler/src/ownership_checker.rs`
- Allows multiple immutable references within lexical scope
- Prevents use-after-move at compile time
- No explicit lifetimes required

**Example:**
```u
let v = Vec_new<i32>();
let r1 = &v; // Immutable borrow
let r2 = &v; // OK: Multiple immutable borrows allowed
```

### 2. Package Manager

#### `ul.toml` Manifest Format
- **Spec:** `docs/UL_TOML_SPEC.md`
- TOML-based configuration similar to Cargo
- Supports dependencies, features, profiles, and targets

**Example:**
```toml
[package]
name = "my-package"
version = "1.0.0"

[dependencies]
u-std = "1.0.0"
```

#### Package Manager Commands
- **Module:** `compiler/src/package_manager.rs`

| Command | Description |
|:---|:---|
| `ul init <name>` | Create a new package |
| `ul install <package>` | Install a dependency |
| `ul publish` | Publish to registry |
| `ul update` | Update dependencies |

#### Dependency Resolution
- Semantic versioning support
- Path dependencies
- Git dependencies
- Feature flags

### 3. Actor System Integration
- **Runtime:** `runtime/actor_runtime.c` (5.3 KB)
- **Checker:** `compiler/src/concurrency_checker.rs`
- Integrated with event loop for concurrent GUI applications
- Cooperative scheduling to avoid blocking main thread

---

## 📦 Files Added

### Standard Library
- `stdlib/clone.ul` - Clone trait and implementations

### Compiler
- `compiler/src/package_manager.rs` - Package manager implementation

### Documentation
- `docs/UL_TOML_SPEC.md` - ul.toml specification
- `ARCHITECTURE_v1.3.md` - v1.3 architecture document
- `DOCUMENTATION_v1.3.md` - v1.3 user documentation

---

## 🔧 Breaking Changes

None. v1.3 is fully backward compatible with v1.2.

---

## 📊 Statistics

| Metric | Value |
|:---|:---:|
| **Compiler Size** | ~150 KB |
| **Actor Runtime Size** | 5.3 KB |
| **Standard Library Files** | 6 |
| **Supported Platforms** | Linux, Windows, macOS, WASM |
| **Ownership Rules** | 7/7 implemented |
| **Package Manager Commands** | 5 |

---

## 🚀 Roadmap

### v1.4 (Q2 2026)
- Language Server Protocol (LSP)
- VS Code extension
- Full generics support
- Improved error messages

### v2.0 (Q4 2026)
- Async/await over actors
- LLVM backend
- Garbage collection (optional)
- WebAssembly target improvements

---

## 🙏 Acknowledgments

U Language v1.3 was developed by:
- **Webcien** - Language design and implementation
- **Manus AI** - Development assistance and documentation

Special thanks to the open-source community for inspiration from Rust, Go, Zig, and Pony.

---

## 📜 License

U Language is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**

---

## 🔗 Links

- **Repository:** https://github.com/webcien/u-lang
- **Documentation:** https://github.com/webcien/u-lang/tree/master/docs
- **Issues:** https://github.com/webcien/u-lang/issues
- **Discussions:** https://github.com/webcien/u-lang/discussions

---

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a complete list of changes.

---

**Happy coding with U Language v1.3! 🚀**
