# PrimeZeta — Prime Drag Race Competition Entry

Compute π(1,000,000) = 78,498. Two solutions showcasing Zeta.

## Solutions

| # | Name | Approach | Faithful | Passes/5s |
|---|------|----------|----------|-----------|
| 1 | **CTFE** | Compile-time via `comptime` — sieve runs at build time | ✅ yes | ~308M |
| 2 | **C-Accelerated** | Runtime sieve via extern FFI | ❌ no | ~8K |

## Build

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

## License

MIT
