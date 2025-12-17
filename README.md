# U Language v1.2

**U Language** is a modern systems programming language that combines the safety of Rust, the simplicity of Go, and the native GUI capabilities of Swift/Flutter. It is designed to be a productive, performant, and beautiful language for building cross-platform applications.

![U Language Logo](https://i.imgur.com/your-logo.png)  <!-- Replace with actual logo -->

[![Build Status](https://github.com/webcien/u-lang/actions/workflows/test.yml/badge.svg)](https://github.com/webcien/u-lang/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.2.0-blue.svg)](https://github.com/webcien/u-lang/releases/tag/v1.2.0)

---

## Features

### Core Language

- **Ownership System:** Compile-time memory safety without a garbage collector.
- **Actor-based Concurrency:** Safe concurrency without data races.
- **Static Typing:** Strong type system with type inference.
- **FFI:** Seamless interoperability with C.
- **Cross-Compilation:** Build for Linux, Windows, macOS, and WASM from a single machine.

### GUI DSL

- **Declarative Syntax:** Build user interfaces with a simple, declarative syntax inspired by Flutter and SwiftUI.
- **8+ Widgets:** Container, Text, Button, TextField, Image, Row, Column, ScrollView.
- **Event Handling:** `onClick`, `onHover`, `onChange` handlers.
- **Flexbox Layout:** Flexible and responsive layouts with a powerful Flexbox engine.
- **Skia Backend:** High-performance 2D graphics rendering powered by Skia.

### Standard Library

- **`Option<T>`:** Represents an optional value.
- **`Result<T, E>`:** For error handling.
- **`Vec<T>`:** A growable array.
- **`HashMap<K, V>`:** A hash table.

### Tooling

- **`ul build`:** Build U programs.
- **`ul fmt`:** Format U code.
- **`ul lint`:** Lint U code.
- **`ul test`:** Run tests.

---

## Example: Todo App

```u
extern "C" {
    fn printf(format: ptr, ...);
}

fn handle_add_task() {
    unsafe { printf("Task added!\n"); }
    return 0;
}

ui todo_app {
    Container {
        width: 600,
        height: 800,
        background: rgb(245, 245, 250),
        child: Column {
            spacing: 20,
            children: [
                Container {
                    width: 600,
                    height: 80,
                    background: rgb(60, 120, 220),
                    child: Text {
                        content: "Todo App",
                        size: 28,
                        color: rgb(255, 255, 255)
                    }
                },
                TextField {
                    placeholder: "What needs to be done?",
                    width: 500
                },
                Button {
                    text: "Add Task",
                    onClick: handle_add_task,
                    width: 120
                }
            ]
        }
    }
}

fn main() {
    let surface: ptr;  // Owned by main
    let canvas: ptr;   // Owned by main
    
    unsafe {
        surface = skia_create_surface(600, 800);
        canvas = skia_get_canvas(surface);
        render_ui_todo_app(canvas);  // Ownership borrowed
        skia_save_png(surface, "todo_app.png");
    }
    
    return 0;
}
```

**Output:**

![Todo App Screenshot](https://i.imgur.com/your-screenshot.png)  <!-- Replace with actual screenshot -->

---

## Getting Started

### Prerequisites

- Rust (for building the compiler)
- Zig (for cross-compilation)
- SDL2 (for event loop)
- Skia (for rendering)

### Building from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/webcien/u-lang.git
   cd u-lang
   ```

2. **Build the compiler:**
   ```bash
   cd compiler
   cargo build --release
   ```

3. **Run the tests:**
   ```bash
   cd ../tests
   ./run_tests.sh
   ```

4. **Build an example:**
   ```bash
   cd ../examples
   ../compiler/target/release/ul build todo_app.ul
   ```

---

## Roadmap

- **v1.3 (Q2 2026):** Package manager, LSP, VS Code extension
- **v2.0 (Q4 2026):** Generics, async/await, LLVM backend

---

## Contributing

Contributions are welcome! Please see our [contributing guidelines](CONTRIBUTING.md) for more information.

---

## License

U Language is licensed under the [MIT License](LICENSE).
