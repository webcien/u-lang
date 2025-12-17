_to_write="""# U Language v1.3 Architecture Document_EN-US.md

**Date:** December 17, 2025
**Version:** 1.3.0
**Authors:** Manus AI, Webcien

---

## 1. Overview

This document outlines the architecture for U Language v1.3, which focuses on three critical areas:

1.  **Completing the Ownership System:** Finalizing the memory safety model.
2.  **Completing the Actor System:** Integrating the actor runtime with the event loop for concurrent GUI applications.
3.  **Implementing a Package Manager:** Creating a modern dependency management system to foster an ecosystem.

---

## 2. Ownership System Enhancements

### 2.1. `.clone()` Method

**Objective:** Provide a mechanism for explicit deep copies of owned values.

**Implementation:**
1.  **Compiler:**
    -   The parser will recognize `.clone()` as a special method call on a variable.
    -   The `ownership_checker` will validate that `.clone()` is only called on types that implement the `Clone` trait.
    -   The `codegen` will generate a call to a type-specific `_clone` function (e.g., `String_clone`, `Vec_clone`).
2.  **Standard Library:**
    -   A `Clone` trait will be defined in `std/core.ul`.
    -   All relevant standard library types (`String`, `Vec`, `HashMap`) will implement `Clone`.
    -   The implementation will perform a deep copy of the underlying data.

**Example:**
```u
let s1 = "hello";
let s2 = s1.clone(); // s2 is a new, independent string
```

### 2.2. Immutable Reference Validation

**Objective:** Allow multiple immutable references to an owned value within a lexical scope, preventing use-after-free.

**Implementation:**
1.  **Compiler (`ownership_checker.rs`):**
    -   Introduce a "borrow checker" concept.
    -   Track immutable borrows of a variable within its scope.
    -   Any attempt to mutate the original value or a borrowed reference will result in a compile-time error.
    -   Any attempt to move the original value while it is borrowed will result in a compile-time error.
    -   Borrows are automatically released at the end of their lexical scope.

**Example:**
```u
let v = Vec_new<i32>();
let r1 = &v; // Immutable borrow
let r2 = &v; // OK: Multiple immutable borrows allowed

// ERROR: Cannot mutate `v` because it is borrowed
// v = Vec_push(v, 10);

// Borrows r1 and r2 are released here
```

---

## 3. Actor System Integration

**Objective:** Enable concurrent GUI applications by integrating the actor runtime with the main event loop.

**Architecture:**

```
+--------------------+
|   Main Thread      |
| (SDL2 Event Loop)  |
+--------------------+
| 1. Polls for events|
|    (mouse, key)    |
| 2. Dispatches UI   |
|    events to       |
|    event handlers  |
| 3. Ticks the actor |
|    scheduler       |
| 4. Renders frame   |
+--------------------+
         |
         | 3. actor_scheduler_tick()
         v
+--------------------+
|   Actor Runtime    |
|  (Separate Module) |
+--------------------+
| 1. Manages actor   |
|    mailboxes       |
| 2. Executes actor  |
|    messages (co-op)|
| 3. Posts UI update |
|    events to main  |
|    event loop      |
+--------------------+
```

**Implementation:**
1.  **Event Loop (`event_loop_sdl2.c`):**
    -   In the main `while` loop, after processing SDL events, call a new function `actor_scheduler_tick()`.
    -   Implement a custom SDL event type (`U_EVENT_UPDATE_UI`) that the actor runtime can use to request a re-render.
2.  **Actor Runtime (`actor_runtime.c`):**
    -   The `actor_scheduler_tick()` function will process a limited number of messages from actor mailboxes to avoid blocking the main thread.
    -   When an actor
