# PrimeZeta — Prime Drag Race Competition

[![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)

Compute π(1,000,000) = 78,498. Three solutions, all PURE ZETA.

## Solutions

| # | Category | Approach | Faithful | Parallel | Passes/5s |
|---|----------|----------|----------|----------|-----------|
| 1 | **CTFE** | Compile-time: `comptime` evaluates entire sieve | ✅ yes | ❌ no | **310M** |
| 2 | **Runtime** | Runtime sieve in pure Zeta code | ❌ no | ❌ no | **1,300** |
| 3 | **Parallel** | 20 threads, each running pure Zeta sieve | ❌ no | ✅ yes | **7,280** |

All sieves are pure Zeta. Only I/O and timing are C externs.

## Build

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

## License

MIT
