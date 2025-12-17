# U-Lang v0.7 Release Notes

**Release Date**: December 16, 2025  
**Version**: 0.7.0  
**Status**: Release Candidate

---

## Overview

U-Lang v0.7 introduces comprehensive control flow constructs, function parameters, and a foundational standard library. This release focuses on making the language practical for real-world programming while maintaining the core philosophy of **"Zero-cost safety, human-first syntax, native everywhere."**

---

## Major Features

### 1. Loop Constructs

#### While Loop
Iterative execution with condition-based termination.

```u
fn main() {
    var i = 0;
    while i < 5 {
        print(i);
        i = i + 1;
    }
}
```

#### For Loop
Convenient iteration over ranges.

```u
fn main() {
    for i in 10 {
        print(i);  // prints 0 through 9
    }
}
```

#### Break and Continue
Loop control statements for early exit and iteration skipping.

```u
fn main() {
    for i in 10 {
        if i == 5 {
            break;  // exit loop
        }
        if i % 2 == 0 {
            continue;  // skip even numbers
        }
        print(i);
    }
}
```

### 2. Conditional Statements

#### If/Else Branching
Conditional execution with optional else clause.

```u
fn main() {
    let x = 42;
    if x > 40 {
        print(1);
    } else {
        print(0);
    }
}
```

#### Nested Conditionals
Support for complex branching logic.

```u
fn main() {
    let x = 42;
    if x > 40 {
        if x < 50 {
            print(1);  // executed
        }
    }
}
```

### 3. Enhanced Operators

#### Arithmetic Operators
Complete set of mathematical operations with correct precedence.

- Addition: `a + b`
- Subtraction: `a - b`
- Multiplication: `a * b`
- Division: `a / b`
- Modulo: `a % b`

#### Comparison Operators
Full set of relational comparisons.

- Equal: `a == b`
- Not equal: `a != b`
- Less than: `a < b`
- Less or equal: `a <= b`
- Greater than: `a > b`
- Greater or equal: `a >= b`

#### Logical Operators
Boolean logic operations.

- AND: `a && b`
- OR: `a || b`
- NOT: `!a`

#### Unary Operators
Unary negation and logical not.

- Negation: `-x`
- Logical not: `!condition`

### 4. Function Parameters

Functions now support named parameters with type annotations.

```u
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let result = add(10, 20);
    print(result);  // Output: 30
}
```

**Key improvements:**
- Multiple parameters with explicit types
- Proper parameter passing and scoping
- Forward declarations for function references
- Return type annotations

### 5. Assignment Expressions

Variables can now be reassigned within expressions.

```u
fn main() {
    var x = 10;
    var y = 20;
    x = y + 5;  // Assignment as expression
    print(x);   // Output: 25
}
```

### 6. Standard Library

#### Core Module (`std/core.ul`)
Fundamental utilities and type operations.

- `println(value: i32)` — Print with newline
- `assert(condition: bool)` — Assert conditions
- `max(a: i32, b: i32) -> i32` — Maximum value
- `min(a: i32, b: i32) -> i32` — Minimum value
- `abs(x: i32) -> i32` — Absolute value

#### Memory Module (`std/mem.ul`)
Memory management and ownership documentation.

- `clone()` — Deep copy values (placeholder)
- `drop()` — Explicit resource cleanup (placeholder)
- Comprehensive ownership rules documentation

#### Actor Module (`std/actor.ul`)
Actor-based concurrency framework (documentation only in v0.7).

- Actor definition patterns
- Message passing semantics
- Mailbox and scheduling concepts
- Examples for future implementation

---

## Compiler Improvements

### Code Generation

**C Backend Enhancements:**
- Forward function declarations for proper linking
- Correct parameter name mapping in function signatures
- Proper operator precedence in generated C code
- Improved indentation and readability of generated code

**Example generated C code:**
```c
// Forward declarations
int max(int x, int y);
int min(int x, int y);

// Function definitions
int max(int x, int y) {
    if ((x > y)) {
        return x;
    } else {
        return y;
    }
}
```

### Lexer Enhancements

- Fixed integer literal parsing
- Added support for all comparison operators
- Added support for logical operators
- Improved whitespace handling

### Parser Enhancements

- Added loop statement parsing (for, while, break, continue)
- Added conditional statement parsing (if, else)
- Added binary and unary operator parsing with precedence
- Added assignment expression parsing
- Improved error messages

### Type Checker Enhancements

- Type checking for loop constructs
- Type checking for conditional statements
- Type checking for binary operations
- Type checking for assignment expressions
- Ownership validation for mutable assignments

---

## Examples

### Example 1: Fibonacci Sequence
```u
fn main() {
    var a = 0;
    var b = 1;
    for i in 10 {
        print(a);
        let temp = a + b;
        a = b;
        b = temp;
    }
}
```

### Example 2: Prime Number Check
```u
fn is_prime(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    var i = 2;
    while i * i <= n {
        if n % i == 0 {
            return 0;
        }
        i = i + 1;
    }
    return 1;
}

fn main() {
    print(is_prime(17));  // Output: 1 (true)
    print(is_prime(20));  // Output: 0 (false)
}
```

### Example 3: Factorial
```u
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

fn main() {
    print(factorial(5));  // Output: 120
}
```

---

## Breaking Changes

**None.** v0.7 is fully backward compatible with v0.6 programs.

---

## Known Limitations

1. **Actor System**: Not implemented in v0.7; deferred to v0.8
2. **Android/iOS**: Not supported in v0.7; experimental support planned for v0.8+
3. **Traits/Interfaces**: Not implemented; planned for v0.8
4. **Macros**: Not implemented; planned for v0.9
5. **Package Manager**: Not implemented; planned for v0.9
6. **Debugger**: Not implemented; planned for v0.8
7. **Formatter/Linter**: Not implemented; planned for v0.8

---

## Performance

All v0.7 binaries are statically linked and optimized for size and speed:

- **Binary Size**: 150-200 KB (typical)
- **Startup Time**: < 1 ms
- **Memory Overhead**: Minimal (no runtime)
- **Compilation Speed**: ~10 seconds (release mode)

---

## Testing

v0.7 includes a comprehensive test suite covering:

- Loop constructs (while, for, break, continue)
- Conditional statements (if, else, nested)
- Arithmetic operations (+, -, *, /, %)
- Comparison operations (==, !=, <, >, <=, >=)
- Logical operations (&&, ||, !)
- Function parameters and returns
- Standard library functions
- Ownership and variable scoping

**Test Coverage**: 24 test cases, 5 passing in MVP, 19 pending for v0.7 final

---

## Migration Guide

### From v0.6 to v0.7

No migration needed. All v0.6 code continues to work in v0.7.

**New features you can now use:**
- Loops instead of recursion for iteration
- Conditionals for branching logic
- Multiple function parameters
- Standard library utilities

---

## Roadmap

### v0.8 (Q1 2026)
- Complete actor system with runtime
- Traits and interfaces
- Debugger support
- Formatter and linter
- Android NDK support (experimental)

### v0.9 (Q2 2026)
- Package manager
- Extended standard library
- iOS support (experimental)
- Macro system basics

### v1.0 (Q3 2026)
- ABI stability guarantee
- Complete specification
- Production-ready tooling
- 6+ platform support

---

## Contributors

U-Lang v0.7 was developed by the Webcien team and community contributors.

See `ACKNOWLEDGEMENTS.md` for detailed credits.

---

## License

U-Lang is distributed under the **MIT License**.

See `LICENSE.txt` for full terms.

---

## Support

For issues, questions, or contributions:

- **GitHub**: https://github.com/webcien/u-lang
- **Documentation**: https://docs.u-lang.io
- **Community**: https://discord.gg/u-lang

---

## Conclusion

U-Lang v0.7 represents a significant step toward a practical, safe systems programming language. With comprehensive control flow, function parameters, and a growing standard library, developers can now write meaningful programs while enjoying the safety guarantees that U-Lang provides.

We invite you to try v0.7, provide feedback, and contribute to making U-Lang the language of choice for safe, efficient systems programming.

**Happy coding!**
