# Zeta Compiler — Recovery Sprint Plan

**Objective:** Close the ~80 gaps identified in the audit, targeting self-hosting bootstrap.
**Method:** Iterative v0.8.x releases. Each successful push = new release.
**Cadence:** Push often. Run `cargo fmt && cargo clippy` before every push.
**Status:** ✅ = Done, 🔄 = In Progress, ⬜ = Pending

---

## Phase 1: Quick Wins — AST → MIR Lowering (Highest Impact)

The 22 AST node types that have NO MIR lowering. Adding them is the single biggest step toward self-hosting.

### Priority A: Core Type Definitions (Self-Hosting Critical)

- [x] `AstNode::StructDef` — Struct definitions (needed for stdlib types)
- [x] `AstNode::EnumDef` — Enum definitions (needed for Option, Result)
- [x] `AstNode::ImplBlock` — Implementation blocks (needed for methods)
- [x] `AstNode::ConceptDef` — Trait/concept definitions (needed for stdlib traits)
- [x] `AstNode::TypeAlias` — Type aliases (needed for type reorganization)

### Priority B: Pattern Matching (Self-Hosting Critical)

- [x] `AstNode::IfLet` — If-let pattern matching
- [x] `AstNode::StructPattern` — Struct destructuring in patterns
- [x] `AstNode::OrPattern` — Pattern alternatives `|`
- [x] `AstNode::BindPattern` — Binding patterns `x @`
- [x] `AstNode::RangePattern` — Range patterns `1..=10`
- [x] `AstNode::Ignore` — Wildcard pattern `_`
- [x] `AstNode::Tuple` — Tuple expressions/patterns
- [x] Improve `AstNode::Match` — Handle non-literal patterns

### Priority C: Module System (Self-Hosting Critical)

- [x] `AstNode::Use` — Import/use declarations
- [x] `AstNode::ModDef` — Module definitions
- [x] Fix `module_resolver.rs:1104` — `serde_json::from_str` stub

### Priority D: Control Flow & Expressions

- [x] `AstNode::Loop` — Infinite loop construct
- [x] `AstNode::Closure` — Lambda/closure expressions
- [x] `AstNode::Defer` — Deferred execution
- [x] `AstNode::Await` — Async/await expressions
- [x] `AstNode::Bool` — Boolean literal (currently via Lit(1)/Lit(0))
- [x] `AstNode::ExternFunc` — FFI declaration lowering
- [x] `AstNode::AssociatedType` — Associated types in concepts

### Priority E: Concurrency & Security

- [x] `AstNode::Spawn` — Actor spawn
- [x] `AstNode::TimingOwned` — Constant-time operations

---

## Phase 2: Codegen Completeness

- [x] `MirStmt::Consume` — Add codegen handler
- [x] `collect_ids_from_expr_safe` — Fix `_ => {}` fallthrough for 10+ MirExpr types
- [x] Fix `collect_ids_from_stmt_safe` — Add handler for new MirStmt variants
- [x] Verify all MirStmt variants have codegen handlers (incl. StructNew)

---

## Phase 3: Type System Completeness

- [x] `TraitBound::Future` — Implement proper trait check (AsyncFunction + Future named types)
- [x] `Type::from_string` — Fix identity type parsing stub
- [x] `instantiate_generic_with_bounds` — Implement bounds checking
- [x] `resolver.rs:146-147` — Handle generics and lifetimes in name resolution
- [x] Integrate new/old typecheckers (`unified_typecheck.rs`)

---

## Phase 4: Cleanup & Quality

- [x] Remove `eprintln!` debug calls from production code (62 occurrences)
- [x] Remove commented-out debug statements from parser
- [x] Clean up `zeta_runtime.rs` stub functions
- [x] Remove dead code in blockchain/distributed modules (or gate behind feature flags)

---

## Phase 5: Runtime Completeness

- [x] Uncomment atomic host functions in `sync.rs`
- [x] Implement async runtime (`host_async_spawn`, `host_async_await`)
- [x] Fix heap allocation tracking (`host.rs:612`)

---

## Phase 6: Performance Optimization

- [x] Array init: use memset for zero-initialized arrays (`mir/gen.rs:1581`)
- [x] Reduce alloca usage for simple intermediate values
- [x] Enable for-loop over collections (not just ranges)

---

## Phase 7: Codegen Quality

- [x] Bool/char small-type optimization (not always i64)
- [x] Improve pointer passing (avoid inttoptr/ptrtoint round-trips)
- [x] Add LLVM intrinsic for memset/memcpy
- [x] MIR optimizer: loop until convergence

---

## Release History

| Version | Date | Changes |
|---------|------|---------|
| v0.8.0 | Apr 28 | Inline array access, CTFE sprint |
| v0.8.1 | Apr 29 | LLVM -O3 pass pipeline |
| v0.8.2 | Apr 29 | `__builtin_ctpop` POPCNT intrinsic |
| v0.8.3 | Apr 29 | V4I64 vector types, AVX2 intrinsics |
| v0.8.4 | Apr 29 | Break/continue support |
| v0.8.5 | Apr 29 | AST→MIR lowering: StructDef, EnumDef, ImplBlock, ConceptDef, TypeAlias, Method, AssociatedType, ExternFunc |
| v0.8.6 | Apr 29 | AST→MIR lowering: pattern matching (IfLet, OrPattern, BindPattern, RangePattern, Ignore, Tuple, Match improvements) |
| v0.8.7 | Apr 29 | AST→MIR lowering: remaining 12 nodes (Use, ModDef, Closure, Defer, Await, Spawn, TimingOwned, module_resolver fix) |
| v0.8.8 | Apr 29 | Codegen completeness — Consume handler, collect_ids fix, MirStmt verification
| v0.8.9 | Apr 29 | TraitBound::Future — non-hardcoded trait check |
| v0.8.10 | Apr 29 | Type System Completeness — identity parsing, bounds checking, generics in resolver, unified typechecker |
| v0.8.11 | Apr 29 | Cleanup & Quality — parser debug removal, zeta_runtime stubs, type_cache cleanup |
| v0.8.12 | Apr 29 | Runtime Completeness — atomic host functions, async spawn/await, heap allocation tracking |
| v0.8.13 | Apr 29 | Performance Optimization — memset for zero arrays, collection for-loops |
| v0.8.14 | Apr 29 | Codegen Quality — LLVM memset intrinsic, MIR optimizer convergence |

---

## Best Practices

- Run `cargo fmt` before every commit
- Run `cargo clippy` before every commit
- Verify `cargo build --release` succeeds
- Test with existing working programs (min_test.z, prime solutions)
- Push to dev branch on every completed task
- Create GitHub release with notes after each push
- Tag as v0.8.x (incrementing minor)
