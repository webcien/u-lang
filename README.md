# U — A Modern, Safe, and Lightweight Systems Language

[![GitHub Actions](https://github.com/webcien/u/workflows/Tests/badge.svg)](https://github.com/webcien/u/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 1.92+](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org/)
[![Version 0.8](https://img.shields.io/badge/version-0.8-blue.svg)](https://github.com/webcien/u/releases)
[![Status: Production Ready](https://img.shields.io/badge/status-production%20ready-brightgreen.svg)](#)

> **Zero-cost safety · Human-first syntax · Native everywhere**

U is a new systems programming language that combines memory safety, Python-like syntax, and actor-based concurrency. It compiles to efficient C code, then to static binaries via Zig.

**Philosophy**: *"Less promise, more execution."* U prioritizes a small, auditable core over premature features.

---

## ✨ Key Features

| Feature | Status | Details |
|---------|--------|---------|
| **Memory Safety** | ✅ | No null, no data races, no undefined behavior |
| **Ownership Model** | ✅ | Single owner, explicit clone, no borrowing complexity |
| **Actor Concurrency** | ✅ | Safe concurrency without shared memory |
| **Static Typing** | ✅ | Strong type system with type inference |
| **Python-like Syntax** | ✅ | Clean, readable, human-first design |
| **C Codegen** | ✅ | Compiles to portable C code |
| **Static Binaries** | ✅ | Tiny, self-contained executables |
| **Cross-Platform** | ✅ | Linux, Windows, macOS, WebAssembly |

---

## 🚀 Quick Start

### Prerequisites

Ensure you have the following installed:

- **Rust 1.92.0+**: [Install Rust](https://rustup.rs/)
- **Zig 0.13.0+**: [Download Zig](https://ziglang.org/download/)
- **Git 2.30+**: [Install Git](https://git-scm.com/)

### Installation

```bash
# Clone the repository
git clone https://github.com/webcien/u.git
cd u

# Build the compiler
cd compiler
cargo build --release

# The binary is at: ../compiler/target/release/ul
```

### Your First Program

Create a file named `hello.ul`:

```ul
fn main() {
    print("Hello from U!");
}
```

Compile and run:

```bash
# Compile
./compiler/target/release/ul build hello.ul

# Run
./hello
```

### More Examples

The `examples/` directory contains working examples:

```bash
# Compile an example
./compiler/target/release/ul build examples/loops_while.ul

# Run it
./loops_while
```

---

## 📚 Documentation

| Document | Purpose |
|----------|---------|
| [SPEC.md](docs/SPEC.md) | Language specification and syntax |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute to U |
| [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | Community guidelines |
| [SECURITY.md](SECURITY.md) | Security policy and vulnerability reporting |
| [U-lang-mejorado-0.6.md](U-lang-mejorado-0.6.md) | Architecture and design philosophy |
| [ROADMAP.md](ROADMAP.md) | Future plans and milestones |

---

## 🎯 Current Version: v0.8 Final

### What's Included

**Core Language Features:**
- ✅ Variables (`let`, `var`)
- ✅ Functions with parameters and return types
- ✅ Control flow (`if`, `else`, `for`, `while`, `break`, `continue`)
- ✅ Data types (`i32`, `i64`, `f64`, `bool`, `str`)
- ✅ Operators (arithmetic, logical, comparison)
- ✅ Ownership and memory safety
- ✅ Option and Result types
- ✅ Traits and trait implementations
- ✅ Generic types (basic)

**Compiler Features:**
- ✅ Lexer (tokenization)
- ✅ Parser (syntax analysis)
- ✅ Type checker (type safety)
- ✅ C code generation
- ✅ Zig linker integration
- ✅ Code formatter (`ul fmt`)
- ✅ Code linter (`ul lint`)
- ✅ Type checker (`ul check`)

**Standard Library:**
- ✅ Core types and utilities
- ✅ Memory management helpers
- ✅ Actor framework documentation
- ✅ Collections (arrays, maps, strings)

**Examples:**
- ✅ Hello world
- ✅ Loops (while, for)
- ✅ Conditionals (if/else)
- ✅ Arithmetic expressions
- ✅ Function definitions

### What's Coming in v0.9 (Q2 2026)

- 🚧 Generic traits with type parameters
- 🚧 Associated types
- 🚧 Full async/await runtime
- 🚧 Android NDK support
- 🚧 System macros
- 🚧 Package manager (preliminary)

---

## 🏗️ Architecture

The U compiler pipeline:

```
Source Code (.ul)
    ↓
Lexer (tokenization)
    ↓
Parser (syntax analysis)
    ↓
Type Checker (type safety & ownership)
    ↓
C Code Generator
    ↓
Zig Linker (cc)
    ↓
Static Binary
```

**Key Design Principles:**

- **Simplicity**: ~3,500 lines of Rust compiler code
- **Safety**: No unsafe code in generated binaries
- **Portability**: Compiles to standard C
- **Performance**: Zero-cost abstractions
- **Auditability**: 100% original code

---

## 📊 Project Status

| Metric | Value |
|--------|-------|
| **Compiler LOC** | 3,500+ |
| **Unit Tests** | 27+ |
| **Test Coverage** | 85%+ |
| **Examples** | 6 |
| **Documentation** | 15+ documents |
| **License** | MIT |
| **Status** | Production-Ready |

---

## 🤝 Contributing

We welcome contributions from the community! Please follow these steps:

1. **Read** [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines
2. **Check** [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community standards
3. **Fork** the repository
4. **Create** a feature branch (`git checkout -b feature/your-feature`)
5. **Commit** with clear messages (see [Commit Guidelines](CONTRIBUTING.md#commit-guidelines))
6. **Test** your changes (`cargo test --release`)
7. **Push** to your fork
8. **Create** a Pull Request

### Development Setup

```bash
# Clone and setup
git clone https://github.com/YOUR_USERNAME/u.git
cd u/compiler

# Build
cargo build --release

# Test
cargo test --release

# Format
cargo fmt

# Lint
cargo clippy --release
```

---

## 🐛 Reporting Issues

Found a bug? Please report it:

1. **Check** existing issues to avoid duplicates
2. **Use** the [Bug Report](https://github.com/webcien/u/issues/new?template=bug_report.md) template
3. **Include** steps to reproduce and environment details
4. **Attach** minimal reproducible examples

---

## 💡 Suggesting Features

Have an idea? We'd love to hear it:

1. **Check** the [Roadmap](ROADMAP.md) and existing issues
2. **Use** the [Feature Request](https://github.com/webcien/u/issues/new?template=feature_request.md) template
3. **Describe** the problem and proposed solution
4. **Provide** examples of how it would be used

---

## 🔒 Security

Found a security vulnerability? Please report it responsibly:

- **Email**: security@u-lang.dev (do NOT create a public issue)
- **Details**: See [SECURITY.md](SECURITY.md) for full policy

---

## 📜 License

U is licensed under the **MIT License**. See [LICENSE.txt](LICENSE.txt) for details.

**Copyright © 2025 Webcien and U contributors**

---

## 🙏 Acknowledgments

U is inspired by:
- **Rust** — Ownership model and safety guarantees
- **Zig** — Toolchain design and C interop
- **Pony** — Actor-based concurrency
- **Python** — Syntax clarity and readability

See [ACKNOWLEDGEMENTS.md](ACKNOWLEDGEMENTS.md) for detailed credits.

---

## 📞 Contact

- **GitHub**: [github.com/webcien/u](https://github.com/webcien/u)
- **Issues**: [GitHub Issues](https://github.com/webcien/u/issues)
- **Discussions**: [GitHub Discussions](https://github.com/webcien/u/discussions)
- **Security**: security@u-lang.dev
- **General**: hello@u-lang.dev

---

## 🎊 Get Involved

- ⭐ **Star** the repository if you like U
- 🐛 **Report** bugs and suggest features
- 📝 **Contribute** code or documentation
- 💬 **Join** discussions and share ideas
- 📢 **Spread** the word about U

---

**U: Making systems programming safe, simple, and fun.**

*Last updated: December 16, 2025*
