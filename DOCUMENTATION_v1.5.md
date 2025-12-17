_to_write="""# U Language v1.5 Documentation

**Date:** December 17, 2025
**Version:** 1.5.0

---

## Overview

U Language v1.5 introduces a powerful **declarative macro system** (macros-by-example), similar to Rust's `macro_rules!`.

---

## 1. Macro System

**Module:** `compiler/src/macro_expander.rs`

Macros allow you to write code that writes code, enabling powerful abstractions and reducing boilerplate.

### Defining Macros

Macros are defined with the `macro` keyword.

```u
macro my_macro {
    (rule_pattern) => (expansion)
}
```

### Built-in Macros

U v1.5 includes several built-in macros:

#### `println!`

Prints to standard output with a newline.

```u
println!(\"Hello, U Language!\");
println!(\"Value: {}\", 42);
```

#### `assert!`

Asserts that a boolean expression is true. Panics if it is false.

```u
let x = 10;
assert!(x == 10);
```

#### `vec!`

Creates a `Vec<T>` with a list of elements.

```u
let v = vec![1, 2, 3];
```

### Macro Hygiene

A basic hygiene system prevents macros from accidentally capturing or shadowing variables.

---

## 2. Compile-time Execution

(Planned for v1.6)

---

## 3. Advanced Pattern Matching

(Planned for v1.6)

---

## License

U Language is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**
"""
