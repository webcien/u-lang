# U Language v1.0 - Release Notes

**Release Date:** December 17, 2025  
**Version:** 1.0.0  
**Codename:** "Phoenix"

## 🎉 Introducing U Language v1.0

We are thrilled to announce the first stable release of **U**, a modern systems programming language that combines the safety and expressiveness of high-level languages with the performance and control of low-level systems programming. U introduces a groundbreaking feature: a **declarative GUI DSL** that enables developers to build native, cross-platform applications with a syntax as elegant as Flutter or SwiftUI, but compiling directly to native code via C and Skia.

U is designed for developers who want to build fast, safe, and beautiful applications without sacrificing control or performance. Whether you're creating command-line tools, system utilities, or full-fledged GUI applications for desktop and mobile, U provides the tools you need.

---

## 🚀 What's New in v1.0

### A Complete Systems Programming Language

U v1.0 delivers a fully functional compiler that transforms U source code into optimized C code, which is then compiled to native binaries. The language features a strong static type system, memory safety through explicit `unsafe` blocks, and seamless interoperability with C libraries through a robust FFI system.

The compiler is written in Rust and produces clean, readable C code that can be easily inspected, debugged, and integrated into existing C/C++ projects. This makes U an excellent choice for both greenfield projects and gradual migration of legacy codebases.

### Declarative GUI DSL: Build UIs Like Never Before

The standout feature of U v1.0 is its **declarative GUI DSL**, which allows you to describe user interfaces in a natural, hierarchical syntax. The DSL is deeply integrated into the language and compiler, enabling compile-time validation and optimization of your UI code.

Here's a simple example:

```u
ui my_window {
    Container {
        width: 600,
        height: 400,
        background: rgb(240, 240, 245),
        child: Column {
            spacing: 20,
            children: [
                Text { content: "Welcome to U!", size: 24 },
                Button { text: "Click Me", onClick: handle_click }
            ]
        }
    }
}
```

This code compiles directly to Skia rendering calls, producing a native GUI application with zero runtime overhead. No virtual machines, no interpreters—just pure native code.

### Cross-Platform by Design

U v1.0 includes build scripts and integration guides for **Android** (via NDK) and **iOS** (via Xcode SDK), in addition to full support for Linux desktop environments. The same U code can be compiled for multiple platforms, with the runtime automatically adapting to the target environment.

The mobile integration is designed to be straightforward: compile your U code to a static library, link it with your native Android or iOS project, and you're ready to go. This approach gives you the best of both worlds: the productivity of a high-level language and the performance of native code.

### Event-Driven Architecture

U v1.0 introduces an event loop system that handles user input (mouse, keyboard, touch) and dispatches events to your UI components. The event system is designed to be extensible, allowing you to integrate with platform-specific event sources or build custom event handlers.

The layout engine, based on Flexbox, automatically positions and sizes your UI components, making it easy to build responsive interfaces that adapt to different screen sizes and orientations.

### Production-Ready Skia Integration

While the current release includes a proof-of-concept Skia wrapper for demonstration purposes, U v1.0 is designed to integrate seamlessly with the real Skia graphics library. The included documentation provides step-by-step instructions for building Skia from source and linking it with your U applications, enabling production-quality rendering with GPU acceleration, advanced text layout, and image processing.

---

## 📦 What's Included

### Compiler and Tools

The U compiler (`ul`) is a command-line tool that compiles `.ul` source files to C code and optionally links them into executable binaries. The compiler supports multiple build modes, including debug and release configurations, and provides clear error messages with line numbers to help you quickly identify and fix issues.

### Runtime Libraries

U v1.0 includes three runtime libraries that provide the foundation for GUI applications:

- **skia_glue.c:** A wrapper around the Skia graphics library that provides a C-compatible API for rendering.
- **event_loop.c:** An event dispatch system that handles user input and routes it to your event handlers.
- **layout.c:** A Flexbox-based layout engine that automatically positions and sizes UI components.

These libraries are designed to be minimal and efficient, adding only a few kilobytes to your final binary.

### Standard Library

The standard library includes a theme module that defines standardized colors, sizes, and spacing for UI components, making it easy to build consistent, professional-looking applications.

### Documentation and Examples

U v1.0 ships with comprehensive documentation, including a language guide, DSL specification, mobile integration guide, and Skia integration guide. The `examples/` directory contains multiple sample programs that demonstrate key language features, from basic FFI usage to a complete Todo application.

---

## 🎯 Use Cases

U is designed for a wide range of applications:

**System Utilities:** Build fast, efficient command-line tools that interact directly with the operating system.

**Desktop Applications:** Create native GUI applications with modern, responsive interfaces using the declarative DSL.

**Mobile Apps:** Compile your U code for Android and iOS, leveraging the full power of native development without the complexity of Java/Kotlin or Swift/Objective-C.

**Embedded Systems:** U's minimal runtime and direct C code generation make it suitable for resource-constrained environments.

**Game Development:** The Skia integration provides a solid foundation for 2D games, with support for sprites, animations, and custom rendering.

---

## 🔧 Getting Started

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/webcien/u-lang.git
   cd u-lang
   ```

2. Build the compiler:
   ```bash
   cd compiler
   cargo build --release
   ```

3. Add the compiler to your PATH:
   ```bash
   export PATH="$PWD/target/release:$PATH"
   ```

### Your First Program

Create a file `hello.ul`:

```u
fn main() {
    print("Hello, U!");
    return 0;
}
```

Compile and run:

```bash
ul build hello.ul
./hello
```

### Your First GUI Application

Create a file `gui_hello.ul`:

```u
extern "C" {
    fn skia_init();
    fn skia_create_surface(width: i32, height: i32) -> ptr;
    fn skia_get_canvas(surface: ptr) -> ptr;
    fn skia_save_png(surface: ptr, filename: ptr);
}

ui greeting {
    Text {
        content: "Hello, GUI World!",
        size: 32,
        color: rgb(50, 100, 200)
    }
}

fn main() {
    unsafe {
        skia_init();
        let surface = skia_create_surface(400, 200);
        let canvas = skia_get_canvas(surface);
        render_ui_greeting(canvas);
        skia_save_png(surface, "greeting.png");
    }
    return 0;
}
```

Compile and run:

```bash
ul build gui_hello.ul
./gui_hello
```

This will generate `greeting.png` with your rendered UI.

---

## 🛣️ Roadmap

The v1.0 release is just the beginning. Here's what we're planning for future releases:

**v1.1 (Q1 2026):** Full Skia integration, platform-specific event loops, expanded standard library, and package manager.

**v1.2 (Q2 2026):** Language server protocol (LSP) support, syntax highlighting for VS Code and other editors, and improved error messages.

**v2.0 (Q4 2026):** Generics, async/await, advanced type system features, optional garbage collection, and WebAssembly target.

---

## 🤝 Contributing

U is an open-source project, and we welcome contributions from the community. Whether you're fixing bugs, adding features, improving documentation, or building example applications, your help is appreciated.

Visit our [GitHub repository](https://github.com/webcien/u-lang) to get started.

---

## 📄 License

U is released under the MIT License, which means you're free to use it for any purpose, including commercial applications, without any restrictions.

---

## 🙏 Acknowledgments

U would not be possible without the incredible work of the Rust, Skia, and open-source communities. We're grateful for the tools and libraries that make projects like this possible.

---

## 📞 Contact

- **Website:** https://github.com/webcien/u-lang
- **Email:** u-lang@webcien.com
- **Issues:** https://github.com/webcien/u-lang/issues

---

**Welcome to the future of systems programming. Welcome to U.**
