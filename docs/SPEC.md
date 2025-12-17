# U Language Specification (v0.6)

## 1. MVP Objectives

- Readable and minimalist syntax (inspired by Python).
- Static typing with local type inference.
- Memory safety by default (no null, no UB).
- Safe concurrency through actors (no shared memory).
- Compilation to static binaries via C + Zig.
- Initial support: **Linux, Windows, macOS, WebAssembly (WASI)**.

---

## 2. Basic Grammar (EBNF)

```ebnf
program        = { declaration } ;
declaration    = function_decl | actor_decl ;
function_decl  = "fn", identifier, "(", [params], ")", [return_type], block ;
actor_decl     = "actor", identifier, block ;
params         = param, { ",", param } ;
param          = identifier, ":", type ;
return_type    = "->", type ;
type           = "i32" | "str" | "bool"
               | "Option<", type, ">"
               | "Result<", type, ",", type, ">" ;
block          = "{", { statement }, "}" ;
statement      = let_stmt
               | var_stmt
               | expr_stmt
               | return_stmt ;
let_stmt       = "let", identifier, "=", expression, ";" ;
var_stmt       = "var", identifier, "=", expression, ";" ;
expr_stmt      = expression, ";" ;
return_stmt    = "return", expression, ";" ;
expression     = string_literal
               | integer_literal
               | identifier
               | function_call
               | method_call ;
function_call  = identifier, "(", [arguments], ")" ;
method_call    = identifier, ".", identifier, "(", [arguments], ")" ;
arguments      = expression, { ",", expression } ;
string_literal = '"', { ? any char except unescaped " ? }, '"' ;
integer_literal = digit+ ;
identifier     = letter, { letter | digit | "_" } ;
letter         = "a" | ... | "z" | "A" | ... | "Z" ;
digit          = "0" | ... | "9" ;

```
### 3. Ownership Rules (Formal)
All values have a single owner.
let: creates an immutable variable.
var: creates a mutable variable (single mutable owner).
Deep copies only with explicit .clone() method.
No pointers or mutable references.
Ownership transfer occurs in:
Assignment (let x = y; → y is no longer usable).
Function call (f(y) → y is moved).
No explicit lifetimes; the compiler infers validity by lexical scope.
⚠️ Violating these rules → compilation error.

### 4. Concurrency Model
Only actors are allowed (defined with actor Name { ... }).
Communication exclusively by message passing.
No shared memory access → data races impossible by design.
Micro-runtime static (≤5 KB) manages:
Message queues (mailboxes).
Cooperative scheduling (no system threads).
Actor lifecycle.
Example:
```
actor Counter {
    var count: i32 = 0
    fn inc() { self.count += 1 }
    fn get() -> i32 { return self.count }
}

let c = Counter.spawn()
c.inc()
print(c.get().await)
```

### 5. Basic Types
