# stdlib Sprint — Tier 1→3 Shipping Plan

**Goal:** Ship Zeta's standard library modules that programmers actually need, tier-by-tier, each as a tagged release.

**Order:** Tier 1 → Tier 2 → Tier 3, strict dependency order. No skipping ahead.

---

## Tier 1 — Fundamentals (v0.9.0)

Release scope: `std::mem`, `std::ptr`, `std::cmp::Ordering` + comparison fns, `std::hash::Hash`, `std::iter` adapters.

These are the building blocks every generic collection and data structure needs. Without them, every programmer writes the same glue.

### Tasks

| # | Module | What | Why |
|---|--------|------|-----|
| 1 | `std::mem` | `size_of<T>`, `align_of<T>`, `offset_of<T, field>`, `swap`, `replace`, `drop`, `needs_drop` | Compile-time type introspection. LLVM already knows these — surface them as intrinsics with Zeta signatures. |
| 2 | `std::ptr` | `null<T>`, `is_null`, `addr_of`, `addr_of_mut`, `read`, `write`, `offset`, `add`, `sub`, `copy`, `drop_in_place` | Raw pointer operations — currently every zeta programmer has to declare their own `extern` bindings. |
| 3 | `std::cmp` | `Ordering` enum (`Less`/`Equal`/`Greater`), `Ord` trait, `Eq` trait, `PartialOrd`/`PartialEq`, min/max, clamp, reverse, `then_with`, `by_key` variants | Needed for every sort, comparison, and ordered collection. Build on Zeta's concept system. |
| 4 | `std::hash` | `Hash` trait, `Hasher` trait, default hasher (SipHash-1-3), `hash` method | Your HashMap works on i64 keys because Rust backs it. For `HashMap<MyStruct, V>` this is required. |
| 5 | `std::iter` | `Iterator` trait: `next`, `map`, `filter`, `fold`, `for_each`, `collect`, `zip`, `chain`, `enumerate`, `skip`, `take`, `skip_while`, `take_while`, `find`, `position`, `any`, `all`, `count`, `last`, `nth`, `max_by`, `min_by`, `partition`, `flatten`, `flat_map` | Compose-able iterator adapters. You have the concept hierarchy (source/successor/advance) but no functional chain API. This is the gap between "I can iterate" and "I can write readable data pipelines." |

### Definition of Done
- Each module compiles in Zeta (`.z` files)
- Tests exist for each function (at least happy path)
- A Zeta program can do: `vec![3, 1, 4, 1, 5].iter().filter(|x| x > 2).map(|x| x * 2).collect::<Vec<i64>>()`
- A Zeta program can sort a `Vec<CustomStruct>` using a custom `cmp` / `Hash` implementation
- Tag: `v0.9.0-stdlib-tier1`

---

## Tier 2 — I/O & Networking (v0.10.0)

Release scope: full `std::fs`, `std::path`, `std::net::TcpStream`, atomics (`std::sync::atomic`).

### Tasks

| # | Module | What | Why |
|--|--------|------|-----|
| 1 | `std::fs` (full) | `read`, `write`, `metadata`, `create_dir`, `create_dir_all`, `remove_file`, `remove_dir`, `rename`, `copy`, `read_dir`, `canonicalize`, `exists` | Beyond basic file open/read/write. Navigation and metadata matters for every non-trivial project. Your compiler itself will need this. |
| 2 | `std::path` | `Path` type, `PathBuf` type, `join`, `parent`, `file_name`, `extension`, `with_extension`, `is_absolute`, `components`, `as_os_str` | Without this, everyone reinvents with string ops and gets OS path separator bugs. Depends on proper `String` type. |
| 3 | `std::net` | `TcpStream` (connect, read, write, peer_addr, local_addr), `TcpListener` (bind, accept, incoming), `Shutdown` | You have `http_get` — a single-shot convenience. For a blockchain language where nodes communicate over TCP, you need sockets. |
| 4 | `std::sync::atomic` | `AtomicBool`, `AtomicI32`, `AtomicI64`, `AtomicU64`: `load`, `store`, `swap`, `compare_exchange`, `fetch_add`, `fetch_sub`, `fetch_and`, `fetch_or`, memory ordering | Your scheduler uses these internally; expose them. Needed for lock-free data structures without pulling in the actor runtime. |

### Definition of Done
- File: create, write, read, rename, list directory, delete — full cycle works
- Path: join two paths, extract extension, get parent — cross-platform
- TcpStream: echo server round-trip compiles and passes
- Atomic: concurrent counter with fetch_add, load/store
- Tag: `v0.10.0-stdlib-tier2`

---

## Tier 3 — Ergonomics (v0.11.0)

Release scope: `std::char`, `std::fmt::Display`/`Debug`, `std::time::Duration`, `std::io::BufRead`/`Seek`, `std::thread` (lightweight), `std::process`.

### Tasks

| # | Module | What | Why |
|--|--------|------|-----|
| 1 | `std::char` | `is_digit`, `is_alphabetic`, `is_alphanumeric`, `is_lowercase`, `is_uppercase`, `is_whitespace`, `to_lowercase`, `to_uppercase`, `from_u32` | You have string-level case ops; char-level predicates are finer-grained and used in parsing. |
| 2 | `std::fmt::Display` + `Debug` | `Display` trait (user display), `Debug` trait (programmer display), `format!` macro, write!-style API, `fmt::write` | You have `println` as a host function. Trait-based formatting means `println("{}", my_struct)` just works. |
| 3 | `std::time::Duration` | `Duration::new`, `from_secs`, `from_millis`, `from_micros`, `from_nanos`, `as_secs`, `as_millis`, `as_micros`, `as_nanos`, arithmetic (`+`, `-`, `*`, `/`), comparison | You have `system_time_now`/`elapsed`. `Duration` arithmetic makes timing and timeout code readable. |
| 4 | `std::io::BufRead` + `Seek` | `BufReader` (buffered file/stream reading), `BufWriter` (buffered writing), `Seek` (rewind, forward by position) | Without buffering, every read is a syscall. Matters for file-heavy workloads. |
| 5 | `std::process` | `Command::new`, `arg`, `args`, `output`, `status`, `stdout`/`stderr` capture | Running external commands (build tools, git hooks, etc.) is a common need. Lightweight — thin wrapper over libc fork+exec or posix_spawn. |
| 6 | `std::thread` (lightweight) | `spawn`, `join`, `sleep`, `yield_now` | Your actor model is the primary concurrency story. A lightweight `spawn` that maps to a new actor (not necessarily an OS thread) gives programmers a simple escape hatch for parallel work without committing to the full actor runtime. |

### Definition of Done
- `println!("{}", point)` works via `Display` trait impl
- `Duration::from_secs(5) + Duration::from_millis(500)` compiles and produces correct result
- `BufReader` on a file reads line-by-line without per-call syscall overhead
- `Command::new("ls").output()` compiles and returns stdout
- `thread::spawn() + join()` compiles and runs
- Tag: `v0.11.0-stdlib-tier3`

---

## Rules of the Sprint

1. **Strict ordering** — Tier 1 first, then Tier 2, then Tier 3. Tier 1 modules can ship independently within the tier; Tier 2/3 depend on Tier 1 being complete.
2. **Each tier ships as a tagged release** — `v0.9.0`, `v0.10.0`, `v0.11.0`.
3. **No feature creep** — Each module gets the listed functions and no more. Add on demand.
4. **Tests required** — Each function needs at minimum one happy-path test in `zeta_src/tests/`.
5. **Existing host layer is your friend** — Over the Rust host primitives you already have (`src/runtime/`, `src/std/`) rather than rewriting everything in Zeta immediately. The Zeta `.z` files can delegate to `extern` host functions where a Rust implementation already exists. Refactor to pure Zeta later.
6. **Ship fast, iterate later** — "Works" beats "perfect". Get the shape right, test it, ship it.

---

*Start with Tier 1 Module 1: `std::mem`. Kick off by dropping the functions in `zeta_src/runtime/mem.z` (or wherever fits the module layout) and wiring them through the compiler's intrinsic/module resolution.*
