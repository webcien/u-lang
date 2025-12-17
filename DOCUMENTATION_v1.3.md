_to_write="""# U Language v1.3 Documentation

**Date:** December 17, 2025
**Version:** 1.3.0

---

## Overview

U Language v1.3 is a major release that completes the ownership system, integrates the actor runtime with the event loop, and introduces a full-featured package manager. This version solidifies U as a production-ready systems programming language with a modern toolchain.

---

## 1. Ownership System (Complete)

### 1.1. `.clone()` Method

**Trait:** `Clone`
**File:** `stdlib/clone.ul`

Provides a mechanism for explicit deep copies of owned values.

**Example:**
```u
let s1 = "hello";
let s2 = s1.clone(); // s2 is a new, independent string
```

### 1.2. Immutable References

Allows multiple immutable references to an owned value within a lexical scope.

**Example:**
```u
let v = Vec_new<i32>();
let r1 = &v; // Immutable borrow
let r2 = &v; // OK

// ERROR: Cannot mutate `v` because it is borrowed
// v = Vec_push(v, 10);
```

---

## 2. Actor System (Complete)

### 2.1. Integration with Event Loop

**File:** `runtime/event_loop_sdl2.c`

The actor runtime is now integrated with the main event loop, allowing for concurrent GUI applications.

**Architecture:**
- The main thread (SDL2 event loop) ticks the actor scheduler.
- Actors can post UI update events to the main thread.

### 2.2. Actor Scheduler

**File:** `runtime/actor_runtime.c`

- Cooperative scheduling: processes a limited number of messages per tick to avoid blocking the main thread.
- Mailbox FIFO: guarantees message order.

---

## 3. Package Manager

### 3.1. `ul.toml` Manifest

**File:** `docs/UL_TOML_SPEC.md`

Defines package metadata, dependencies, and build configuration.

**Example:**
```toml
[package]
name = "my-package"
version = "1.0.0"

[dependencies]
u-std = "1.0.0"
```

### 3.2. Commands

**Module:** `compiler/src/package_manager.rs`

| Command | Description |
|:---|:---|
| `ul init` | Create a new package |
| `ul build` | Build the package |
| `ul install` | Install dependencies |
| `ul publish` | Publish to registry |
| `ul update` | Update dependencies |

### 3.3. Dependency Resolution

- Resolves dependency graph to ensure compatible versions.
- Supports simple, detailed, path, and Git dependencies.

### 3.4. Registry

- Default registry: **https://packages.u-lang.dev** (placeholder)

---

## 4. Standard Library

- **`stdlib/clone.ul`:** `Clone` trait
- **`stdlib/option.ul`:** `Option<T>`
- **`stdlib/result.ul`:** `Result<T, E>`
- **`stdlib/vec.ul`:** `Vec<T>`
- **`stdlib/hashmap.ul`:** `HashMap<K, V>`

---

## 5. Tooling

- **Compiler:** `ul`
- **Build system:** `ul build`
- **Package manager:** `ul install`, `ul publish`, etc.

---

## 6. Roadmap

- **v1.4 (Q2 2026):** LSP, VS Code extension, full generics
- **v2.0 (Q4 2026):** Async/await, LLVM backend

---

## License

U Language is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**
""
