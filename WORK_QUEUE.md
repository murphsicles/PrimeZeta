# WORK QUEUE - Zeta Bootstrap Project

## Current Status: v0.3.84 - STABLE (April 13, 2026 - 19:30 UTC)

**COMPILER STATUS**: ✅ **ZERO WARNINGS** - All 241 warnings eliminated (100% reduction)
**LIBRARY TESTS**: ✅ **106/106 PASSING**
**FULL TEST SUITE**: ✅ **185+ tests, 0 failures** - All test suites green
**GIT STATUS**: ✅ **Clean, synchronized with origin/main**
**VERSION**: v0.3.84 (Cargo.toml + Cargo.lock synced)

## Day Summary (April 13, 2026)

Massive progress day: **v0.3.78 → v0.3.84** in one day.

### Milestones Achieved:
- **v0.3.79-v0.3.81**: Warning cleanup (241 → 1 warning, 99.6% reduction)
- **v0.3.82**: Zero warnings milestone + full test suite green (185/185)
- **v0.3.83**: Flaky CSE test fixed (HashMap ordering), release tags v0.3.81 & v0.3.82 created
- **v0.3.84**: Critical parser bugs fixed (comparison operators, comment skipping, boolean NOT, CTFE arrays)

### Critical Bugs Fixed in v0.3.84:
1. **Comparison operator parsing** - `<` was misinterpreted as generic type args; now requires turbofish `::` syntax
2. **Comment skipping** - Parser `skip_ws_and_comments0` reverted to correctly skip `//` and `/* */`
3. **Boolean NOT operator** - `AstNode::UnaryOp` was not handled in MIR generation (fell through to `Lit(0)`)
4. **CTFE array constants** - Fixed borrow checker error; CTFE now properly embeds array constants in binary

## Next Version: v0.3.85 - Competition Readiness

### Priority 1: Heap Allocation for Large Arrays
- **Why**: 30030-wheel sieve needs arrays larger than stack allows
- **What**: Implement heap-allocated arrays via `std_malloc` / runtime allocation
- **Status**: Not started

### Priority 2: Array/Slice Type Coercion
- **Why**: Arrays don't coerce to slices in function arguments
- **What**: Add implicit coercion from `[N]T` to `&[T]` in function call contexts
- **Status**: Not started

### Priority 3: 30030-Wheel Algorithm Implementation
- **Why**: Competition domination - targeting C #1 (~12,451 passes/5s)
- **What**: Implement Father's 30030-wheel algorithm with CTFE-generated residue tables
- **Depends on**: Heap allocation, array/slice coercion
- **Status**: Algorithm documented, compiler support partially ready

### Priority 4: AVX-512 SIMD Support
- **Why**: Maximum sieve performance
- **What**: SIMD intrinsics for bit manipulation in sieve
- **Status**: SIMD functions implemented in compiler, need sieve-specific usage

### Priority 5: Performance Profiling & Benchmarking
- **Why**: Validate competition performance claims
- **What**: Benchmark against C reference implementations
- **Status**: Not started

## Release Tags
- **v0.3.81**: Warning cleanup milestone (240/241 fixed) - tagged at e4b17f38
- **v0.3.82**: Zero warnings + full green test suite - tagged at 140a8313
- **v0.3.84**: Latest on main (critical parser/codegen fixes)

## Architecture Notes
- **CTFE**: Uses `comptime` keyword (not `const`)
- **Array syntax**: `[N]T` (converted to Rust `[T; N]` internally)
- **Mutability**: `let mut` syntax (like Rust)
- **Loops**: `while (condition)` requires parentheses
- **Turbofish**: Required for type args in expression context (like Rust)
