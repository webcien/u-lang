_to_write="""# U Language v1.4 Architecture Document

**Date:** December 17, 2025
**Version:** 1.4.0
**Authors:** Manus AI, Webcien

---

## 1. Overview

This document outlines the architecture for U Language v1.4, which focuses on improving the developer experience and completing the type system:

1.  **Language Server Protocol (LSP):** Providing IDE features like autocompletion, go-to-definition, and diagnostics.
2.  **VS Code Extension:** A dedicated extension for U Language development.
3.  **Full Generics Support:** Completing the implementation of generics for functions, structs, and traits.
4.  **Improved Error Messages:** Making compiler errors more user-friendly.

---

## 2. Language Server Protocol (LSP)

**Objective:** Create a language server to provide IDE features.

**Implementation:**
1.  **New Crate:** A new binary crate `u-lsp` will be created within the `compiler` directory.
2.  **Dependencies:**
    -   `lsp-server`: For handling the LSP protocol.
    -   `lsp-types`: For LSP data structures.
    -   The existing `u` compiler library.
3.  **Features:**
    -   `textDocument/didOpen`, `didChange`, `didSave`: To track document state.
    -   `textDocument/completion`: Provide autocompletion for keywords, variables, and functions.
    -   `textDocument/hover`: Show type information on hover.
    -   `textDocument/definition`: Go to definition for variables and functions.
    -   `textDocument/publishDiagnostics`: Show compiler errors and warnings in the editor.
4.  **Communication:** The LSP server will communicate with the client (e.g., VS Code) over stdin/stdout.

**Architecture:**
```
+-----------------+      +-----------------+
|   VS Code       |      |   u-lsp         |
| (LSP Client)    |----->| (LSP Server)    |
+-----------------+      +-----------------+
                         | 1. Receives LSP |
                         |    requests     |
                         | 2. Uses compiler|
                         |    modules to   |
                         |    analyze code |
                         | 3. Sends LSP    |
                         |    responses    |
                         +-----------------+
```

---

## 3. VS Code Extension

**Objective:** Create a VS Code extension for U Language.

**Implementation:**
1.  **New Directory:** A new directory `editors/vscode` will be created.
2.  **Technology:** The extension will be written in TypeScript.
3.  **Features:**
    -   **Language Client:** The extension will start and communicate with the `u-lsp` server.
    -   **Syntax Highlighting:** A TextMate grammar file (`syntaxes/u.tmLanguage.json`) will be created for U syntax.
    -   **Language Configuration:** A `language-configuration.json` file will define comment toggling, bracket matching, etc.
    -   **Snippets:** Code snippets for common constructs (functions, loops, etc.).
4.  **Packaging:** The extension will be packaged into a `.vsix` file for installation.

---

## 4. Full Generics Support

**Objective:** Complete the implementation of generics.

**Implementation:**
1.  **Compiler (`type_checker.rs`):**
    -   **Monomorphization:** The type checker will perform monomorphization, creating a specialized version of each generic function or type for each concrete type it is used with.
    -   **Trait Bounds:** The parser and type checker will be extended to support trait bounds on generic parameters (e.g., `fn<T: Clone>(value: T)`).
2.  **Compiler (`codegen/c.rs`):**
    -   The code generator will generate C code for each monomorphized version of a function or type.
    -   Name mangling will be used to create unique names for each specialized version (e.g., `Vec_push_i32`, `Vec_push_String`).

**Example:**
```u
// Generic function with trait bound
fn print_clonable<T: Clone>(value: T) {
    let cloned = value.clone();
    // ...
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

---

## 5. Improved Error Messages

**Objective:** Make compiler errors more helpful.

**Implementation:**
1.  **Compiler (`diagnostics.rs`):**
    -   A new module `diagnostics.rs` will be created to handle error reporting.
    -   Errors will be represented as structured data, including error code, message, location (line and column), and optional hints.
2.  **Error Formatting:**
    -   The error formatter will generate user-friendly, colorful output similar to Rust's compiler errors, pointing to the exact location of the error in the source code.

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

This specification is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**
"""
