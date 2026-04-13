# WORK QUEUE - Zeta Bootstrap Project

## Current Status: v0.3.85 - STABLE (April 13, 2026 - 19:00 UTC)

**COMPILER STATUS**: ✅ **ZERO WARNINGS** - All 241 warnings eliminated (100% reduction)
**LIBRARY TESTS**: ✅ **106/106 PASSING**
**FULL TEST SUITE**: ✅ **185+ tests, 0 failures** - All test suites green
**HEAP ALLOCATION**: ✅ **WORKING** - Sieve of Eratosthenes verified up to 1,000,000
**VERSION**: v0.3.85

## Day Summary (April 13, 2026)

Massive progress day: **v0.3.78 → v0.3.85** in one day.

### Milestones Achieved:
- **v0.3.79-v0.3.81**: Warning cleanup (241 → 1 warning, 99.6% reduction)
- **v0.3.82**: Zero warnings milestone + full test suite green (185/185)
- **v0.3.83**: Flaky CSE test fixed (HashMap ordering), release tags v0.3.81 & v0.3.82 created
- **v0.3.84**: Critical parser bugs fixed (comparison operators, comment skipping, boolean NOT, CTFE arrays)
- **v0.3.85**: Heap allocation working + while-condition re-evaluation bug fixed

### Critical Bugs Fixed in v0.3.85:
1. **Heap array runtime linkage** - `array_set_len`, `stack_array_get`, `stack_array_set` were missing from JIT global mappings
2. **While-loop condition re-evaluation** - Complex expressions in while conditions (e.g., `i * i < limit`) were only evaluated once, causing infinite loops. Fixed by capturing condition computation statements and re-executing them in the condition basic block each iteration.
3. **Void function recognition** - `array_set`, `array_set_len`, `array_free`, `stack_array_set` were being lowered as value-returning `MirStmt::Call` instead of `MirStmt::VoidCall`, causing stack corruption when called from user code.
4. **Missing `array_set_len` runtime function** - Implemented in `src/runtime/array.rs` with proper bounds checking against capacity.
5. **Missing `stack_array_get`/`stack_array_set` runtime functions** - Added for completeness (normally inlined via GEP by codegen).

### Architectural Fix: MIR While Loop
- Added `cond_stmts: Vec<MirStmt>` field to `MirStmt::While`
- Condition computation statements are now captured during MIR lowering
- Codegen re-executes condition statements in the while.cond basic block each iteration
- This fixes all complex while conditions (multiplication, function calls, etc.)

### Verification:
- ✅ `array_new(10)` + `array_set_len` + `array_set` + `array_get` + `array_free` = 150
- ✅ Heap sieve: 10 primes below 30
- ✅ Heap sieve: 168 primes below 1,000
- ✅ Heap sieve: 78,498 primes below 1,000,000 (π(10^6) exact)
- ✅ While condition `i * i < limit` works correctly (was infinite loop before)
- ✅ All 185+ existing tests pass, 0 regressions

## Next Version: v0.3.86 - Competition Readiness

### Priority 1: Array/Slice Type Coercion
- **Why**: Arrays don't coerce to slices in function arguments
- **What**: Add implicit coercion from `[N]T` to `&[T]` in function call contexts
- **Status**: Not started

### Priority 2: 30030-Wheel Algorithm Implementation
- **Why**: Competition domination - targeting C #1 (~12,451 passes/5s)
- **What**: Implement Father's 30030-wheel algorithm with CTFE-generated residue tables
- **Depends on**: Heap allocation (✅ DONE), array/slice coercion
- **Status**: Algorithm documented, compiler support ready

### Priority 3: AVX-512 SIMD Support
- **Why**: Maximum sieve performance
- **What**: SIMD intrinsics for bit manipulation in sieve
- **Status**: SIMD functions implemented in compiler, need sieve-specific usage

### Priority 4: Performance Profiling & Benchmarking
- **Why**: Validate competition performance claims
- **What**: Benchmark against C reference implementations
- **Status**: Not started

## Release Tags
- **v0.3.81**: Warning cleanup milestone (240/241 fixed) - tagged at e4b17f38
- **v0.3.82**: Zero warnings + full green test suite - tagged at 140a8313
- **v0.3.84**: Critical parser/codegen fixes
- **v0.3.85**: Heap allocation + while-condition fix (latest on main)

## Architecture Notes
- **CTFE**: Uses `comptime` keyword (not `const`)
- **Array syntax**: `[N]T` (converted to Rust `[T; N]` internally)
- **Mutability**: `let mut` syntax (like Rust)
- **Loops**: `while (condition)` requires parentheses
- **Turbofish**: Required for type args in expression context (like Rust)
- **Heap arrays**: `array_new(capacity)` → `array_set_len(arr, len)` → `array_get`/`array_set` → `array_free`
- **While conditions**: `cond_stmts` in MIR ensures re-evaluation each iteration
