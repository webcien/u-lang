# U Compiler Architecture

This document describes the architecture and design of the U compiler.

---

## Overview

The U compiler is a multi-stage compiler that transforms U source code into static binaries:

```
.ul Source Code
    ↓
Lexer (Tokenization)
    ↓
Parser (Syntax Analysis)
    ↓
AST (Abstract Syntax Tree)
    ↓
Type Checker (Type Safety & Ownership)
    ↓
C Code Generator
    ↓
Zig Linker (zig cc)
    ↓
Static Binary
```

---

## Compiler Stages

### 1. Lexer (lexer.rs)

**Purpose**: Convert source code into tokens

**Responsibilities:**
- Read source code character by character
- Recognize keywords, identifiers, literals, operators
- Track line and column numbers for error reporting
- Skip whitespace and comments

**Key Types:**
- `Token` — Represents a single token
- `TokenType` — Enumeration of all token types
- `Lexer` — Main lexer struct

**Example:**
```rust
let input = "fn main() { print(\"Hello\"); }";
let tokens = Lexer::new(input).tokenize();
// Produces: [Fn, Identifier("main"), LeftParen, RightParen, ...]
```

### 2. Parser (parser.rs)

**Purpose**: Build an Abstract Syntax Tree (AST) from tokens

**Responsibilities:**
- Parse function definitions
- Parse statements (let, var, if, while, for, return)
- Parse expressions (binary, unary, function calls)
- Handle operator precedence
- Report syntax errors

**Key Types:**
- `Declaration` — Top-level items (functions, types, traits)
- `Statement` — Executable code (assignments, loops, conditionals)
- `Expression` — Values and computations
- `Type` — Type annotations
- `Parser` — Main parser struct

**Example:**
```rust
let tokens = lexer.tokenize();
let ast = Parser::new(tokens).parse();
// Produces: Program with declarations
```

### 3. Type Checker (type_checker.rs)

**Purpose**: Verify type safety and ownership rules

**Responsibilities:**
- Verify all types are correct
- Check function signatures match calls
- Verify ownership rules (single owner, explicit clone)
- Detect use-after-free and data races
- Build symbol table and type registry

**Key Types:**
- `Type` — Enumeration of all types
- `TypeChecker` — Main type checker struct
- `TypeError` — Type checking errors
- `Scope` — Variable scope management

**Example:**
```rust
let ast = parser.parse();
let checked = TypeChecker::new().check(&ast)?;
// Produces: Type-safe AST or errors
```

### 4. Code Generator (codegen/c.rs)

**Purpose**: Generate C code from the AST

**Responsibilities:**
- Generate C function definitions
- Generate C variable declarations
- Generate C expressions and statements
- Handle type conversions
- Generate vtables for traits
- Emit function prototypes

**Key Types:**
- `CodeGenerator` — Main code generator struct
- `CType` — C type representations

**Example:**
```rust
let checked_ast = type_checker.check(&ast)?;
let c_code = CodeGenerator::new().generate(&checked_ast);
// Produces: C source code
```

### 5. Linker (Zig)

**Purpose**: Compile C code to static binary

**Process:**
1. Save generated C code to file
2. Invoke `zig cc` to compile C to object files
3. Link object files into static binary
4. Strip symbols for smaller binary

---

## Module Organization

```
compiler/src/
├── main.rs              # CLI and entry point
├── lexer.rs            # Tokenization
├── parser.rs           # Syntax analysis
├── type_checker.rs     # Type safety
├── traits.rs           # Trait system
├── actor_runtime.rs    # Actor framework
├── formatter.rs        # Code formatting
├── linter.rs           # Static analysis
├── diagnostics.rs      # Error reporting
├── codegen/
│   ├── mod.rs          # Module definition
│   └── c.rs            # C code generation
└── Cargo.toml          # Dependencies
```

---

## Data Structures

### Token

Represents a single token in the source code:

```rust
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}
```

### AST Nodes

**Declaration:**
```rust
pub enum Declaration {
    Function {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    Type { ... },
    Trait { ... },
    TraitImpl { ... },
}
```

**Statement:**
```rust
pub enum Statement {
    Let { name: String, value: Expression },
    Var { name: String, value: Expression },
    If { condition: Expression, then_body: Vec<Statement>, else_body: Option<Vec<Statement>> },
    While { condition: Expression, body: Vec<Statement> },
    For { var: String, range: Expression, body: Vec<Statement> },
    Return { value: Option<Expression> },
    Expression { expr: Expression },
}
```

**Expression:**
```rust
pub enum Expression {
    Literal { value: String, type_: Type },
    Identifier { name: String },
    Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> },
    Unary { op: UnaryOp, operand: Box<Expression> },
    Call { function: String, args: Vec<Expression> },
    Assignment { target: String, value: Box<Expression> },
}
```

**Type:**
```rust
pub enum Type {
    I32,
    I64,
    F64,
    Bool,
    String,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Custom(String),
    Generic(String, Vec<Type>),
}
```

---

## Type System

### Built-in Types

| Type | Size | Description |
|------|------|-------------|
| `i32` | 32 bits | Signed 32-bit integer |
| `i64` | 64 bits | Signed 64-bit integer |
| `f64` | 64 bits | 64-bit floating point |
| `bool` | 1 bit | Boolean (true/false) |
| `str` | Variable | String (UTF-8) |

### Composite Types

- `Option<T>` — Optional value (Some(T) or None)
- `Result<T, E>` — Result or error (Ok(T) or Err(E))
- Custom types (structs, enums)
- Generic types (T, U, V, etc.)

### Type Checking Rules

1. **Assignment**: Target type must match value type
2. **Function Calls**: Argument types must match parameter types
3. **Binary Operations**: Both operands must be compatible
4. **Ownership**: Single owner, explicit clone required
5. **Traits**: Implementation must satisfy trait signature

---

## Ownership Model

U implements a simplified ownership model:

### Rules

1. **Single Owner**: Each value has exactly one owner
2. **Move on Assignment**: Assignment transfers ownership
3. **Explicit Clone**: Copying requires explicit `.clone()`
4. **No Borrowing**: No mutable or immutable references
5. **Scope-based Cleanup**: Values dropped at end of scope

### Example

```ul
let x = 42;        // x owns 42
let y = x;         // x moves to y, x is invalid
let z = x.clone(); // Error: x already moved
```

---

## Error Handling

### Error Types

**LexerError:**
- Unexpected character
- Unterminated string
- Invalid number format

**ParseError:**
- Unexpected token
- Missing semicolon
- Invalid syntax

**TypeError:**
- Type mismatch
- Undefined variable
- Undefined function
- Ownership violation

### Error Reporting

Errors include:
- Error message
- Location (file, line, column)
- Suggestion for fix
- Severity level (Note, Warning, Error, Fatal)

---

## Optimization Strategies

### Current Optimizations

1. **Dead Code Elimination**: Remove unused variables
2. **Constant Folding**: Evaluate constant expressions at compile time
3. **Type Specialization**: Generate specialized code for generic types

### Planned Optimizations

1. **Inlining**: Inline small functions
2. **Loop Unrolling**: Unroll simple loops
3. **SIMD**: Auto-vectorization
4. **Tail Call Optimization**: Optimize tail calls

---

## Testing

### Unit Tests

Each module includes unit tests:

```rust
#[test]
fn test_lexer_basic() {
    let input = "fn main() { }";
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(tokens.len(), 5);
}
```

### Integration Tests

End-to-end tests compile and run programs:

```bash
./compiler/target/release/ul build examples/hello.ul
./hello
# Output: Hello from U!
```

### Test Coverage

Current coverage: 85%+

Target coverage: 90%+

---

## Performance Characteristics

### Compilation Time

- **Lexing**: O(n) where n = source code size
- **Parsing**: O(n) with linear lookahead
- **Type Checking**: O(n) with single pass
- **Code Generation**: O(n) with linear output

### Binary Size

- **Typical Program**: 150-200 KB (static)
- **Minimal Program**: ~50 KB
- **Complex Program**: 500 KB - 1 MB

### Runtime Performance

- **Zero-cost Abstractions**: No runtime overhead
- **C Compilation**: Optimized by GCC/Clang
- **Static Linking**: No dynamic library overhead

---

## Design Decisions

### Why C as Intermediate?

1. **Portability**: C compilers available everywhere
2. **Performance**: Mature optimizations in C compilers
3. **Interop**: Easy to call C libraries
4. **Simplicity**: Easier than generating machine code

### Why Zig for Linking?

1. **Cross-compilation**: Zig handles cross-compilation well
2. **Musl Support**: Easy static linking with musl
3. **No Dependencies**: Zig is self-contained
4. **Future**: Zig can replace C codegen eventually

### Why Ownership Model?

1. **Safety**: Prevents memory errors
2. **Simplicity**: Easier than borrow checker
3. **Performance**: No garbage collection
4. **Predictability**: Deterministic cleanup

---

## Future Improvements

### Parser Enhancements

- [ ] Macro system
- [ ] Pattern matching
- [ ] Destructuring
- [ ] Async/await syntax

### Type System Enhancements

- [ ] Generic traits with type parameters
- [ ] Associated types
- [ ] Higher-ranked types
- [ ] Type classes

### Code Generation Enhancements

- [ ] LLVM backend option
- [ ] WebAssembly backend
- [ ] Direct machine code generation
- [ ] JIT compilation

### Optimization Enhancements

- [ ] Interprocedural optimization
- [ ] Vectorization
- [ ] Parallelization
- [ ] Profile-guided optimization

---

## Contributing to the Compiler

To contribute to the compiler:

1. **Understand** this architecture document
2. **Read** the relevant module's documentation
3. **Write** tests for your changes
4. **Follow** the code style guidelines
5. **Submit** a pull request with clear description

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

**U: Making systems programming safe, simple, and fun.**

*Architecture Documentation — December 16, 2025*
