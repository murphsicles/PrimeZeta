# WASM Sprint — v0.9.22

Native WebAssembly compilation for Zeta. Compile any Zeta program to `.wasm`.

## Phase 1: Target Triple & Object File (1 day)

**Goal:** `zetac hello.z --target wasm32` produces a `.wasm` object file.

- Add `--target` CLI flag: `"native"` (default) or `"wasm32"`
- `Target::initialize_all()` instead of `Target::initialize_native()` — registers WASM targets
- Switch target triple: `"wasm32-unknown-unknown"` for standalone WASM, `"wasm32-wasi"` for WASI
- `FileType::Object` still works — LLVM writes a `.wasm` object file

**Files:** `src/backend/codegen/jit.rs`, `src/main.rs`

## Phase 2: WASM Linker (1 day)

**Goal:** Link `.wasm` object → final `.wasm` module.

- Detect `--target wasm32` → use `wasm-ld` instead of `gcc`
- WASM flags: `--no-entry`, `--export-all` (export functions), `--allow-undefined`
- Fallback: check for `wasm-ld` availability, error with install hint if missing
- Runtime linking: compile `zeta_runtime_c.c` to `.wasm` object → link together

**Files:** `src/main.rs`

## Phase 3: Runtime for WASM (1 day)

**Goal:** `zeta_runtime_c.c` compiles with `clang --target=wasm32-wasi`.

- Compile: `clang --target=wasm32-wasi -c zeta_runtime_c.c -o zeta_runtime_c_wasm.o`
- Stub out pthread/signal functions for WASM (or use WASI equivalents)
- Thread spawning → no-op (single-threaded fallback)
- `clock_gettime` → `__wasi_clock_time_get`

**Files:** `runtime.c`, `runtime_wasi_stubs.c`

## Phase 4: Platform Guards (1 day)

**Goal:** Compiler compiles on WASM host without panicking on unsupported features.

- JIT: `finalize_and_jit` returns error on WASM with "JIT not supported for WASM target"
- Actor scheduler: `#[cfg(not(target_arch = "wasm32"))]` around `std::thread::spawn`
- HTTP/TLS host functions: `#[cfg(not(target_arch = "wasm32"))]` with WASM stubs returning error

**Files:** `src/backend/codegen/jit.rs`, `src/runtime/actor/scheduler.rs`, `src/runtime/host.rs`

## Phase 5: Size Optimization (1 day)

**Goal:** Minimal `.wasm` binary size.

- `wasm-opt -Oz` post-processing pass (from Binaryen)
- Strip unused runtime functions via `--lto-style` dead code elimination
- Default binary target: < 10KB for simple programs (vs 50KB+ for Rust/C Emscripten)

**Files:** `src/main.rs`

## Phase 6: Test & Verify (1 day)

**Goal:** End-to-end `.wasm` compilation and execution verified.

- `wasmtime hello.wasm` — runs via WASM runtime
- `node --experimental-wasm-modules hello.wasm` — runs in Node.js
- Test: simple prime counter, SIMD (if enabled), string operations
- Benchmark `.wasm` binary size vs equivalent C/Rust

**No files — testing only.**

## Phase 7: WASM REPL / Playground (3 days)

**Goal:** Interactive Zeta REPL in the browser via WASM.

- Compile the Zeta compiler itself to WASM (`zeta.wasm`)
- Compile a simple REPL wrapper that reads Zeta source → compiles → returns output
- Host at `zeta-lang.org/play` or similar
- Shareable links with embedded source code

**Files:** `src/main.rs` (REPL path), `www/` (public directory with HTML/JS)
