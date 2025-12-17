# U Language v1.5.0 Release Notes

**Date:** December 17, 2025  
**Version:** 1.5.0  
**Repository:** https://github.com/webcien/u-lang  
**Tag:** v1.5.0

---

## 🎉 Overview

U Language v1.5.0 introduces a **declarative macro system** (macros-by-example), enabling powerful code generation and abstraction capabilities.

---

## ✅ What's New

### 1. Declarative Macro System

**Module:** `compiler/src/macro_expander.rs`

A complete macro expansion system similar to Rust's `macro_rules!`.

**Features:**
- ✅ **Macro Registry** - Register and manage macros
- ✅ **Pattern Matching** - Match macro invocations against patterns
- ✅ **Token Expansion** - Expand macros into AST nodes
- ✅ **Hygiene System** - Prevent variable capture and shadowing

**Architecture:**
```
Source Code → Lexer → Parser → Macro Expansion → Type Checking → Code Generation
```

### 2. Built-in Macros

Three essential macros are included:

#### `println!`
Prints to standard output with a newline.

```u
println!("Hello, U Language!");
println!("Value: {}", 42);
```

#### `assert!`
Asserts that a boolean expression is true.

```u
let x = 10;
assert!(x == 10);
```

#### `vec!`
Creates a `Vec<T>` with a list of elements.

```u
let v = vec![1, 2, 3];
```

### 3. Macro Hygiene

A basic hygiene system prevents macros from accidentally capturing or shadowing variables by generating unique identifiers.

---

## 📦 Files Added

### Compiler
- `compiler/src/macro_expander.rs` - Macro expansion engine

### Documentation
- `ARCHITECTURE_v1.5.md` - Macro system architecture
- `DOCUMENTATION_v1.5.md` - User documentation

---

## 🔧 Breaking Changes

None. v1.5 is fully backward compatible with v1.4.

---

## 📊 Statistics

| Metric | Value |
|:---|:---:|
| **Macro Expander Size** | ~200 LOC |
| **Built-in Macros** | 3 |
| **Hygiene** | Basic |
| **Performance** | Zero runtime overhead |

---

## 🚀 Roadmap

### v1.6 (Q4 2026)
- Procedural macros
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

U Language v1.5 was developed by:
- **Webcien** - Language design and implementation
- **Manus AI** - Development assistance and documentation

---

## 📜 License

U Language is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**

---

## 🔗 Links

- **Repository:** https://github.com/webcien/u-lang
- **Documentation:** https://github.com/webcien/u-lang/tree/master/docs
- **Issues:** https://github.com/webcien/u-lang/issues

---

**Happy coding with U Language v1.5! 🚀**
