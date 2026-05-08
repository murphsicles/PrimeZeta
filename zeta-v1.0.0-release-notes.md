# Zeta v1.0.0 — The Foundational Release

> *"The language that will outlive its bootstrap."*

This is it. v1.0.0 strips away the last Rust scaffolding and delivers **pure Zeta** — a fully self-hosted systems language compiler, written in Zeta, compiled by Zeta, for anything you can throw at a CPU.

> *Source: https://github.com/murphsicles/zeta*
> *Website: https://z-lang.org*

---

## What v1.0.0 Means

The Rust bootstrap is gone. Every source file under `src/` is now Zeta. The compiler binary ships in `bin/zetac` — one static binary, zero dependencies, ready to compile Zeta code to native executables via LLVM.

**51+ self-hosted source files** spanning:

- A full compiler pipeline (lexer → parser → HIR → resolver → THIR → MIR → LLVM IR → machine code)
- A three-tier standard library
- Runtime support (actors, channels, tensors, ML inference)
- Actor-model concurrency with work-stealing scheduler
- 44+ categories of tests

---

## The Language

### Core Design

Zeta is a systems language built from first principles, inspired by Stepanov's *Elements of Programming*. Strong static typing, algebraic foundations, and surgical efficiency.

```zeta
fn add(x: i64, y: i64) -> i64 {
    return x + y;
}

fn infer() {
    let x = 42;       // inferred i64
    let y = 3.14;     // inferred f64
    let b = true;     // inferred bool
}
```

### Algebraic Data Types

Structs, enums, tuples — the full toolkit.

```zeta
struct Point {
    x: i64,
    y: i64,
}

enum Option<T> {
    Some(T),
    None,
}
```

### Generics & Specialization

Parametric polymorphism with monomorphization, concept constraints, and specialization.

```zeta
fn identity<T>(x: T) -> T {
    return x;
}

fn max<T: Ord>(a: T, b: T) -> T {
    if a >= b { return a; }
    return b;
}
```

### First-Class Functions & Closures

Functions are values. Closures capture their environment.

```zeta
fn apply(f: (i64) -> i64, x: i64) -> i64 {
    return f(x);
}

fn make_adder(n: i64) -> (i64) -> i64 {
    return fn(x: i64) -> i64 { x + n };
}
```

### Pattern Matching

Match expressions with destructuring, exhaustiveness checking.

```zeta
fn describe(v: Option<i64>) -> str {
    return match v {
        Option::Some(n) => if n > 0 { "positive" } else { "non-positive" },
        Option::None => "nothing",
    };
}
```

---

## Memory Model

Stack-priority allocation with ownership semantics and optional heap via `Vec`, `String`, and collections.

```zeta
fn stack_example() {
    let arr: [i64; 5] = [1, 2, 3, 4, 5];  // stack allocated
    let sum = arr[0] + arr[1];
}

fn heap_example() {
    let vec = Vec::new();
    vec.push(42);
    vec.push(100);
}
```

Immutability by default, explicit mutability with `mut`, borrow semantics that guarantee safety without a full borrow checker's complexity tax.

---

## Compiler Pipeline

```
Zeta Source (.z)
    │
    ▼ Lexer → Parser
    │
    ▼ AST → HIR
    │
    ▼ Resolver (imports, generics, specialization, concepts)
    │
    ▼ THIR (Typed HIR — fully resolved)
    │
    ▼ MIR (Mid-level IR — CFG with basic blocks)
    │
    ▼ LLVM IR → Optimization → Machine Code
```

The compiler is self-hosted in Zeta, meaning every stage of this pipeline is itself compiled by Zeta.

### Compile-Time Evaluation (CTFE)

Execute arbitrary Zeta code at compile time via `comptime` blocks. Algebraic semiring fusion enables mathematical optimization during compilation.

```zeta
const FACTORIAL_10: i64 = comptime {
    fn fact(n: i64) -> i64 {
        if n <= 1 { return 1; }
        return n * fact(n - 1);
    }
    fact(10)
};
// FACTORIAL_10 = 3628800 at runtime — zero compute cost
```

---

## Standard Library

Three tiers of modules, all implemented in self-hosted Zeta.

### Tier 1 — Core Types

| Module | Description |
|--------|-------------|
| `std::mem` | Memory operations, `size_of<T>()`, `align_of<T>()` |
| `std::ptr` | Raw pointer operations, null, read, write, copy |
| `std::cmp` | Ordering, comparison traits |
| `std::hash` | Hashing infrastructure |
| `std::iter` | Iterator traits and adapters |
| `std::vec` | Dynamic-size vector |
| `std::string` | UTF-8 string type |
| `std::option` | `Option<T>` enum |
| `std::result` | `Result<T, E>` enum |
| `std::marker` | Marker types for trait-based dispatch |

### Tier 2 — Systems & I/O

| Module | Description |
|--------|-------------|
| `std::fs` | Filesystem operations (read, write, create, remove) |
| `std::path` | Cross-platform path manipulation |
| `std::net` | Networking primitives |
| `std::sync` | Synchronization, atomics |
| `std::io` | Input/output streams |

### Tier 3 — Advanced

| Module | Description |
|--------|-------------|
| `std::char` | Character predicates and conversions |
| `std::time::Duration` | Time intervals and measurement |
| `std::process::Command` | Subprocess management |
| `std::thread` | Threading primitives |
| `std::collections` | Collection types |
| `std::ffi` | Foreign function interface |

---

## SIMD

Native SIMD vector types with auto-vectorization where possible.

```zeta
fn simd_add(a: [i64; 4], b: [i64; 4]) -> [i64; 4] {
    return a + b;
}
```

---

## Error System

Every compiler error has a unique code (E1001–E9015). 175+ distinct error codes covering syntax, type mismatches, resolution failures, and codegen issues.

```
error[E2001]: type mismatch: expected i64, found str
  ┌─ src/main.z:12:13
  │
7 │     let x: i64 = "hello";
  │                  ^^^^^^^
  │                  │
  │                  expected i64
  │                  found str (use std::ffi or explicit conversion)
```

---

## WebAssembly Support

Pass `--target wasm32` to compile straight to WebAssembly. Same LLVM codegen, different target triple.

```bash
zetac hello.z -o hello
./hello               # native binary — 42

zetac --target wasm32 hello.z -o hello
wasmtime hello.wasm   # WebAssembly — same 42
```

---

## Murphy's Sieve

Competition-ready wheel-optimized prime counting, baked into the language's DNA. 30030-wheel (2×3×5×7×11×13) gives 80.8% reduction in checks.

See `docs/02_MURPHYS_SIEVE_IMPLEMENTATION_GUIDE.md` for the full treatment.

---

## Correctness & Verification

Precondition and postcondition assertions at function entry/exit:

```zeta
fn divide(a: i64, b: i64) -> i64 {
    pre(b != 0, "division by zero");
    post(result matches _, "always returns");
    return a / b;
}
```

Loop invariants, mathematical property annotations (`#[commutative]`, `#[associative]`, `#[identity]`), and concept-based refinement hierarchies from Stepanov's elements:

```
Regular → TotallyOrdered → Semigroup → Monoid → Group → Ring
```

---

## Architecture

```
zeta/
├── bin/zetac              # v1.0.0 pre-built binary (Linux x86-64)
├── src/
│   ├── main.z             # Entry point
│   ├── frontend/          # Lexer, parser, AST, borrow checker
│   ├── middle/            # Resolver, MIR, type system, specialization
│   ├── backend/           # LLVM codegen, IR gen, JIT
│   └── runtime/           # Actors, arrays, tensors, host bindings, ML
├── tests/                 # 44+ categories of test coverage
├── docs/                  # Comprehensive documentation suite
├── examples/              # Sample programs
└── build/stubs/           # Generated stdlib stubs for bootstrap
```

---

## Getting Started

```bash
git clone https://github.com/murphsicles/zeta.git
cd zeta

# Compile and run
./bin/zetac examples/hello.z -o hello
./hello

# Run the test suite
./bin/zetac tests/language/basic_tests.z
```

---

## License

MIT — free as in freedom, free as in beer, free as in Zeta.

---

*Zeta v1.0.0 — Pure Zeta. Self-hosting. Foundational.*

*From first principles, for the final frontier.*
