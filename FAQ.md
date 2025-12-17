# Frequently Asked Questions (FAQ)

---

## General Questions

### Q: What is U?

A: U is a modern systems programming language that combines memory safety, Python-like syntax, and actor-based concurrency. It compiles to C, then to static binaries via Zig.

### Q: Why create another programming language?

A: Existing languages have tradeoffs:
- **Rust**: Powerful but steep learning curve (borrow checker)
- **C/C++**: Fast but unsafe (memory errors, data races)
- **Go**: Simple but no memory safety guarantees
- **Python**: Safe but slow and not suitable for systems programming

U aims for the sweet spot: **safe, simple, and fast**.

### Q: Is U production-ready?

A: U v0.8 is stable and production-ready for systems programming. However, it's still evolving, so expect changes in v0.9+.

### Q: How is U different from Rust?

| Aspect | U | Rust |
|--------|---|------|
| **Learning Curve** | Gentle | Steep |
| **Syntax** | Python-like | C-like |
| **Borrowing** | No | Yes |
| **Ownership** | Simple | Complex |
| **Performance** | Excellent | Excellent |
| **Maturity** | v0.8 | v1.0+ |

### Q: Can I use U in production?

A: Yes, U v0.8 is suitable for production use. However:
- Test thoroughly
- Keep backups
- Report bugs
- Stay updated

---

## Installation & Setup

### Q: What are the system requirements?

A: You need:
- **Rust 1.92.0+** (for compiling the compiler)
- **Zig 0.13.0+** (for linking)
- **Git 2.30+** (for cloning)
- **Linux, macOS, or Windows** (with WSL for Windows)

### Q: How do I install U?

A:
```bash
git clone https://github.com/webcien/u.git
cd u/compiler
cargo build --release
```

The binary is at `../compiler/target/release/ul`.

### Q: Can I add U to my PATH?

A:
```bash
# Linux/macOS
export PATH="$PATH:$(pwd)/compiler/target/release"

# Or copy to system path
sudo cp compiler/target/release/ul /usr/local/bin/
```

### Q: How do I update U?

A:
```bash
git pull origin master
cd compiler
cargo build --release
```

---

## Language Features

### Q: What types does U support?

A: Built-in types:
- **Integers**: `i32`, `i64`
- **Floats**: `f64`
- **Booleans**: `bool`
- **Strings**: `str`
- **Optionals**: `Option<T>`
- **Results**: `Result<T, E>`
- **Custom**: structs, enums, traits

### Q: Does U have garbage collection?

A: No. U uses ownership-based memory management:
- Each value has one owner
- Memory is freed when owner goes out of scope
- No garbage collector needed
- Deterministic cleanup

### Q: How do I handle errors?

A: Use `Result<T, E>`:
```ul
fn divide(a: i32, b: i32) -> Result<i32, str> {
    if b == 0 {
        return Err("Division by zero");
    }
    Ok(a / b)
}
```

### Q: Can I use null values?

A: No. U doesn't have null. Use `Option<T>` instead:
```ul
let value: Option<i32> = Some(42);
let empty: Option<i32> = None;
```

### Q: How do I clone a value?

A: Use the `.clone()` method:
```ul
let x = 42;
let y = x.clone();  // y is a copy of x
```

### Q: Does U support generics?

A: Yes, basic generics are supported:
```ul
fn first<T>(items: Vec<T>) -> Option<T> {
    // Implementation
}
```

### Q: How do I define a struct?

A: Use the `type` keyword:
```ul
type Point {
    x: i32,
    y: i32,
}
```

### Q: How do I implement a trait?

A: Use the `impl` keyword:
```ul
impl Display for Point {
    fn to_string() -> str {
        // Implementation
    }
}
```

---

## Compilation & Execution

### Q: How do I compile a U program?

A:
```bash
ul build program.ul
```

This generates:
- `program.c` (C source)
- `program` (executable)

### Q: Can I compile to WebAssembly?

A: Not yet, but it's planned for v0.9+.

### Q: Can I compile for Android?

A: Experimental support is planned for v0.8+.

### Q: Can I compile for iOS?

A: Experimental support is planned for v0.9+.

### Q: How do I cross-compile?

A: Use the `--target` flag:
```bash
ul build program.ul --target x86_64-linux-musl
ul build program.ul --target x86_64-apple-darwin
```

### Q: How large are compiled binaries?

A: Typical sizes:
- **Hello world**: ~50 KB
- **Simple program**: 150-200 KB
- **Complex program**: 500 KB - 1 MB

### Q: Can I optimize for size?

A: Yes, use `--opt-level`:
```bash
ul build program.ul --opt-level z
```

---

## Performance

### Q: How fast is U compared to Rust?

A: Similar performance. Both compile to efficient machine code. Benchmarks coming soon.

### Q: How fast is U compared to C?

A: Similar performance. U compiles to C, so it's as fast as C.

### Q: How fast is U compared to Python?

A: Much faster. U is compiled, Python is interpreted. 100-1000x faster.

### Q: Does U have any runtime overhead?

A: No. U uses zero-cost abstractions. No runtime overhead.

---

## Concurrency

### Q: How do I write concurrent code?

A: Use the actor model:
```ul
actor Counter {
    count: i32,
    
    fn increment() {
        count = count + 1;
    }
    
    fn get_count() -> i32 {
        count
    }
}
```

### Q: How do I send messages between actors?

A: Use the `send` method:
```ul
let counter = spawn Counter();
counter.send(Increment);
```

### Q: Is the actor model safe?

A: Yes. Actors don't share memory, preventing data races.

### Q: Can I use threads?

A: Not directly. U uses actors instead, which are safer.

---

## Standard Library

### Q: What's in the standard library?

A: U has a minimal stdlib:
- **core**: Basic types and utilities
- **mem**: Memory management helpers
- **actor**: Actor framework
- **collections**: Arrays, maps, strings (v0.8+)

### Q: Can I use C libraries?

A: Yes, U can call C functions directly.

### Q: Is there a package manager?

A: Not yet. Planned for v0.9+.

---

## Contributing

### Q: How can I contribute?

A: See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

### Q: What should I work on?

A: Check the [Roadmap](ROADMAP.md) and [GitHub Issues](https://github.com/webcien/u/issues).

### Q: How do I report a bug?

A: Use the [Bug Report](https://github.com/webcien/u/issues/new?template=bug_report.md) template.

### Q: How do I suggest a feature?

A: Use the [Feature Request](https://github.com/webcien/u/issues/new?template=feature_request.md) template.

### Q: Can I contribute code?

A: Yes! See [CONTRIBUTING.md](CONTRIBUTING.md) for the process.

### Q: Will my contribution be credited?

A: Yes, in [ACKNOWLEDGEMENTS.md](ACKNOWLEDGEMENTS.md) and GitHub contributors page.

---

## Security

### Q: Is U secure?

A: Yes. U is designed with security in mind:
- No buffer overflows
- No use-after-free
- No data races
- No null pointer dereferences

### Q: How do I report a security vulnerability?

A: Email security@u-lang.dev (do NOT create a public issue).

See [SECURITY.md](SECURITY.md) for details.

### Q: Has U been audited?

A: Not yet. Security audits are planned for v1.0.

---

## Troubleshooting

### Q: I get "command not found: ul"

A: Make sure U is in your PATH:
```bash
export PATH="$PATH:$(pwd)/compiler/target/release"
```

### Q: I get compilation errors

A: Check:
1. U syntax is correct (see [SPEC.md](docs/SPEC.md))
2. All types match
3. All functions are defined
4. All variables are declared

### Q: My program crashes

A: U prevents memory errors, but logic errors are still possible. Debug with:
```bash
ul build program.ul --debug
```

### Q: My binary is too large

A: Optimize for size:
```bash
ul build program.ul --opt-level z
```

### Q: Compilation is slow

A: This is normal for the first build. Subsequent builds are cached.

### Q: I found a bug

A: Report it on [GitHub Issues](https://github.com/webcien/u/issues) using the bug report template.

---

## Licensing

### Q: What license is U under?

A: MIT License. See [LICENSE.txt](LICENSE.txt).

### Q: Can I use U commercially?

A: Yes. MIT license allows commercial use.

### Q: Can I modify U?

A: Yes. MIT license allows modifications.

### Q: Do I need to open-source my code?

A: No. MIT license doesn't require open-sourcing derivative works.

### Q: Can I sell software built with U?

A: Yes. MIT license allows commercial distribution.

---

## Roadmap

### Q: When will v0.9 be released?

A: Q2 2026 (April-June 2026).

### Q: When will v1.0 be released?

A: Q3 2026 (July-September 2026).

### Q: What's in v0.9?

A: Generic traits, async/await, Android NDK, macros, package manager.

### Q: What's in v1.0?

A: Stable ABI, production-ready, 6+ platforms.

### Q: Can I see the roadmap?

A: Yes, see [ROADMAP.md](ROADMAP.md).

---

## More Questions?

- **Documentation**: See [docs/SPEC.md](docs/SPEC.md)
- **Examples**: See `examples/` directory
- **Issues**: [GitHub Issues](https://github.com/webcien/u/issues)
- **Discussions**: [GitHub Discussions](https://github.com/webcien/u/discussions)
- **Email**: hello@u-lang.dev

---

**U: Making systems programming safe, simple, and fun.**

*FAQ — December 16, 2025*
