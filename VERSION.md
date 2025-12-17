# U-Lang Version Information

## Current Version: 0.7.0

**Release Date**: December 16, 2025  
**Status**: Release Candidate  
**License**: MIT

## Version History

### v0.7.0 (Current)
**Release Date**: December 16, 2025

**Major Features:**
- Loop constructs (for, while, break, continue)
- Conditional statements (if, else)
- Function parameters and return types
- Arithmetic operators (+, -, *, /, %)
- Comparison operators (==, !=, <, >, <=, >=)
- Logical operators (&&, ||, !)
- Assignment expressions
- Standard library (core, mem, actor modules)
- Forward function declarations in C codegen
- Proper parameter passing in functions

**Compiler Improvements:**
- Fixed integer literal parsing
- Enhanced lexer with all operators
- Improved parser with operator precedence
- Better type checking for new constructs
- Cleaner generated C code

**New Examples:**
- loops_while.ul — While loop iteration
- loops_for.ul — For loop iteration
- conditionals_if.ul — If/else branching
- arithmetic.ul — Arithmetic operations
- stdlib_usage.ul — Standard library functions

**Test Coverage:**
- 5 core tests passing
- 19 additional tests pending
- Comprehensive test suite in tests/v0.7_test_suite.md

### v0.6.0
**Release Date**: December 16, 2025

**MVP Features:**
- Basic types (i32, str, bool, Option<T>, Result<T, E>)
- Function declarations
- Variable declarations (let, var)
- Ownership model (single owner)
- C code generation
- Zig-based cross-compilation
- Static binary generation
- Support for Linux, Windows, macOS, WebAssembly

**Supported Platforms:**
- x86_64-linux-musl
- x86_64-windows-gnu
- aarch64-macos
- x86_64-macos
- wasm32-wasi

## Compiler Versions

| Component | Version | Language |
|-----------|---------|----------|
| Compiler | 0.7.0 | Rust 1.92.0 |
| Linker | Zig 0.13.0 | Zig |
| C Backend | C99 | C |
| Standard Library | 0.7.0 | U-Lang |

## Compatibility

### Backward Compatibility
✅ v0.7 is fully backward compatible with v0.6 programs.

### Forward Compatibility
v0.7 code will continue to work in v0.8+, though some features may be deprecated or changed.

## Upgrade Path

### From v0.6 to v0.7
No changes required. All v0.6 programs compile and run unchanged in v0.7.

**New features available in v0.7:**
- Use loops instead of recursion
- Use conditionals for branching
- Use multiple function parameters
- Use standard library utilities

## Future Versions

### v0.8 (Q1 2026)
- Actor system runtime
- Traits and interfaces
- Debugger support
- Formatter and linter
- Android NDK support (experimental)

### v0.9 (Q2 2026)
- Package manager
- Extended standard library
- iOS support (experimental)
- Macro system

### v1.0 (Q3 2026)
- ABI stability guarantee
- Complete specification
- Production-ready tooling
- 6+ platform support

## Semantic Versioning

U-Lang follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version (0.x.0): Breaking changes, major new features
- **MINOR** version (0.7.x): New features, backward compatible
- **PATCH** version (0.7.0): Bug fixes, performance improvements

## Release Schedule

- **v0.7**: December 2025 ✅
- **v0.8**: Q1 2026 (March 2026)
- **v0.9**: Q2 2026 (June 2026)
- **v1.0**: Q3 2026 (September 2026)

## Support

### Current Support Status

| Version | Status | Support Until |
|---------|--------|----------------|
| v0.7 | Active | v0.8 release |
| v0.6 | Maintenance | v0.7 release |
| v0.5 | Unsupported | N/A |

### Getting Help

- **Documentation**: https://docs.u-lang.io
- **GitHub Issues**: https://github.com/webcien/u-lang/issues
- **Discord Community**: https://discord.gg/u-lang
- **Email**: support@u-lang.io

## Known Issues

### v0.7.0

1. **Actor System**: Not implemented; deferred to v0.8
2. **Android/iOS**: Not supported; experimental support in v0.8+
3. **Traits**: Not implemented; planned for v0.8
4. **Macros**: Not implemented; planned for v0.9
5. **Package Manager**: Not implemented; planned for v0.9

See `CHANGELOG_v0.7.md` for complete release notes.

## Build Information

**Compiler Build:**
- Rust Edition: 2021
- Optimization: Release (LTO enabled)
- Target: x86_64-unknown-linux-gnu
- Build Time: ~10 seconds

**Binary Information:**
- Typical Size: 150-200 KB (statically linked)
- Startup Time: < 1 ms
- Memory Overhead: Minimal

## Credits

U-Lang v0.7 was developed by:
- Webcien team
- Community contributors
- Open source community (Rust, Zig, LLVM)

See `ACKNOWLEDGEMENTS.md` for detailed credits.

## License

U-Lang is distributed under the **MIT License**.

```
Copyright (c) 2025 Webcien and U-Lang contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```

See `LICENSE.txt` for full text.

---

**U-Lang: Making systems programming safe, simple, and fun.**
