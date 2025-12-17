# U Language v1.4 Documentation

**Date:** December 17, 2025
**Version:** 1.4.0

---

## Overview

U Language v1.4 is a major release focused on developer experience and completing the type system. It introduces:

1.  **Language Server Protocol (LSP)** for IDE integration
2.  **VS Code Extension** for a first-class editing experience
3.  **Full Generics Support** with monomorphization and trait bounds
4.  **Improved Error Messages** for a better debugging experience

---

## 1. Language Server Protocol (LSP)

**Crate:** `u-lsp`
**Executable:** `u-lsp`

Provides IDE features for any editor that supports LSP.

### Features

- **Autocompletion:** Keywords, variables, functions
- **Hover Information:** Type information on hover
- **Go to Definition:** Navigate to definitions
- **Diagnostics:** Real-time error and warning reporting

### Architecture

The `u-lsp` server communicates with the editor over stdin/stdout, using the compiler as a library to analyze code.

---

## 2. VS Code Extension

**Name:** U Language
**Publisher:** webcien
**ID:** `webcien.u-language`

Provides a rich development experience for U in Visual Studio Code.

### Features

- **Syntax Highlighting:** Full syntax highlighting for `.ul` files
- **IntelliSense:** Powered by the `u-lsp` server
- **Snippets:** For common code patterns
- **Auto-formatting:** Using `ul fmt`

### Installation

Install from the Visual Studio Code Marketplace or by building the `.vsix` file from source.

---

## 3. Full Generics Support

**Module:** `compiler/src/generics.rs`

U now supports full generics for functions, structs, and traits.

### Monomorphization

The compiler generates a specialized version of each generic function or type for each concrete type it is used with.

### Trait Bounds

Generic parameters can be constrained by traits:

```u
// Generic function with a trait bound
fn print_clonable<T: Clone>(value: T) {
    let cloned = value.clone();
    // ...
}
```

### Generic Structs

```u
type Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point<i32> { x: 10, y: 20 };
    let p2 = Point<f64> { x: 1.0, y: 2.0 };
}
```

---

## 4. Improved Error Messages

**Module:** `compiler/src/diagnostics.rs`

The compiler now provides more user-friendly error messages with context and hints.

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

## License

U Language is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**
