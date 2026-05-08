# String Sprint — Phases 1–8

## Patch v0.9.19 — Phases 1–4 (Quick Wins)

### Phase 1: String Method Syntax
- Fix method dispatch name mapping so `s.len()` → `str_len`, `s.concat()` → `str_concat`, etc.
- Currently resolves to `len_str`, runtime expects `str_len` — codegen alias fix
- **Files:** `src/backend/codegen/codegen.rs`

### Phase 2: Single-quote and Raw Strings
- `'hello'` single-quote string literals
- `r"raw\string"` raw strings (no escape processing)
- **Files:** `src/frontend/parser/expr.rs`

### Phase 3: Dict Literals
- `{"key": value}` syntax — AST node already exists, just needs parser expression
- `DictLit` handled in borrow checker and MIR gen already
- **Files:** `src/frontend/parser/expr.rs`

### Phase 4: `str.split()` and `str.join()`
- Add `host_str_split` and `host_str_join` functions to runtime
- Register in resolver as `str_split`, `str_join`
- Wire method dispatch through the same Phase 1 fix
- **Files:** `src/runtime/host.rs`, `src/middle/resolver/resolver.rs`, `src/backend/codegen/codegen.rs`

---

## Patch v0.9.20 — Phases 5–8 (Deeper Work)

### Phase 5: Triple-quote Strings
- `"""multiline"""` and `'''multiline'''`
- Preserve newlines and indentation
- **Files:** `src/frontend/parser/expr.rs`

### Phase 6: Match as Expression
- Currently match compiles but always returns 0
- Debug the return value chain through MIR → codegen for `Match` AST node
- **Files:** `src/middle/mir/gen.rs`, `src/backend/codegen/codegen.rs`

### Phase 7: More String Host Functions
- `str_strip`, `str_lstrip`, `str_rstrip`
- `str_find`, `str_rfind`, `str_count`
- `str_isalpha`, `str_isnumeric`, `str_isalnum`
- **Files:** `src/runtime/host.rs`, `src/middle/resolver/resolver.rs`

### Phase 8: Span-based Error Diagnostics
- Parser already tracks positions via `nom` error locations
- Create `Diagnostic` struct with span, severity, message
- Surface errors with code snippets and underlines
- Replace raw `eprintln!` / `format!` with proper diagnostics
- **Files:** `src/diagnostics.rs`, `src/lib.rs`, `src/main.rs`
