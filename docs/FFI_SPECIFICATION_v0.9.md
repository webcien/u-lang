# FFI Specification for U Language v0.9

**Version**: 0.9.0  
**Date**: December 16, 2025  
**Status**: Implementation Ready  
**Author**: U Language Team

---

## 🎯 **Objective**

Enable **safe and efficient bidirectional interoperability** between U and C code:
- **U → C**: Call C functions from U
- **C → U**: Register U functions as C callbacks

This FFI system is the **foundation for Skia GUI integration** and all external library bindings.

---

## 📋 **Table of Contents**

1. [Syntax](#syntax)
2. [Type Mapping](#type-mapping)
3. [Safety Model](#safety-model)
4. [Memory Management](#memory-management)
5. [Callbacks](#callbacks)
6. [Code Generation](#code-generation)
7. [Examples](#examples)
8. [Testing Requirements](#testing-requirements)
9. [Implementation Phases](#implementation-phases)

---

## 1. Syntax

### 1.1 Declaring External Functions

External C functions are declared in `extern "C" { }` blocks:

```ul
extern "C" {
    fn printf(format: str, ...) -> i32;
    fn malloc(size: usize) -> ptr;
    fn free(ptr: ptr);
    fn sqrt(x: f64) -> f64;
}
```

**Grammar**:
```ebnf
extern_block = "extern", string_literal, "{", { extern_function }, "}" ;
extern_function = "fn", identifier, "(", [parameters], ")", ["->", type], ";" ;
```

**Notes**:
- Only `"C"` ABI is supported in v0.9
- Variadic functions (`...`) are supported but type-unsafe
- Functions must end with semicolon (no body)

---

### 1.2 Unsafe Blocks

All FFI calls must be wrapped in `unsafe { }` blocks:

```ul
fn main() {
    unsafe {
        printf("Hello from U!\n");
        let ptr = malloc(100);
        free(ptr);
    }
}
```

**Grammar**:
```ebnf
unsafe_block = "unsafe", "{", { statement }, "}" ;
```

**Rationale**: FFI calls bypass U's safety guarantees (ownership, null safety).

---

### 1.3 Callback Type Declarations

Function pointers for callbacks are declared as types:

```ul
type OnClick = fn() -> void;
type OnChange = fn(value: i32) -> void;
type Comparator = fn(a: i32, b: i32) -> bool;
```

**Grammar**:
```ebnf
function_type = "fn", "(", [parameters], ")", ["->", type] ;
```

---

### 1.4 Registering Callbacks

U functions can be passed to C as callbacks:

```ul
extern "C" {
    fn skia_set_click_handler(callback: fn());
}

fn handle_click() {
    print("Button clicked!");
}

fn main() {
    unsafe {
        skia_set_click_handler(handle_click);
    }
}
```

**Restrictions**:
- Only **top-level functions** can be callbacks (no closures in v0.9)
- Callback functions must match the expected signature exactly
- Callbacks must be kept alive (no premature deallocation)

---

## 2. Type Mapping

### 2.1 Primitive Types

| U Type | C Type | Size | Notes |
|--------|--------|------|-------|
| `i8` | `int8_t` | 1 byte | Signed 8-bit |
| `i16` | `int16_t` | 2 bytes | Signed 16-bit |
| `i32` | `int32_t` | 4 bytes | Signed 32-bit |
| `i64` | `int64_t` | 8 bytes | Signed 64-bit |
| `u8` | `uint8_t` | 1 byte | Unsigned 8-bit |
| `u16` | `uint16_t` | 2 bytes | Unsigned 16-bit |
| `u32` | `uint32_t` | 4 bytes | Unsigned 32-bit |
| `u64` | `uint64_t` | 8 bytes | Unsigned 64-bit |
| `f32` | `float` | 4 bytes | IEEE 754 single |
| `f64` | `double` | 8 bytes | IEEE 754 double |
| `bool` | `_Bool` | 1 byte | C99 boolean |
| `usize` | `size_t` | Platform | Pointer-sized unsigned |
| `isize` | `ssize_t` | Platform | Pointer-sized signed |

---

### 2.2 String Type

| U Type | C Type | Conversion | Notes |
|--------|--------|------------|-------|
| `str` | `const char*` | Copy to null-terminated | U owns memory |

**Conversion Rules**:
- **U → C**: String is copied to heap, null-terminated, passed as `const char*`
- **C → U**: Pointer is wrapped, U does NOT take ownership
- **Lifetime**: U-allocated strings are freed after FFI call returns

**Example**:
```ul
extern "C" {
    fn puts(s: str) -> i32;
}

fn main() {
    unsafe {
        puts("Hello!");  // String copied, null-terminated, freed after call
    }
}
```

---

### 2.3 Pointer Type

| U Type | C Type | Notes |
|--------|--------|-------|
| `ptr` | `void*` | Opaque pointer |

**Usage**:
- For C pointers that U doesn't dereference
- No type safety (intentionally opaque)
- Used for handles, contexts, etc.

**Example**:
```ul
extern "C" {
    fn malloc(size: usize) -> ptr;
    fn free(p: ptr);
}

fn main() {
    unsafe {
        let p = malloc(100);
        free(p);
    }
}
```

---

### 2.4 Option Type

| U Type | C Type | Conversion | Notes |
|--------|--------|------------|-------|
| `Option<T>` | `T*` | `Some(x)` → `&x`, `None` → `NULL` | For nullable pointers |

**Example**:
```ul
extern "C" {
    fn process(data: Option<i32>) -> i32;
}

fn main() {
    unsafe {
        process(Some(42));  // Passes pointer to 42
        process(None);      // Passes NULL
    }
}
```

---

### 2.5 Function Pointers

| U Type | C Type | Notes |
|--------|--------|-------|
| `fn(T1, T2) -> R` | `R (*)(T1, T2)` | Function pointer |

**Example**:
```ul
type Callback = fn(i32) -> void;

extern "C" {
    fn register_callback(cb: Callback);
}
```

---

## 3. Safety Model

### 3.1 Unsafe Blocks Required

**All FFI operations must be in `unsafe` blocks**:

```ul
fn main() {
    // ❌ ERROR: FFI call outside unsafe block
    printf("Hello\n");
    
    // ✅ OK: FFI call inside unsafe block
    unsafe {
        printf("Hello\n");
    }
}
```

**Rationale**: FFI bypasses U's safety guarantees:
- No null safety
- No ownership tracking
- No bounds checking
- Potential for undefined behavior

---

### 3.2 Type Safety

**The compiler validates**:
- ✅ Argument types match declared signature
- ✅ Return type matches expected type
- ✅ Callback signatures match exactly

**The compiler does NOT validate**:
- ❌ Pointer validity (C can pass invalid pointers)
- ❌ String encoding (C strings must be UTF-8)
- ❌ Memory lifetimes (C can free memory early)

**Example**:
```ul
extern "C" {
    fn foo(x: i32) -> i32;
}

fn main() {
    unsafe {
        foo(42);        // ✅ OK
        foo("hello");   // ❌ ERROR: type mismatch
    }
}
```

---

### 3.3 Callback Safety

**Callbacks must be**:
- ✅ Top-level functions (no closures)
- ✅ Kept alive during C code execution
- ✅ Thread-safe if called from multiple threads (C responsibility)

**Example**:
```ul
fn callback() {
    print("Called from C!");
}

fn main() {
    unsafe {
        register_callback(callback);  // ✅ OK: top-level function
    }
}
```

---

## 4. Memory Management

### 4.1 Ownership Rules

**U → C**:
- Primitive types: **copied**
- Strings: **copied** (U allocates, null-terminates, frees after call)
- Pointers: **borrowed** (U retains ownership)

**C → U**:
- Pointers: **borrowed** (C retains ownership)
- U must NOT free C-allocated memory (unless documented)

---

### 4.2 String Handling

**U → C**:
```ul
extern "C" {
    fn puts(s: str) -> i32;
}

fn main() {
    unsafe {
        puts("Hello!");  // U allocates, copies, null-terminates, frees
    }
}
```

**Generated C code**:
```c
// Generated by U compiler
char* u_str_to_c(const char* u_str, size_t len) {
    char* c_str = malloc(len + 1);
    memcpy(c_str, u_str, len);
    c_str[len] = '\0';
    return c_str;
}

void u_main() {
    char* tmp = u_str_to_c("Hello!", 6);
    puts(tmp);
    free(tmp);  // U frees after call
}
```

---

### 4.3 Pointer Lifetime

**U must ensure pointers remain valid**:

```ul
extern "C" {
    fn process(data: ptr);
}

fn main() {
    let x = 42;
    unsafe {
        process(&x as ptr);  // ✅ OK: x is alive
    }
    // x goes out of scope here
}
```

**Dangerous**:
```ul
extern "C" {
    fn store_pointer(data: ptr);
}

fn main() {
    let x = 42;
    unsafe {
        store_pointer(&x as ptr);  // ⚠️ DANGER: x will be freed
    }
    // x goes out of scope, but C might still have pointer!
}
```

**Solution**: Use heap allocation for long-lived data.

---

## 5. Callbacks

### 5.1 Callback Declaration

```ul
type OnClick = fn() -> void;
type OnChange = fn(value: i32) -> void;
type Comparator = fn(a: i32, b: i32) -> bool;
```

---

### 5.2 Callback Registration

```ul
extern "C" {
    fn skia_set_click_handler(callback: fn());
    fn qsort(base: ptr, nmemb: usize, size: usize, compar: fn(ptr, ptr) -> i32);
}

fn handle_click() {
    print("Clicked!");
}

fn compare(a: ptr, b: ptr) -> i32 {
    // Comparison logic
    return 0;
}

fn main() {
    unsafe {
        skia_set_click_handler(handle_click);
        qsort(data, 10, 4, compare);
    }
}
```

---

### 5.3 Callback Code Generation

**U function**:
```ul
fn handle_click() {
    print("Button clicked!");
}
```

**Generated C wrapper**:
```c
// U function (internal)
void u_handle_click() {
    u_print("Button clicked!");
}

// C-callable wrapper (generated)
void u_callback_handle_click() {
    u_handle_click();
}
```

**Registration**:
```c
// U code: skia_set_click_handler(handle_click);
// Generated C:
skia_set_click_handler(u_callback_handle_click);
```

---

### 5.4 Callback Restrictions (v0.9)

**Supported**:
- ✅ Top-level functions
- ✅ Functions with primitive parameters
- ✅ Functions returning primitives or void

**NOT Supported** (future):
- ❌ Closures (capturing environment)
- ❌ Methods (associated functions)
- ❌ Generic functions

---

## 6. Code Generation

### 6.1 External Function Calls

**U code**:
```ul
extern "C" {
    fn printf(format: str, ...) -> i32;
}

fn main() {
    unsafe {
        printf("Hello %d\n", 42);
    }
}
```

**Generated C**:
```c
#include <stdio.h>

void u_main() {
    char* tmp = u_str_to_c("Hello %d\n", 10);
    printf(tmp, 42);
    free(tmp);
}
```

---

### 6.2 Callback Wrappers

**U code**:
```ul
fn handle_click() {
    print("Clicked!");
}

extern "C" {
    fn register_callback(cb: fn());
}

fn main() {
    unsafe {
        register_callback(handle_click);
    }
}
```

**Generated C**:
```c
// U function
void u_handle_click() {
    u_print("Clicked!");
}

// C wrapper
void u_callback_handle_click() {
    u_handle_click();
}

// Main
void u_main() {
    register_callback(u_callback_handle_click);
}
```

---

### 6.3 Type Marshalling

**Primitives**: Direct pass-through (no conversion)

**Strings**: Copy, null-terminate, free

**Pointers**: Cast to `void*`

**Option**: Convert to pointer or NULL

---

## 7. Examples

### 7.1 Basic FFI Call

```ul
extern "C" {
    fn puts(s: str) -> i32;
}

fn main() {
    unsafe {
        puts("Hello from U!");
    }
}
```

---

### 7.2 Memory Allocation

```ul
extern "C" {
    fn malloc(size: usize) -> ptr;
    fn free(p: ptr);
}

fn main() {
    unsafe {
        let p = malloc(100);
        // Use p...
        free(p);
    }
}
```

---

### 7.3 Math Functions

```ul
extern "C" {
    fn sqrt(x: f64) -> f64;
    fn sin(x: f64) -> f64;
    fn cos(x: f64) -> f64;
}

fn main() {
    unsafe {
        let result = sqrt(16.0);
        print(result);  // 4.0
    }
}
```

---

### 7.4 Callbacks

```ul
type OnClick = fn() -> void;

extern "C" {
    fn register_click(cb: OnClick);
}

fn handle_click() {
    print("Button clicked!");
}

fn main() {
    unsafe {
        register_click(handle_click);
    }
}
```

---

### 7.5 Skia Integration (Preview)

```ul
extern "C" {
    fn skia_init(width: i32, height: i32);
    fn skia_clear(color: u32);
    fn skia_draw_rect(x: f64, y: f64, w: f64, h: f64, color: u32);
    fn skia_flush();
}

fn main() {
    unsafe {
        skia_init(800, 600);
        skia_clear(0xFF121212);
        skia_draw_rect(100.0, 100.0, 200.0, 100.0, 0xFFFF0000);
        skia_flush();
    }
}
```

---

## 8. Testing Requirements

### 8.1 Unit Tests (20+ required)

**Category 1: Basic FFI Calls** (5 tests)
- ✅ Call C function with no arguments
- ✅ Call C function with primitive arguments
- ✅ Call C function with string argument
- ✅ Call C function returning value
- ✅ Call C function with multiple arguments

**Category 2: Type Marshalling** (5 tests)
- ✅ i32, i64, u32, u64 marshalling
- ✅ f32, f64 marshalling
- ✅ bool marshalling
- ✅ str marshalling (copy, null-terminate)
- ✅ ptr marshalling

**Category 3: Callbacks** (5 tests)
- ✅ Register callback with no arguments
- ✅ Register callback with primitive arguments
- ✅ Register callback returning value
- ✅ Multiple callbacks
- ✅ Callback invocation from C

**Category 4: Safety** (3 tests)
- ✅ Error on FFI call outside unsafe block
- ✅ Type mismatch error
- ✅ Signature mismatch error

**Category 5: Memory** (2 tests)
- ✅ String memory freed after call
- ✅ Pointer validity

---

### 8.2 Integration Tests (3+ required)

1. **printf test**: Call C printf with formatted string
2. **math test**: Call C math functions (sqrt, sin, cos)
3. **callback test**: Register U callback, invoke from C

---

## 9. Implementation Phases

### Phase 1: Lexer Updates (Week 1)
- Add `extern` keyword
- Add `unsafe` keyword
- Add `ptr` keyword

### Phase 2: Parser Updates (Week 2)
- Parse `extern "C" { }` blocks
- Parse `unsafe { }` blocks
- Parse function pointer types
- Parse callback declarations

### Phase 3: Type Checker Updates (Week 3-4)
- Validate FFI signatures
- Enforce unsafe blocks
- Check type compatibility
- Validate callback signatures

### Phase 4: Code Generation (Week 5-6)
- Generate C function declarations
- Generate FFI call code
- Generate callback wrappers
- Implement type marshalling

### Phase 5: Testing (Week 7)
- Write 20+ unit tests
- Write 3+ integration tests
- Test on Linux and Windows

### Phase 6: Documentation (Week 8)
- Complete specification
- Write examples
- Create tutorial

---

## 10. Success Criteria

### ✅ Must Have (v0.9 MVP)
- [ ] `extern "C" { }` syntax works
- [ ] `unsafe { }` blocks work
- [ ] Primitive type marshalling works
- [ ] String marshalling works (U → C)
- [ ] `ptr` type works
- [ ] Callbacks work (C → U)
- [ ] 20+ tests passing
- [ ] 3+ examples working
- [ ] Documentation complete

### 🎯 Nice to Have (v0.9.1+)
- [ ] `Option<T>` marshalling
- [ ] Variadic function support
- [ ] Better error messages
- [ ] Performance optimizations

---

## 11. Future Enhancements (v1.0+)

- Closures as callbacks
- Generic callbacks
- C++ interop
- Other ABIs (stdcall, fastcall)
- Automatic binding generation
- Inline assembly

---

## 📊 **Status**

**Version**: 0.9.0  
**Date**: December 16, 2025  
**Status**: ✅ Specification Complete — Ready for Implementation  
**Next**: Begin lexer updates

---

*U: Making systems programming safe, simple, and fun.*
