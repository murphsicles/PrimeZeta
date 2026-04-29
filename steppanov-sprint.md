# Stepanov Sprint — Elements of Programming in Zeta

**Goal:** Implement Alexander Stepanov's foundational ideas from *Elements of Programming* into Zeta, enabling provably correct, mathematically grounded generic programming.

**Scope:** Concepts, Regular types, Iterator hierarchy, algorithmic refinement, swap, type traits, structured results.

**Status: ✅ ALL 6 PHASES COMPLETE**

---

## Phase 1: Foundations — Regular Types & swap (v0.9.0)

### 1.1 `Regular(T)` concept derivation
- Automatically derive whether a type satisfies the Regular concept:
  - Default-constructible
  - Copy-constructible (has `clone` or copy semantics)
  - Destructible
  - Equality-comparable (`==` / `!=`)
  - Total ordering (`<`, `>`, `<=`, `>=` when meaningful)
- Add `regular` built-in concept to the type system
- Compile-time check: `static_assert(regular(T))`

### 1.2 Built-in `swap` operation
- `swap(a, b)` as a language primitive (not a library function)
- For stack values: bitwise swap via LLVM `memcpy` or XOR swap
- For heap values: pointer swap
- Implementation: add `__builtin_swap` intrinsic in codegen

### 1.3 Value semantics guarantee
- Clarify that `let b = a` copies the value (independent object)
- Add `move(a)` for explicit transfer of ownership
- Add `copy(a)` for explicit deep copy

**Files to modify:**
- `src/middle/types/mod.rs` — Regular concept derivation
- `src/middle/mir/gen.rs` — swap lowering
- `src/backend/codegen/codegen.rs` — `__builtin_swap` codegen

---

## Phase 2: Type Traits (v0.9.1)

### 2.1 Built-in type traits
- `value_type(T)` — the element type for iterators/containers
- `difference_type(T)` — the type for distance between iterators
- `iterator_category(T)` — input/forward/bidirectional/random_access
- `is_regular(T)` — true if T satisfies Regular concept
- `is_totally_ordered(T)` — true if T has consistent <, >, <=, >=
- `is_integer(T)` — true for i8/i16/i32/i64/usize
- `is_floating_point(T)` — true for f32/f64

### 2.2 Compile-time trait queries
- `trait::value_type<Container>` — get the element type
- `trait::is_same<T, U>` — type equality check at compile time
- `trait::enable_if<condition, T>` — SFINAE-like constraint

**Files to modify:**
- `src/middle/types/mod.rs` — Add trait query types
- `src/frontend/parser/parser.rs` — Parse `trait::` syntax
- `src/backend/codegen/codegen.rs` — Codegen for trait queries

---

## Phase 3: Concept Satisfaction Checking (v0.9.2)

### 3.1 Concept satisfaction at compile time
- When a function is constrained by a concept, verify the argument type satisfies all requirements
- Example: `fn sort(seq: Vector<T>) where T: TotallyOrdered`
- `TotallyOrdered` requires: `==`, `!=`, `<`, `<=`, `>`, `>=` all defined and consistent

### 3.2 Concept-based dispatch
- Multiple implementations of the same algorithm for different concept refinements
- Example: `advance(it, n)` — different implementation for ForwardIterator vs RandomAccessIterator
- Compile-time selection based on concept hierarchy

### 3.3 Refinement hierarchy
Implement Stepanov's concept lattice:
```
Regular
├── TotallyOrdered    (has <, >, <=, >=)
├── Semigroup         (has associative +)
├── Monoid            (Semigroup + identity element)
├── Group             (Monoid + inverse)
└── Ring              (Group + associative *)
```

**Files to modify:**
- `src/middle/resolver/` — Concept checking during type resolution
- `src/middle/types/mod.rs` — Concept hierarchy definitions
- `src/frontend/ast.rs` — Concept bounds on generics

---

## Phase 4: Iterator Hierarchy (v0.9.3)

### 4.1 Iterator categories
- `InputIterator<T>` — can read, single-pass
- `OutputIterator<T>` — can write, single-pass
- `ForwardIterator<T>` — multi-pass, can save and restore position
- `BidirectionalIterator<T>` — Forward + can go backward (`--`)
- `RandomAccessIterator<T>` — Bidirectional + can jump in O(1) (`+= n`)

### 4.2 Iterator traits
- `value_type<It>` → `T`
- `difference_type<It>` → `i64`
- `iterator_category<It>` → one of the category tags

### 4.3 Iterator operations
- `source(it)` — read value at iterator (dereference)
- `sink(it, val)` — write value at iterator
- `successor(it)` — advance by one (`++`)
- `predecessor(it)` — go back by one (`--`), for Bidirectional
- `advance(it, n)` — advance by n steps (specialized by category)

### 4.4 Range concept
- `Range<R>` — has `begin(R)` and `end(R)` returning iterators
- All containers satisfy Range
- Algorithms operate on ranges, not containers

**Files to modify:**
- `src/frontend/ast.rs` — Iterator expressions
- `src/middle/mir/gen.rs` — Iterator operation lowering
- `src/backend/codegen/codegen.rs` — Iterator codegen
- `src/middle/types/mod.rs` — Iterator category types

---

## Phase 5: Algorithmic Refinement (v0.9.4)

### 5.1 Concept-constrained algorithms
Implement Stepanov's fundamental algorithms with concept constraints:

```zeta
fn find<I: InputIterator>(first: I, last: I, val: value_type<I>) -> I
fn find_if<I: InputIterator>(first: I, last: I, pred: UnaryFunction<bool, value_type<I>>) -> I
fn count<I: InputIterator>(first: I, last: I, val: value_type<I>) -> difference_type<I>
fn copy<I: InputIterator, O: OutputIterator>(first: I, last: I, out: O) -> O
fn swap_ranges<I: ForwardIterator, O: ForwardIterator>(first: I, last: I, out: O) -> O
```

### 5.2 Refinement-based optimization
- `advance(it, n)` — default: loop n times (for InputIterator)
- `advance(it, n)` — optimized: `it += n` (for RandomAccessIterator)
- Compile-time dispatch based on `iterator_category<It>`

### 5.3 Structured results
- Algorithms that return multiple values use tuple/pair:
- `find` returns `(iterator, found_flag)`
- `minmax` returns `(min, max)`

**Files to modify:**
- Standard library: `zeta_src/` directory
- `src/middle/resolver/` — Concept-based dispatch

---

## Phase 6: Provable Correctness (v0.9.5)

### 6.1 Precondition and postcondition assertions
- `pre(condition)` — assert at function entry
- `post(condition)` — assert at function exit
- Both are checked at runtime in debug mode, elided in release

### 6.2 Loop invariants
- `invariant(condition)` — asserted at loop entry/exit
- Helps verify algorithm correctness

### 6.3 Mathematical type properties
- `commutative(Op)` — true if `a op b == b op a`
- `associative(Op)` — true if `(a op b) op c == a op (b op c)`
- `identity_element(Op)` → e such that `a op e == a`

**Files to modify:**
- `src/frontend/parser/parser.rs` — Parse `pre`/`post`/`invariant`
- `src/middle/mir/gen.rs` — Lower to assertion MIR
- `src/backend/codegen/codegen.rs` — Assertion codegen

---

## Release Plan

| Version | Phase | Focus | ETA |
|---------|-------|-------|-----|
| v0.9.0 | Phase 1 | Regular types, swap, value semantics | ✅ Complete |
| v0.9.1 | Phase 2 | Type traits (value_type, is_regular, etc.) | ✅ Complete |
| v0.9.2 | Phase 3 | Concept satisfaction checking + refinement hierarchy | ✅ Complete |
| v0.9.3 | Phase 4 | Iterator hierarchy + Range concept | ✅ Complete |
| v0.9.4 | Phase 5 | Algorithmic refinement + structured results | ✅ Complete |
| v0.9.5 | Phase 6 | Provable correctness (pre/post/invariant) | ✅ Complete |

---

## Success Criteria

1. ✅ `Type::is_regular()` method — auto-derives Regular concept from type properties
2. ✅ `__builtin_swap(a_ptr, b_ptr, size)` — LLVM memcpy-based swap intrinsic in MIR + codegen
3. ✅ `trait::value_type<T>` parsed and lowered to TraitResult at compile time
4. ✅ Concept satisfaction checking via `satisfies_concept()` + `check_concept_satisfaction()`
5. ✅ Iterator operations (source, sink, successor, predecessor, advance, begin, end) in MIR gen
6. ✅ `pre(cond)`, `post(cond)`, `invariant(cond)` assertions with runtime failure handling
7. ✅ The entire pipeline compiles, `cargo fmt` + `clippy` clean (no errors)
