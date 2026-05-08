# Diagnostic Sprint — v0.9.21

World-class error diagnostics for Zeta. Every possible compiler error has a code (EXXXX), a human-readable message, a source span, a help hint, and where applicable a "did you mean?" suggestion. Warnings are configurable. `--explain` prints full documentation. Output supports human (color), plain, and JSON modes.

**Target: 155 error codes + 30 warning codes = 185 total**

---

## Phase 1: Wire the Pipeline

### 1.1 Route main.rs through compile_with_diagnostics
- Refactor main.rs to use `compile_with_diagnostics()` from lib.rs
- Preserve AOT/JIT/REPL/--bootstrap modes
- Thread source code through for snippet extraction
- **Files:** `src/main.rs`, `src/lib.rs`

### 1.2 Add `--explain EXXXX` CLI flag
- Look up error code in registry
- Print: code, category, full description, example code, suggestion
- Exit without compiling
- **Files:** `src/main.rs`, `src/error_codes.rs`

### 1.3 Add warning control CLI flags
- `-A EXXXX` / `--allow` — suppress warning
- `-D EXXXX` / `--deny` — upgrade warning to error
- `-W EXXXX` / `--warn` — force warning level
- Thread through `DiagnosticReporter.WarningConfig`
- **Files:** `src/main.rs`, `src/diagnostics.rs`

### 1.4 Add `--error-format` flag
- `--error-format=human` (default) — colored with snippets
- `--error-format=plain` — no colors, still formatted
- `--error-format=json` — machine-readable JSON
- **Files:** `src/main.rs`, `src/diagnostics.rs`

---

## Phase 2: Parser Diagnostics (E1XXX — 25 codes)

### Foundation: Parser position tracking
- `parse_zeta()` returns `(remaining, asts)` but discards `nom` error positions
- Wrap parser to capture `nom::Err::Error(NomError { input })` → derive line/column from remaining
- Track filename through parser for span creation
- **Files:** `src/frontend/parser/parser.rs`, `src/lib.rs`

### Error codes

| Code | Error | Detail |
|---|---|---|
| E1001 | General parse error | Catch-all syntax error with position |
| E1002 | Unexpected token | Parser hit something it couldn't match |
| E1003 | Expected identifier | `fn ( )` — missing function name |
| E1004 | Expected type | `let x: = 5;` — missing type annotation |
| E1005 | Expected expression | `if { }` — empty condition |
| E1006 | Expected semicolon | Expression statement missing `;` |
| E1007 | Expected `{` | `fn main)` — missing opening brace |
| E1008 | Expected `(` or `<` | Function call missing argument list |
| E1009 | Expected `)` | Unclosed parentheses |
| E1010 | Expected `}` | Unclosed brace in block/struct/dict |
| E1011 | Expected `]` | Unclosed bracket in array/index |
| E1012 | Unmatched delimiter | Mismatched `(`/`{`/`[` pairs |
| E1013 | Invalid literal integer | `let x = 99999999999999999999;` overflow |
| E1014 | Invalid literal float | `let x = .;` — malformed |
| E1015 | Invalid escape sequence | `"\z"` — unknown escape |
| E1016 | Invalid string literal | Unterminated or malformed string |
| E1017 | Invalid character literal | `let c: char = 'ab';` — too many chars |
| E1018 | Unknown operator | `let x = a ** b;` — `**` not defined |
| E1019 | Invalid attribute syntax | `#[[]]` — malformed |
| E1020 | Unknown attribute | `#[magic]` — attribute doesn't exist |
| E1021 | Invalid pattern syntax | Match arm pattern malformed |
| E1022 | Invalid where clause | `where T:` — missing bounds |
| E1023 | Invalid generic syntax | `<T,>` — trailing comma (non-final) |
| E1024 | Reserved keyword used as identifier | `let fn = 5;` |
| E1025 | Doc comment on invalid item | `///` on expressions |

---

## Phase 3: Type System Diagnostics (E2XXX — 40 codes)

### Foundation: Typechecker emits diagnostics
- Change `Resolver::typecheck(&self, asts) -> bool` to `fn typecheck(&mut self, asts) -> Vec<Diagnostic>`
- Thread a `&mut Vec<Diagnostic>` through infer/constrain_eq for targeted errors
- Track source spans on AST nodes for type error positioning
- **Files:** `src/middle/resolver/resolver.rs`, `src/middle/resolver/typecheck.rs`

### Error codes

| Code | Error | Detail |
|---|---|---|
| E2001 | Typecheck failed | Generic type error catch-all |
| E2002 | Type mismatch | `i64` where `str` expected |
| E2003 | Const eval error | CTFE failure (exists) |
| E2004 | Cannot infer type | Type annotation required |
| E2005 | Missing type annotation | Function parameter/return needs type |
| E2006 | Undefined function | Call to unknown function (with suggestion) |
| E2007 | Wrong number of arguments | `foo(1,2)` but `foo` takes 3 |
| E2008 | Argument type mismatch | `foo(5)` but `foo` takes `str` |
| E2009 | Undefined variable | Variable not in scope (with suggestion) |
| E2010 | Undefined type | `let x: Foo = 5;` — `Foo` unknown |
| E2011 | Undefined concept/trait | `impl Foo for Bar` — `Foo` unknown |
| E2012 | Undefined struct field | `point.z` — field doesn't exist (with suggestion) |
| E2013 | Undefined method | `s.foo()` — method not found (with suggestion) |
| E2014 | Cannot infer return type | Function body doesn't constrain return |
| E2015 | Return type mismatch | Function returns `i64` but body returns `str` |
| E2016 | Concept not satisfied | `T: Display` but `T` doesn't implement |
| E2017 | Missing concept implementation | `impl Display for Foo` but method missing |
| E2018 | Duplicate concept implementation | `impl Display for Foo` twice |
| E2019 | Invalid operator for type | `a + b` where types don't support `+` |
| E2020 | Invalid unary operator | `!x` where x doesn't support negation |
| E2021 | Invalid comparison | `a > b` where types aren't ordered |
| E2022 | Recursive type definition | `type A = A;` |
| E2023 | Recursive type alias | `type A = B; type B = A;` |
| E2024 | Invalid generic parameter count | `Vec<i32, str>` — too many |
| E2025 | Generic parameter kind mismatch | Type param where const param expected |
| E2026 | Generic bound not satisfied | `fn foo<T: Clone>` called with non-Clone type |
| E2027 | Cycle in generic resolution | Self-referential generic constraint |
| E2028 | Invalid enum variant | `Option::Foo` — variant doesn't exist |
| E2029 | Invalid struct constructor | Struct `Point` has no constructor `new` |
| E2030 | Mismatched struct fields | `Point { x: 1 }` but `y` is required |
| E2031 | Missing struct field | Required field not provided |
| E2032 | Extra struct field | Field doesn't exist on struct |
| E2033 | Cannot apply unary operator | `-true` — doesn't make sense |
| E2034 | Array index type | Index must be integer, got `str` |
| E2035 | Array out of bounds (const) | `arr[100]` where `arr` has size 5 |
| E2036 | Invalid array size | `[i64; -1]` — negative size |
| E2037 | Type has no size known at compile time | `[str; 5]` — str is unsized |
| E2038 | Mismatched types in binary operation | `5 + "hello"` |
| E2039 | IF expression type mismatch | All branches must return same type |
| E2040 | Match arm type mismatch | All arms must return same type |

---

## Phase 4: Semantic Diagnostics (E3XXX — 20 codes)

| Code | Error | Detail |
|---|---|---|
| E3001 | Macro expansion error | Macro expansion failure (exists) |
| E3002 | Unsupported macro derive | `#[derive(Default)]` not implemented (exists) |
| E3003 | Undefined module | `use nonexistent::mod;` — module path unknown |
| E3004 | Undefined name | Name not found in any scope (with Levenshtein suggestion) |
| E3005 | Unused variable | Variable declared but never read |
| E3006 | Unused parameter | Function parameter never used |
| E3007 | Unused function | Function defined but never called |
| E3008 | Unused type definition | Type defined but never referenced |
| E3009 | Variable shadowed | `let x = 1; let x = 2;` — rebinding without `mut` |
| E3010 | Unreachable expression | Code after `return`/`break`/`continue` |
| E3011 | Dead code in if/else | Else branch after if that always returns |
| E3012 | Divergent function | Function that never returns (missing return) |
| E3013 | Missing return value | Function with return type missing return expr |
| E3014 | Invalid visibility qualifier | `pub(crate)` in wrong context |
| E3015 | Private field access | Accessing private field outside module |
| E3016 | Private function call | Calling private function outside module |
| E3017 | Conflicting function definition | Two functions with same name and same params |
| E3018 | Conflicting type definition | Two types with same name in same scope |
| E3019 | Invalid re-export | Re-exporting private item |
| E3020 | Invalid documentation syntax | `///` followed by invalid markdown |

---

## Phase 5: Codegen Diagnostics (E4XXX — 15 codes)

| Code | Error | Detail |
|---|---|---|
| E4001 | No main function | Entry point `fn main` not found (exists) |
| E4002 | Code generation failed | LLVM IR construction error (exists) |
| E4003 | LLVM verification error | Module failed LLVM verification |
| E4004 | Unsupported feature | Feature exists in parser but not in codegen |
| E4005 | Unsupported type in codegen | Type exists but has no LLVM mapping |
| E4006 | Unsupported operation | Operation exists in MIR but not in codegen |
| E4007 | Unsupported intrinsic | `__builtin_*` not implemented in codegen |
| E4008 | Inline assembly error | Bad `unsafe` asm block |
| E4009 | Target feature not available | SIMD on non-AVX target |
| E4010 | AOT compilation failed | Could not write object file |
| E4011 | JIT compilation failed | LLVM JIT engine error |
| E4012 | Function too large | Function body exceeds codegen limits |
| E4013 | Recursive function inline limit | Stack overflow protection |
| E4014 | Constant too large | Const exceeds LLVM value range |
| E4015 | Linkage conflict | Function with same name but different linkage |

---

## Phase 6: Runtime & Linker Diagnostics (E5XXX — 15 codes)

| Code | Error | Detail |
|---|---|---|
| E5001 | Linker error | gcc/ld invocation failed (exists) |
| E5002 | Undefined symbol | `undefined reference` to symbol (exists) |
| E5003 | Multiple definition | Linker found duplicate symbols |
| E5004 | Runtime function not found | Host function declaration not linked |
| E5005 | Linker flag error | Invalid `-l` or `-L` flag |
| E5006 | Object file error | `.o` file format or corruption |
| E5007 | Runtime assertion failure | `assert!()` failed at runtime |
| E5008 | Runtime panic | Panic in generated code |
| E5009 | Stack overflow | Recursion too deep at runtime |
| E5010 | Out of memory | `runtime_malloc` returned 0 |
| E5011 | Null pointer dereference | Pointer 0 used in `unsafe` block |
| E5012 | Arithmetic overflow | Integer overflow in debug mode |
| E5013 | Division by zero | `/` or `%` by zero |
| E5014 | Array index out of bounds | Index >= length at runtime |
| E5015 | Unwrap on None/Err | `unwrap()` on `Option::None` or `Result::Err` |

---

## Phase 7: Module System Diagnostics (E6XXX — 15 codes)

| Code | Error | Detail |
|---|---|---|
| E6001 | Module not found | `use foo;` — no `foo.z` or `foo/mod.z` (exists) |
| E6002 | Failed to load module | IO error reading module file (exists) |
| E6003 | Circular module dependency | `A` imports `B` imports `A` |
| E6004 | Name conflict in module | Two items exported with same name |
| E6005 | Invalid re-export | Re-exporting private/non-existent item |
| E6006 | Unused import | Module imported but never referenced |
| E6007 | Module not in module tree | File outside crate root's module tree |
| E6008 | Invalid module declaration | `mod foo;` but file not found |
| E6009 | Module file parse error | `.z` file has syntax errors |
| E6010 | Conflicting module paths | Module accessible via two paths |
| E6011 | Recursive module alias | `mod a = b; mod b = a;` |
| E6012 | Invalid visibility for module | `pub(self)` doesn't make sense for modules |
| E6013 | Wildcard import conflict | `use A::*; use B::*;` — name `foo` from both |
| E6014 | Missing module name | `mod {}` without name |
| E6015 | Module not expected here | `mod` declaration in expression context |

---

## Phase 8: Borrow Checker Diagnostics (E7XXX — 20 codes)

| Code | Error | Detail |
|---|---|---|
| E7001 | Borrow error | Generic borrow check failure (exists) |
| E7002 | Use after move | Value consumed by move, used again |
| E7003 | Borrow of moved value | Reference to value after move |
| E7004 | Mutable borrow conflict | Two mutable borrows of same value |
| E7005 | Immutable+mutable borrow | Mutable borrow while immutable exists |
| E7006 | Borrow of local returns from function | Returning reference to stack variable |
| E7007 | Dangling reference | Reference outlives referenced value |
| E7008 | Assign to borrowed value | Writing to value through owned reference while borrowed |
| E7009 | Reborrow beyond original borrow | Extending borrow lifetime |
| E7010 | Borrow in conflicting scope | Borrow spans across a mutation boundary |
| E7011 | Cannot move out of borrow | Moving value that's behind a reference |
| E7012 | Cannot move out of `&mut` | Cannot move from mutable reference |
| E7013 | Partial move conflicts | Moving one field while other fields borrowed |
| E7014 | Invalid ownership transfer | Moving value in a context that expects borrowing |
| E7015 | Lifetime mismatch | Reference lifetime doesn't match expected |
| E7016 | Missing lifetime annotation | Function needs explicit lifetime |
| E7017 | Lifetime elision conflict | Multiple lifetimes, can't elide |
| E7018 | Captured variable in closure | Closure captures variable with wrong borrow mode |
| E7019 | Mutability mismatch | `&mut` expected, `&` given |
| E7020 | Consumed by move in loop | Value moved in one iteration, used in next |

---

## Phase 9: Optimization / CTFE Diagnostics (E8XXX — 10 codes)

| Code | Error | Detail |
|---|---|---|
| E8001 | Const evaluation limit | CTFE exceeded iteration/recursion limit (exists) |
| E8002 | Const evaluation blocked | CTFE couldn't evaluate expression |
| E8003 | Unsupported const operation | Operation not supported in const context |
| E8004 | Mutable reference in const | `&mut` not allowed in const context |
| E8005 | Const fn call outside const | Calling non-const fn from const context |
| E8006 | Inlining limit exceeded | Function too complex to inline |
| E8007 | Loop unrolling limit | Loop too large for full unroll |
| E8008 | Tail call optimization failed | Recursive call not in tail position |
| E8009 | Constant propagation limit | Value set too large for constant propagation |
| E8010 | MIR optimization error | MIR pass failed |

---

## Phase 10: Tooling / Configuration Diagnostics (E9XXX — 15 codes)

| Code | Error | Detail |
|---|---|---|
| E9001 | Invalid CLI argument | Bad command line flag (exists) |
| E9002 | File not found | Input `.z` file doesn't exist (exists) |
| E9003 | Output path invalid | `-o` path unwritable or directory |
| E9004 | Compiler internal error | ICE — bug in compiler, please report |
| E9005 | LSP initialization error | Language server failed to start |
| E9006 | LSP document error | Could not parse document in IDE |
| E9007 | LSP completion error | Completion handler failed |
| E9008 | Configuration error | `zeta.json`/`Zeta.toml` malformed |
| E9009 | Dependency resolution error | Could not resolve package dependency |
| E9010 | Network error | Failed to fetch remote dependency |
| E9011 | Build cache corruption | Incremental cache invalid, rebuilding |
| E9012 | Test runner error | Test harness failed |
| E9013 | Benchmark error | Benchmark harness failed |
| E9014 | REPL initialization error | Interactive mode failed to start |
| E9015 | Resource limit exceeded | Compiler ran out of memory |

---

## Phase 11: Warning Codes (W0XXX — 30 codes)

| Code | Warning | Detail |
|---|---|---|
| W0001 | CTFE warning | Non-fatal CTFE issue (exists) |
| W0002 | Macro expansion warning | Non-fatal macro issue (exists) |
| W0003 | Typecheck non-fatal | Type errors proceeding anyway (exists) |
| W0004 | Unused variable | `let x = 5;` without subsequent use |
| W0005 | Unused import | `use std::foo;` never referenced |
| W0006 | Unused function | Function defined but never called |
| W0007 | Unused type definition | Type defined but never referenced |
| W0008 | Unused assignment | Value assigned but never read |
| W0009 | Unreachable code | Code after `return`/`break` |
| W0010 | Redundant cast | Casting to the same type |
| W0011 | Redundant pattern | Match arm that will never match |
| W0012 | Missing match arms | Non-exhaustive match (without `_`) |
| W0013 | Overlapping match arms | Earlier arm always matches, later arm dead |
| W0014 | Deprecated feature | Using feature marked for removal |
| W0015 | Shadowed variable | Name reused in inner scope |
| W0016 | Variable not mutated | `let mut x = 5;` never mutated |
| W0017 | Unnecessary `mut` | `&mut` parameter never mutated |
| W0018 | Unnecessary `unsafe` | `unsafe` block contains no unsafe operations |
| W0019 | Missing documentation | Public item without `///` |
| W0020 | Suspicious assignment in condition | `if x = 5 { }` — maybe `==` intended |
| W0021 | Comparison of identical values | `if x == x { }` |
| W0022 | Division by zero (constant) | `let x = 5 / 0;` |
| W0023 | Named constant used as pattern | Match arm with constant name, not value |
| W0024 | Non-snake-case identifier | `fn CamelCase(){}` — convention violation |
| W0025 | Non-capitalized type name | `struct foo {}` — types should be PascalCase |
| W0026 | Path statement with no effect | `foo;` — expression result discarded |
| W0027 | Result not used | `Result` value dropped without handling |
| W0028 | Large stack allocation | Array on stack exceeds recommended size |
| W0029 | Recursive call without bound | No base case detected in recursion |
| W0030 | Long function (complexity warning) | Function exceeds line/statement threshold |

---

## Phase 12: Rich Error Output

### 12.1 Source snippets with underlines
Complete `Diagnostic::format()` to produce:

```
error[E2002]: type mismatch
  --> src/main.z:12:5
   |
12 |     let x: str = 42;
   |         ----     ^^ expected `str`, found `i64`
   |         |
   |         expected due to this type annotation
```

- Multi-line span rendering for range errors
- Label support: primary (^^) and secondary (--) labels with annotations
- **Files:** `src/diagnostics.rs`

### 12.2 Color output
- ANSI escape sequences
- Red (`\x1b[31m`) for errors, yellow for warnings, blue (`\x1b[34m`) for notes, cyan (`\x1b[36m`) for code
- Bold for primary labels, dim for secondary
- Respect `NO_COLOR` env var and `--color` flag
- **Files:** `src/diagnostics.rs`

### 12.3 JSON output mode
- `--error-format=json`: each diagnostic as JSON line
```json
{"code":"E2002","severity":"error","file":"src/main.z","line":12,"column":5,"message":"type mismatch: expected `str`, found `i64`"}
```
- Array of diagnostics at end of compilation
- **Files:** `src/diagnostics.rs`, `src/main.rs`

### 12.4 `--explain` full implementation
- Print: code, category, full description, examples, suggestion
- Multi-paragraph explanation with bullet points
- Before/after code examples
- **Files:** `src/main.rs`, `src/error_codes.rs`

### 12.5 Error code documentation generation
- `zeta --error-list` — list all error codes with one-line descriptions
- `zeta --error-list --json` — machine-readable code list
- Auto-generate error code documentation for website/manual
- **Files:** `src/main.rs`, `src/error_codes.rs`

---

## Summary

| Phase | Focus | Files | New Codes |
|---|---|---|---|
| 1 | Pipeline wiring, CLI flags | main.rs, lib.rs, diagnostics.rs | — |
| 2 | Parser diagnostics (E1XXX) | parser.rs, lib.rs, error_codes.rs | 25 |
| 3 | Type system diagnostics (E2XXX) | resolver.rs, typecheck.rs | 40 |
| 4 | Semantic diagnostics (E3XXX) | resolver.rs | 20 |
| 5 | Codegen diagnostics (E4XXX) | codegen.rs | 15 |
| 6 | Runtime & linker diagnostics (E5XXX) | main.rs, host.rs | 15 |
| 7 | Module diagnostics (E6XXX) | module_resolver.rs | 15 |
| 8 | Borrow checker diagnostics (E7XXX) | borrow.rs | 20 |
| 9 | Optimization/CTFE (E8XXX) | ctfe/, optimization.rs | 10 |
| 10 | Tooling diagnostics (E9XXX) | main.rs, lsp/ | 15 |
| 11 | Warnings (W0XXX) | diagnostics.rs, main.rs | 30 |
| 12 | Rich output, color, --explain, JSON | diagnostics.rs, main.rs | — |

**Total: 185 codes (155 error + 30 warning)**
**Target concept: ~10-15 per category, mirrors Rust's comprehensiveness**

---

## Dependencies Between Phases

```
Phase 1 (pipeline) ──► Phase 2 (parser) ──► Phase 3 (type) ──► Phase 4 (semantic)
                        │                                           │
                        ▼                                           ▼
                   Phase 12 (output) ◄──── Phase 6 (linker) ◄── Phase 5 (codegen)
                        │                    │
                        ▼                    ▼
                   Phase 7 (modules) ──► Phase 8 (borrow) ──► Phase 9 (ctfe)
                                                                    │
                                                                    ▼
                   Phase 11 (warnings) ◄── Phase 10 (tooling) ◄─────┘
```

Phases 2, 3, 4, 5, 6, 7, 8, 9, 10 are independent — each can be developed in parallel once Phase 1 is done. Phase 11 (warnings) depends on all error-producing phases. Phase 12 (rich output) is the rendering layer that all phases feed into.
