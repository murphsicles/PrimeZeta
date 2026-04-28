# PrimeZeta — Prime Drag Race Competition

[![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)

Compute π(1,000,000) = 78,498. Three solutions, each optimized for its category.

## Solutions

| # | Category | Approach | Faithful | Parallel | Passes/5s |
|---|----------|----------|----------|----------|-----------|
| 1 | **CTFE** | Compile-time via `comptime` — sieve runs at build | ✅ yes | ❌ no | **313.9M** |
| 2 | **Runtime** | Byte-array sieve via extern FFI (8 bits/flag) | ❌ no | ❌ no | **~9K** |
| 3 | **Parallel** | Multi-threaded via pthreads (20 threads) | ❌ no | ✅ yes | **~8K** |

## Build

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

## License

MIT
