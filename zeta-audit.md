# Zeta Compiler Audit

## Executive Summary

**Overall health: ~55% complete for self-hosting bootstrap.**

The Zeta compiler is a sophisticated, ambitiously multi-paradigm compiler with a full pipeline (parser → AST → MIR → LLVM codegen). It has strong foundations for numeric computation and basic control flow, but has **significant gaps in type system completeness, codegen coverage, runtime support, and self-hosting capability**. The alloca-heavy IR generation strategy (every local variable gets a stack slot) is functional but far from optimal.

---

## Critical Gaps (Block Self-Hosting)

These are the items that prevent Zeta from compiling its own source code.

### 1. AST → MIR Lowering Gaps (src/middle/mir/gen.rs)

Several AST node types are **not lowered to MIR at all** — they fall through to `_ => {}` at line 1745, producing zero MIR:

| AST Node | What It Does | Why It Blocks Self-Hosting |
|---|---|---|
| `AstNode::ConceptDef` | Trait/concept definitions | Zeta stdlib uses concepts/traits |
| `AstNode::ImplBlock` | Trait implementations | Zeta stdlib uses impl blocks |
| `AstNode::StructDef` | Struct definitions | Self-hosting needs to define structs in stdlib |
| `AstNode::EnumDef` | Enum definitions | Self-hosting needs enums (Option, Result) |
| `AstNode::ExternFunc` | FFI declarations | Needed for C interop |
| `AstNode::TypeAlias` | Type aliases | Needed for stdlib type reorganization |
| `AstNode::AssociatedType` | Associated types in concepts | Needed for trait system |
| `AstNode::Await` | Async/await expressions | Needed for async stdlib |
| `AstNode::IfLet` | If-let pattern matching | Pattern matching is used heavily |
| `AstNode::StructPattern` | Struct destructuring | Needed for pattern-based code |
| `AstNode::Defer` | Deferred execution | Used in resource cleanup |
| `AstNode::Spawn` | Actor spawning | Core language feature |
| `AstNode::TimingOwned` | Constant-time operations | Security feature |
| `AstNode::Closure` | Closure/lambda expressions | Heavily used in modern code |
| `AstNode::Bool` | Boolean literal | Actually handled via Lit(1)/Lit(0) as workaround |
| `AstNode::Loop` | Infinite loops | Used in some patterns |
| `AstNode::Ignore` | Wildcard pattern `_` | Used in pattern matching destructuring |
| `AstNode::Tuple` | Tuple expressions/patterns | Used in multi-return |
| `AstNode::OrPattern` | Pattern alternatives `\|` | Pattern matching |
| `AstNode::BindPattern` | Binding patterns `x @` | Pattern matching |
| `AstNode::RangePattern` | Range patterns `1..=10` | Pattern matching |
| `AstNode::Use` | Module imports | Self-hosting needs module system |
| `AstNode::ModDef` | Module definitions | Core for project structure |

### 2. AST → MIR: Incomplete Lowering for Existing Nodes

Even for AST nodes that ARE handled, many cases are incomplete:

- **`AstNode::Match`** (line 1110): Only handles `Lit` patterns and `Var("_")` wildcard. Missing: tuple patterns, struct patterns, nested patterns, guard clauses, range patterns, or-patterns.
- **`AstNode::Let` destructuring** (line 131): Only handles `AstNode::Var(name)` and `AstNode::TypeAnnotatedPattern { .. Var }`. Cannot destructure tuples, structs, or arrays.
- **`AstNode::For` loops** (line 514): Only handles range-based `BinaryOp { ".." }` and `Range` expressions. Cannot iterate over collections.
- **`AstNode::ArrayRepeat`** (line 1534): Only handles literal sizes. Cannot handle const generic sizes.
- **`AstNode::Unsafe`** (line 1731): Falls through to `_ => {}` with only a comment.

### 3. Module System / Resolver: Stub Generators (src/middle/resolver/module_resolver.rs)

The module resolver generates **stubs for serde_json** and other modules instead of real implementations:

- **Line 1104**: `serde_json::from_str` has `unimplemented!()` — panics at runtime
- **Line 1108+**: Generic fallback creates empty stubs for unknown modules
- Zorb package manager integration (`src/package/`) has extensive TODO stubs for dependency resolution (line 149), build process (line 179), testing (line 197), and publishing (line 217)

### 4. Type System: Unification Gaps (src/middle/types/mod.rs)

- **`TraitBound::Future`** (line 1716): Returns hardcoded `false` — `TODO: Implement Future trait check`
- **`Type::from_string` identity parsing** (line 285): `TODO: Implement proper parsing` — creates empty identity types
- **`instantiate_generic_with_bounds` bounds checking** (line 1583): `TODO: Check bounds when we have proper TypeVar -> TypeParam mapping`

### 5. Codegen: Missing MIR Statement Handlers

The `gen_stmt` function at line 1727 handles most MirStmt variants, but the **`MirStmt::Consume` variant has no handler in `gen_stmt`** — it falls through to `_ => {}`. This is used by the borrow checker for ownership tracking.

### 6. Codegen: The `_` Fallthrough for Statements (line 2649)

The `gen_fn` function's `collect_ids_from_stmt_safe` has `_ => {}` at line 1193, meaning if a new MirStmt variant is added without updating this, it silently skips ID collection, leading to missing allocas and use-of-uninitialized-memory bugs.

Similarly, `collect_ids_from_expr_safe` at line 1245 also has `_ => {}` with no handler for `MirExpr::Struct`, `MirExpr::FieldAccess`, `MirExpr::As`, `MirExpr::StackArray`, `MirExpr::Range`, `MirExpr::BinaryOp` (in the expression version), `MirExpr::ConstEval`, `MirExpr::StringLit`, `MirExpr::Lit`, or `MirExpr::SemiringFold`.

---

## Important Gaps (Performance / Correctness)

### 1. Alloca-Heavy IR Generation (Known Issue)

- **src/backend/codegen/codegen.rs lines 1028-1040**: Every local variable gets an `alloca` — even intermediate expression results. This means:
  - Every arithmetic operation: store result to alloca, then load from alloca for next use
  - No SSA value propagation between operations
  - Significant missed optimization opportunity (LLVM can optimize some, but not all)
  - Function arguments are stored to alloca then loaded (even for simple forwarding)

### 2. i64-Only Type System in Codegen

- **src/backend/codegen/codegen.rs line 1028**: All values are `i64` type in LLVM. This means:
  - Booleans waste 63 bits per value
  - Character operations may have incorrect semantics
  - All pointers are passed as `i64` (requires inttoptr/ptrtoint for every access)
  - Struct fields are individually loaded/stored as `i64`
  - No optimization for small integer types

### 3. Async Runtime (src/runtime/async.rs)

- **Line 72**: `host_async_spawn` — `TODO: Implement async task spawning` — returns input as dummy
- **Line 80**: `host_async_await` — `TODO: Implement await` — returns input as dummy

These make async/await completely non-functional at runtime.

### 4. Atomic Operations Disabled (src/runtime/sync.rs)

- **Lines 142-209**: All atomic host functions are **commented out** (`host_atomic_bool_new`, `host_atomic_bool_load`, `host_atomic_bool_store`, `host_atomic_usize_new`, `host_atomic_usize_load`, `host_atomic_usize_store`, `host_atomic_usize_fetch_add`, `host_atomic_usize_fetch_sub`). The Rust implementations exist but have no FFI entry points.

### 5. Dead Code: zeta_runtime.rs Stubs

**src/runtime/zeta_runtime.rs**: Contains 7 stub functions (lines 1-61) all marked `// TODO: Implement` — these are never called from the actual pipeline and appear to be dead code/experimental:
- `zeta_array_get_bool`, `zeta_array_get_i64`
- `zeta_array_set_bool`, `zeta_array_set_i64`
- `zeta_print_i64`, `zeta_println_i64`
- `zeta_sieve_new`

### 6. Macro Expansion Limitations (src/frontend/macro_expand.rs, macro_expand_advanced.rs)

- `macro_expand.rs:324`: `// TODO: Handle literal identifiers in expansion`
- `macro_expand.rs:330`: `// TODO: Parse literal value`
- `macro_expand.rs:622`: `// TODO: Parse impl_str into AST nodes` (block-level macro expansion)
- `macro_expand_advanced.rs:473`: `// TODO: Handle other AST node types`
- `macro_expand_advanced.rs:521`: `// TODO: Implement proper format string expansion`
- `macro_expand_advanced.rs:585`: `// TODO: Implement proper pattern matching with hygiene`
- `macro_expand_advanced.rs:596`: `// TODO: Implement proper expansion with hygiene`

### 7. Proc Macro System (src/frontend/proc_macro.rs)

- Line 255: `depth: 0, // TODO: Track scope depth`
- Line 353: `module_path: vec!["crate".to_string()], // TODO: Get actual module path`
- Line 355: `type_info: HashMap::new(), // TODO: Populate with actual type info`
- Line 681: `// TODO: Handle fields in enum variants`

### 8. MIR Generator: CTFE consts Type

**src/middle/mir/gen.rs line 18**: `ctfe_consts: HashMap<u32, i64>, // TODO: Change to ConstValue` — constant values are limited to i64, cannot hold strings, arrays, or structs as compile-time constants.

### 9. Resolver Integration (src/middle/resolver/unified_typecheck.rs)

- Lines 69, 74: Two separate `// TODO: Integrate` comments — the unified typechecker bridges old and new resolver but integration is incomplete, with the old typecheck being used as fallback.

---

## Minor Gaps (Polish / Edge Cases)

### 1. Debug Prints in Production Code

**src/runtime/memory.rs, memory_enhanced.rs, memory_bulletproof.rs**: `eprintln!("[BULLETPROOF MEMORY] ...")` — debug prints in production runtime code.

**src/runtime/identity/integration.rs**: `eprintln!("[IDENTITY ERROR] ...")` — error logging in identity host functions.

**src/frontend/borrow_enhanced.rs**: 14 `eprintln!("Borrow error: ...")` calls — production error logging via stderr.

**src/middle/resolver/typecheck.rs**: 20 `eprintln!` calls — debug tracing left in production code.

### 2. Commented-Out Debug Statements

**src/frontend/parser/parser.rs lines 672-685**: 3 commented-out `eprintln!` debug statements with note "Disabled for performance".

**src/middle/resolver/typecheck_new.rs line 79**: Commented-out debug eprintln.

### 3. Block-Level Parser: Logical Operators Disabled

**src/frontend/parser/parser.rs line 72**: `// TODO: re-add these when we implement logical operators` — operators like `->`, `=>`, etc. are explicitly disabled in the block parser.

### 4. Module Resolver: Generics and Lifetimes Stubbed

**src/middle/resolver/resolver.rs lines 146-147**:
```rust
generics: vec![], // TODO: Handle generics
lifetimes: vec![], // TODO: Handle lifetimes
```

This means generic type parameters and lifetime annotations are discarded during name resolution.

### 5. Type Inference: Element Type Assumed

**src/middle/resolver/typecheck.rs line 314**: `// TODO: Actually infer the element type from the base` — subscript element type inference is stubbed.

### 6. New Resolver: Compile-Time Evaluation Stub

**src/middle/resolver/new_resolver.rs lines 824-826**: `// TODO: Implement proper compile-time evaluation checking` — with a non-fatal eprint warning.

### 7. Struct Literal Type Inference in MIR

**src/middle/mir/gen.rs line 1410**: `// TODO: Need proper type inference for struct literals` — struct literal field types default to `Type::I64` when type info is unavailable.

### 8. Array Initialization Not Optimized

**src/middle/mir/gen.rs line 1581**: `// TODO: Optimize with memset for zero initialization` — zero-initialized arrays produce N store instructions instead of a single memset.

### 9. Codegen: Range Type Implementation

**src/backend/codegen/codegen.rs line 3008**: `// TODO: Implement proper range type` — range expressions in codegen just return the start value.

### 10. Codegen: Type Conversion Stub

**src/backend/codegen/codegen.rs line 2934**: `// TODO: Implement proper type conversion logic` — type conversions truncate/extend but don't handle signedness, float conversions, or pointer conversions properly.

### 11. Iterator Support Stub

**src/middle/mir/gen.rs line 536**: `// TODO: Handle other iterator types` — only range-based for loops work.

### 12. Blockchain Module: All Network Operations Stubbed

- `src/blockchain/bsv/network.rs`: 12 `// TODO` stubs — all network operations (broadcast, query, balance, fee estimation, etc.) are stubs
- `src/blockchain/bsv/mining.rs`: 10 `// TODO` stubs — all mining operations are stubs
- `src/blockchain/bsv/transaction.rs`: 2 `// TODO` stubs — transaction signing and verification
- `src/blockchain/bsv/script.rs`: 1 `// TODO` stub — script execution
- `src/blockchain/solana/account.rs`: 3 `// TODO` stubs
- `src/blockchain/solana/transaction.rs`: 4 `// TODO` stubs (signing, creation, fee calc)
- `src/blockchain/solana/address.rs`: 1 `// TODO` stub (PDA derivation)

### 13. Distributed Computing: All Operations Functional but Basic

The distributed module (`src/distributed/`) has real implementations but they are minimal and likely not production-ready.

### 14. Heap Allocation Tracking Issue

**src/runtime/host.rs line 612**: `// TODO: Need to track allocation size to free properly` — heap allocations may not be tracking sizes correctly for free operations.

---

## By Category

### Parser (src/frontend/parser/)
- **Strengths**: Expression parsing is robust with operator precedence, closures, blocks, match expressions. Statement parsing covers most control flow (let, if, while, for, loop, unsafe, comptime).
- **Gaps**: Logical operators disabled in block parser (line 72). Pattern parsing may have limitations.

### AST (src/frontend/ast.rs)
- **Strengths**: Comprehensive AST with 45+ node types covering concepts/traits, generics, lifetimes, FFI, pattern matching, concurrency, identity types.
- **Gaps**: Many AST nodes have no MIR lowering (see Critical Gaps #1).

### MIR Gen (src/middle/mir/gen.rs)
- **Strengths**: Good lowering for basic control flow (if/while/for loops), binary ops, function calls, basic pattern matching (match with lit/variable patterns), struct literals, array literals.
- **Gaps**: ~22 AST node types not lowered (see Critical Gaps). Incomplete pattern matching. 0 type-based dispatch.

### MIR Types (src/middle/mir/mir.rs)
- **Strengths**: Well-designed MIR with statement/expression separation. Covers most needed constructs.
- **Gaps**: `Consume` stmt has no codegen handler. `StructNew` in MirStmt but `Struct` in MirExpr — codegen uses `Struct` variant from expressions, not `StructNew` from statements.

### Type System (src/middle/types/mod.rs)
- **Strengths**: Rich type representation with full algebraic types, generics, type variables, unification, trait bound checks, higher-kinded types, identity types.
- **Gaps**: Trait bound checks for Future is hardcoded false. Identity type parsing is a stub. Generic bounds checking in instantiation is a TODO. Trait resolution is incomplete.

### CTFE (src/middle/ctfe/)
- **Strengths**: Full evaluator (`evaluator.rs` + `evaluator_complete.rs`) with AST visitor pattern. Handles const folding for basic arithmetic and string ops. Complete evaluator extends with recursive function evaluation.
- **Gaps**: CTFE evaluation targets limited const expressions. No MIR-level constant propagation.

### Codegen (src/backend/codegen/codegen.rs)
- **Strengths**: Full LLVM codegen covering basic ops, comparisons, SIMD V4I64 intrinsics, struct field access, if/while/for control flow, function calls, monomorphization, type conversion.
- **Gaps**: Alloca-heavy strategy (see Important #1), i64-only types (see Important #2), Consume handler missing, MIR statement `_` fallthrough.

### Optimization (src/middle/optimization.rs)
- **Strengths**: Dead code elimination, constant folding, CSE, strength reduction, algebraic simplification — all properly implemented at MIR level with test coverage.
- **Gaps**: Optimization passes run in sequence but don't loop to convergence (except O3 does 3 iterations). No inlining pass. No loop invariant code motion.

### Runtime - C (runtime.c)
- **Tiny** (50 lines): Provides only what Murphy's Sieve needs — timing, printing, array bridges, malloc/free.
- **Gaps**: No string support (concat, length, comparison). No formatted I/O. No file I/O. No networking.

### Runtime - Rust (src/runtime/)
- **Strengths**: Rich set of host functions (strings, options, maps, vectors, mpsc channels, SIMD vectors).
- **Gaps**: Async both stubbed. Atomic host functions commented out. zeta_runtime.rs is dead stub code.

### Distributed / Blockchain / ML / Consciousness
- These modules exist but are experimental/recreational and not part of the core compiler pipeline. They won't block self-hosting but represent ~30% of the codebase.

---

## Recommendations (Priority-Ordered)

### P0 — Required for Self-Hosting

1. **Lower ConceptDef, ImplBlock, StructDef, EnumDef to MIR**: These are essential for defining the types and traits that the Zeta stdlib (and compiler itself) uses. The MIR generator needs `lower_ast` handlers for these.

2. **Lower pattern matching variants**: Implement lowering for `IfLet`, `StructPattern`, `OrPattern`, `BindPattern`, `RangePattern`, `Ignore` to support the full pattern matching needed by compiler code.

3. **Add Consume codegen handler**: Without this, the language's ownership semantics don't work at runtime.

4. **Complete Module system**: Module resolution (`Use`, `ModDef`) and import resolution are needed to organize self-hosted compiler source.

5. **Enable serde_json::from_str**: The `unimplemented!()` at `module_resolver.rs:1104` needs an actual implementation or at minimum a stub that returns an error instead of panicking.

### P1 — Important for Correctness and Performance

6. **Integrate new/old typecheckers** (`unified_typecheck.rs` TODO at lines 69/74): The type system split between `resolver.rs`/`typecheck.rs` (old) and `new_resolver.rs`/`typecheck_new.rs` (new) creates code duplication and inconsistent behavior.

7. **Fix `collect_ids_from_expr_safe`**: The `_ => {}` fallthrough misses Struct, FieldAccess, As, StackArray, Range, BinaryOp, ConstEval, StringLit, Lit, SemiringFold — these need proper ID collection for correct alloca generation.

8. **Implement Future trait checking**: The hardcoded `false` at `types/mod.rs:1716` makes async function signatures always fail type checking.

9. **Reduce alloca usage**: Start with function arguments and common intermediate results, converting them to direct SSA values before falling back to alloca.

### P2 — Quality of Life

10. **Enable atomic operations**: Uncomment the FFI functions in `sync.rs` or provide an alternative implementation.

11. **Implement async runtime**: `host_async_spawn` and `host_async_await` need real implementations.

12. **Remove debug eprintln calls**: Especially in `borrow_enhanced.rs`, `memory*.rs`, and `typecheck.rs` — replace with proper diagnostic infrastructure or gate behind feature flags.

13. **Clean up dead code**: `zeta_runtime.rs` stubs should be removed or properly integrated.

14. **Improve iterator support**: `mir/gen.rs:536 TODO` — support for-loop over collections.

15. **Optimize array initialization**: `mir/gen.rs:1581 TODO` — use memset for zero-initialized arrays.

---

## Audit Methodology

- Every `.rs` file in `src/` was searched for `todo!()`, `unimplemented!()`, `// TODO`, `// FIXME`, `// HACK`, `eprintln!()`, and `dbg!()` calls
- AST node definitions were cross-referenced against MIR generation handlers
- MIR type variants were cross-referenced against codegen handlers
- Type system completeness was assessed via trailing match arms
- Runtime C/Rust files were checked for missing FFI exports
- Total findings: **~80 issues** across 10 categories
