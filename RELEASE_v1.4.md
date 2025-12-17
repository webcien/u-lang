# U Language v1.4.0 Release Notes

**Date:** December 17, 2025  
**Version:** 1.4.0  
**Repository:** https://github.com/webcien/u-lang  
**Tag:** v1.4.0

---

## 🎉 Overview

U Language v1.4.0 is a major release focused on **developer experience** and **completing the type system**. This version introduces professional IDE integration, full generics support, and significantly improved error messages.

---

## ✅ What's New

### 1. Language Server Protocol (LSP)

**Crate:** `lsp/u-lsp`  
**Executable:** `u-lsp`

A complete Language Server Protocol implementation that provides IDE features for any LSP-compatible editor.

**Features:**
- ✅ **Autocompletion** - Keywords, variables, functions
- ✅ **Hover Information** - Type information on hover
- ✅ **Go to Definition** - Navigate to function and type definitions
- ✅ **Real-time Diagnostics** - Errors and warnings as you type

**Installation:**
```bash
cd lsp
cargo build --release
# Binary at: lsp/target/release/u-lsp
```

### 2. VS Code Extension

**Name:** U Language  
**Publisher:** webcien  
**Version:** 1.4.0

A first-class Visual Studio Code extension for U Language development.

**Features:**
- ✅ **Syntax Highlighting** - Full TextMate grammar for `.ul` files
- ✅ **IntelliSense** - Powered by `u-lsp`
- ✅ **Language Configuration** - Auto-closing brackets, comment toggling
- ✅ **Snippets** - Common code patterns

**Installation:**
```bash
cd editors/vscode
npm install
npm run compile
npm run package
code --install-extension u-language-1.4.0.vsix
```

**Files:**
- `package.json` - Extension manifest
- `syntaxes/u.tmLanguage.json` - Syntax highlighting grammar
- `language-configuration.json` - Language configuration
- `src/extension.ts` - Extension entry point

### 3. Full Generics Support

**Module:** `compiler/src/generics.rs`

U now supports full generics for functions, structs, and traits with **monomorphization** at compile time.

**Features:**
- ✅ **Generic Functions** - Write functions that work with any type
- ✅ **Generic Structs** - Create reusable data structures
- ✅ **Trait Bounds** - Constrain generic parameters with traits
- ✅ **Monomorphization** - Zero runtime overhead

**Example:**
```u
// Generic function with trait bound
fn print_clonable<T: Clone>(value: T) {
    let cloned = value.clone();
    unsafe { printf("Cloned!\n"); }
}

// Generic struct
type Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point<i32> { x: 10, y: 20 };
    let p2 = Point<f64> { x: 1.0, y: 2.0 };
    print_clonable(p1);
}
```

**Compiler generates specialized versions:**
```c
// Generated C code
void print_clonable_i32(int value) { /* ... */ }
void print_clonable_f64(double value) { /* ... */ }

typedef struct Point_i32 { int x; int y; } Point_i32;
typedef struct Point_f64 { double x; double y; } Point_f64;
```

### 4. Improved Error Messages

**Module:** `compiler/src/diagnostics.rs`

The compiler now provides user-friendly error messages with context and hints, similar to Rust's compiler.

**Example:**
```
error[E001]: use of moved value
 --> src/main.ul:5:12
  |
3 |     let x = Vec_new<i32>();
  |         - value moved here
4 |     let y = x;
  |             - value moved here
5 |     let z = x;
  |             ^ value used here after move
  |
  = help: consider using `.clone()` to create a copy
```

---

## 📦 Files Added

### LSP Server (lsp/)
- `lsp/Cargo.toml` - LSP server configuration
- `lsp/src/main.rs` - LSP server implementation

### VS Code Extension (editors/vscode/)
- `package.json` - Extension manifest
- `tsconfig.json` - TypeScript configuration
- `language-configuration.json` - Language configuration
- `syntaxes/u.tmLanguage.json` - Syntax highlighting grammar
- `src/extension.ts` - Extension entry point
- `README.md` - Extension documentation

### Compiler
- `compiler/src/generics.rs` - Generics implementation

### Documentation
- `ARCHITECTURE_v1.4.md` - v1.4 architecture document
- `DOCUMENTATION_v1.4.md` - v1.4 user documentation

---

## 🔧 Breaking Changes

None. v1.4 is fully backward compatible with v1.3.

---

## 📊 Statistics

| Metric | Value |
|:---|:---:|
| **Compiler Size** | ~150 KB |
| **LSP Server Size** | ~2 MB |
| **VS Code Extension Size** | ~50 KB |
| **Generics Engine** | Monomorphization |
| **Supported Editors** | VS Code, any LSP-compatible |

---

## 🚀 Roadmap

### v1.5 (Q3 2026)
- Macro system
- Compile-time execution
- Advanced pattern matching
- Module system improvements

### v2.0 (Q4 2026)
- Async/await over actors
- LLVM backend (optional)
- Garbage collection (optional)
- WebAssembly improvements

---

## 🙏 Acknowledgments

U Language v1.4 was developed by:
- **Webcien** - Language design and implementation
- **Manus AI** - Development assistance and documentation

Special thanks to the Rust, TypeScript, and LSP communities for inspiration and tooling.

---

## 📜 License

U Language is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**

---

## 🔗 Links

- **Repository:** https://github.com/webcien/u-lang
- **Documentation:** https://github.com/webcien/u-lang/tree/master/docs
- **VS Code Extension:** https://github.com/webcien/u-lang/tree/master/editors/vscode
- **LSP Server:** https://github.com/webcien/u-lang/tree/master/lsp
- **Issues:** https://github.com/webcien/u-lang/issues

---

**Happy coding with U Language v1.4! 🚀**
