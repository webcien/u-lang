# U Language - Declarative UI DSL Specification (v0.9)

**Status:** DRAFT
**Version:** 0.1
**Date:** 2025-12-17

## 1. Overview

This document specifies the syntax and semantics for the declarative UI Domain-Specific Language (DSL) in the U programming language. The goal is to provide a developer-friendly, modern syntax for building cross-platform graphical user interfaces, inspired by frameworks like SwiftUI and Flutter.

The DSL is a core part of the language, not a library. The compiler will parse the `ui` blocks and generate low-level Skia rendering calls, leveraging the FFI and Skia bindings developed in Phase 2.

**Core Principles:**
- **Declarative:** Describe *what* the UI should look like, not *how* to draw it.
- **Hierarchical:** Compose complex UIs by nesting simple widgets.
- **Statically Typed:** All widgets and properties are validated at compile time.
- **Performant:** The compiler generates optimized C code that directly calls the Skia wrapper.

## 2. Syntax

The primary entry point for the UI DSL is the `ui` block. This block can only contain widget definitions.

### 2.1. The `ui` Block

A UI definition must be enclosed in a `ui { ... }` block. This block is a top-level statement.

```u
// main.ul

// Main UI definition for the application window
ui main_window {
    // Widget hierarchy starts here
    Container {
        width: 800,
        height: 600,
        background: rgb(250, 250, 250)
    }
}
```

### 2.2. Widget Declaration

Widgets are declared using their type name followed by a block of properties in `{}`.

```
WidgetName {
    property1: value1,
    property2: value2
}
```

- **WidgetName:** The type of the widget (e.g., `Container`, `Text`, `Button`).
- **Properties:** A comma-separated list of `key: value` pairs.
- **Nesting:** Widgets can be nested to create a view hierarchy.

### 2.3. Property Types

Properties can have the following types:

| Type | Example | Description |
|---|---|---|
| `int` | `width: 800` | An integer value. |
| `string` | `text: "Hello, U!"` | A string literal. |
| `Color` | `background: rgb(255, 0, 128)` | A color defined with the `rgb` function. |
| `Function` | `onClick: my_handler_func` | A reference to a U function. |

## 3. Core Widget Library

The initial version of the DSL will include a small but essential set of widgets.

### 3.1. `Container`

A widget used for layout and styling. It can contain a single child widget.

**Properties:**
- `width: int`
- `height: int`
- `background: Color`
- `child: Widget`

**Example:**
```u
Container {
    width: 100,
    height: 100,
    background: rgb(0, 100, 200),
    child: Text { content: "Inside" }
}
```

### 3.2. `Text`

A widget to display a line of text.

**Properties:**
- `content: string`
- `fontSize: int` (default: 14)
- `color: Color` (default: `rgb(0, 0, 0)`)

**Example:**
```u
Text {
    content: "Hello, World!",
    fontSize: 24,
    color: rgb(50, 50, 50)
}
```

### 3.3. `Button`

A simple clickable button.

**Properties:**
- `text: string`
- `width: int`
- `height: int`
- `background: Color`
- `onClick: fn()` (A function pointer to a U function with no arguments)

**Example:**
```u
fn handle_click() {
    core::print("Button clicked!");
}

ui my_ui {
    Button {
        text: "Click Me",
        width: 120,
        height: 40,
        onClick: handle_click
    }
}
```

### 3.4. `Row` & `Column`

Layout widgets for arranging children horizontally or vertically.

**Properties:**
- `children: [Widget]` (A list of child widgets)
- `spacing: int` (Space between children)

**Example:**
```u
Column {
    spacing: 10,
    children: [
        Text { content: "First Item" },
        Text { content: "Second Item" }
    ]
}
```

## 4. Code Generation Strategy

The compiler will traverse the Abstract Syntax Tree (AST) of the `ui` block and generate corresponding C code that calls the `skia_glue.c` functions.

1.  **AST Representation:** New AST nodes will be created for `UiBlock`, `Widget`, and `Property`.
2.  **Semantic Analysis:** The type checker will validate widget names, property names, and value types. It will ensure that function references for event handlers have the correct signature.
3.  **C Code Generation:** The C code generator (`codegen/c.rs`) will be updated to handle the new AST nodes.
    - A `UiBlock` will generate a C function like `void render_ui_main_window(skia_canvas_t* canvas) { ... }`.
    - Each `Widget` node will recursively call a rendering function, passing the canvas and its properties.
    - A `Button` widget will generate calls to `skia_draw_rect` for its background and `skia_draw_text` for its label.
    - Event handling will be managed by a separate event loop that maps coordinates to widgets and triggers the associated function pointers.

## 5. Full Example

This example demonstrates a simple UI with a container, a text label, and a button.

```u
// file: simple_ui.ul

fn on_button_press() {
    core::print("Hello from the button!");
}

ui my_app {
    Container {
        width: 400,
        height: 300,
        background: rgb(240, 240, 240),
        
        child: Column {
            spacing: 20,
            children: [
                Text {
                    content: "Welcome to U!",
                    fontSize: 32
                },
                Button {
                    text: "Press Me",
                    width: 150,
                    height: 50,
                    onClick: on_button_press
                }
            ]
        }
    }
}
```

This specification provides the foundation for implementing the declarative UI DSL in U. The next steps are to modify the compiler to support this new syntax.
