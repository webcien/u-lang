# U — A Modern, Safe, and Lightweight Systems Language

> **Zero-cost safety · Human-first syntax · Native everywhere**

U is a new systems programming language that combines:
- ✅ **Memory safety by default** (no null, no data races, no UB)
- ✅ **Python-like syntax** with static typing
- ✅ **Actor-based concurrency** (no shared memory)
- ✅ **Tiny static binaries** via C + Zig
- ✅ **Cross-compilation out of the box**: Linux, Windows, macOS, WebAssembly

> **Philosophy**: *"Less promise, more execution."*  
> U prioritizes a **small, auditable core** over premature features.

---

## 🚀 Quick Start

### 1. Prerequisites
- [Rust](https://rust-lang.org) (v1.70+)
- [Zig](https://ziglang.org) (v0.12+) in your `PATH`

### 2. Build the compiler
```sh
git clone https://github.com/webcien/u.git
cd u/compiler
cargo build --release
```
### 3. Write a program
```
hello.ul:
fn main() {
    print("Hello from U v0.6!")
}
```
### 4. Compile and run
```sh
# Linux
./target/release/ul build hello.ul --target x86_64-linux-musl
./hello

# WebAssembly
./target/release/ul build hello.ul --target wasm32-wasi
wasmtime hello.wasm
```

### 🎯 What's in v0.8?
✅ Included in v0.7                                  🚫 Not in v0.7 (planned for v0.8+)
i32, str, bool, Option<T>, Result<T,E>                  Arrays, Maps
Functions with parameters and returns                    Traits / Interfaces
Loop constructs (for, while, break, continue)           Borrowing, Lifetimes
Conditional statements (if, else)                       Full actor runtime
Arithmetic and logical operators                        LLVM backend
Ownership (single owner, explicit clone)                Android, iOS
Standard library (core, mem, actor docs)                Debugger, Formatter, Linter
C codegen + Zig linking                                 Package manager
Linux, Windows, macOS, WASM                             Macros

v0.7 focuses on control flow and practical programming while maintaining safety guarantees.

### 🧱 Architecture
.ul → Lexer → Parser → AST
        ↓
Type + Ownership + Concurrency Checker
        ↓
C Codegen → zig cc → static binary

- 100% original code — no copied logic from Rust, Zig, or others.
- Modular design — easy to extend or replace components.
- MIT licensed — free for any use.

### ⚖️ Legal & Originality
- U-Lang is inspired by Rust (ownership), Zig (toolchain), Pony (actors), and Python (syntax).
- No code is copied — all implementation is original.
- Licensed under MIT — see LICENSE.
- Acknowledgements in ACKNOWLEDGEMENTS.md.

### 🛠️ Roadmap
Version                        Goal
v0.6                           MVP: compile hello.ul to 4 platforms ✅ COMPLETE
v0.7                           Control flow, functions, stdlib basics ✅ COMPLETE
v0.8 (Q1 2026)                Actor runtime, traits, Android NDK, tooling
v0.9 (Q2 2026)                Package manager, iOS support, macros
v1.0 (Q3 2026)                Stable ABI, production-ready, 6+ platforms


### 🤝 Contributing
We welcome contributions! Please read:
- CONTRIBUTING.md
- docs/SPEC.md — the single source of truth
Note: All code must be original and compatible with MIT.

### 📜 License
MIT © 2025 Webcien and US contributors
See LICENSE for details.
