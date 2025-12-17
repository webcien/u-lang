_to_write="""# U Language v1.5 Macro System Architecture

**Date:** December 17, 2025
**Version:** 1.5.0
**Authors:** Manus AI, Webcien

---

## 1. Overview

This document outlines the architecture for the macro system in U Language v1.5. The goal is to implement a simple, yet powerful, declarative macro system (macros-by-example) similar to Rust's `macro_rules!`.

---

## 2. Macro Definition Syntax

Macros will be defined using the `macro` keyword.

```u
macro my_macro {
    // Rules
    (rule_pattern) => (expansion)
}
```

**Example: `println!` macro**

```u
macro println {
    ($fmt:expr) => (printf(concat($fmt, "\n")));
    ($fmt:expr, $($arg:expr),*) => (printf(concat($fmt, "\n"), $($arg),*));
}
```

### Designators

The following designators will be supported in patterns:

- `expr`: An expression
- `stmt`: A statement
- `ident`: An identifier
- `ty`: A type
- `pat`: A pattern
- `item`: An item (function, struct, etc.)
- `block`: A block of code
- `meta`: A meta item (attributes)
- `path`: A path (e.g., `std::io`)
- `tt`: A token tree

### Repetition

Repetition will be handled with `$(...)*` (zero or more) and `$(...)+` (one or more), with an optional separator.

---

## 3. Macro Expansion Process

Macro expansion will occur as a new step in the compilation pipeline, right after parsing.

**New Compilation Pipeline:**
1.  Lexing
2.  Parsing (generates AST with `MacroCall` nodes)
3.  **Macro Expansion** (transforms `MacroCall` nodes into other AST nodes)
4.  Type Checking
5.  Ownership Checking
6.  Optimization
7.  Code Generation

### Implementation Details

1.  **`parser.rs`:**
    -   A new `Statement::MacroCall` and `Expression::MacroCall` will be added to the AST.
    -   The parser will recognize `name!(...)` syntax and create `MacroCall` nodes.

2.  **`macro_expander.rs` (New Module):**
    -   This module will contain the logic for macro expansion.
    -   It will traverse the AST, find `MacroCall` nodes, and replace them with the expanded AST nodes.
    -   It will manage a registry of defined macros.

3.  **Built-in Macros:**
    -   Macros like `println!`, `assert!`, and `vec!` will be pre-registered in the macro expander.

---

## 4. Macro Hygiene

To prevent macros from accidentally capturing or shadowing variables, a basic hygiene system will be implemented.

**Implementation:**
-   Identifiers introduced by a macro expansion will be given a unique suffix based on the macro invocation site.
-   This prevents name clashes with user-defined variables.

**Example:**

```u
let x = 10;
my_macro!(); // If my_macro introduces a variable `x`
```

The `x` inside `my_macro` will be renamed to something like `x_macro1` to avoid conflicting with the user's `x`.

---

## 5. Built-in Macros for v1.5

### `println!`
-   Prints to standard output with a newline.
-   **Example:** `println!(\"Hello, {}!\", name);`

### `assert!`
-   Asserts that a boolean expression is true. Panics if it is false.
-   **Example:** `assert!(x == 10);`

### `vec!`
-   Creates a `Vec<T>` with a list of elements.
-   **Example:** `let v = vec![1, 2, 3];`

---

## 6. Roadmap

-   **v1.5:** Declarative macros (`macro_rules!` style)
-   **v1.6 (Future):** Procedural macros (macros that operate on token streams)

---

## License

This specification is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**
"""
