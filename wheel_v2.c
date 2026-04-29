#define _GNU_SOURCE
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// Zeta bridge + timing (copied from solution_1 runtime.c)
void *runtime_malloc(long long s) { return malloc(s); }
void runtime_free(void *p) { free(p); }
long long array_get(long long a, long long i) { return ((long long*)a)[i]; }
void array_set(long long a, long long i, long long v) { ((long long*)a)[i] = v; }
long long stack_array_get(long long b, long long i) { return ((long long*)b)[i]; }
void stack_array_set(long long b, long long i, long long v) { ((long long*)b)[i] = v; }
long long array_push(long long a, long long v) { return a; }
void array_free(long long a) {}
void array_set_len(long long a, long long l) {}
void println_i64(long long v) { printf("%lld\n", v); fflush(stdout); }

// Precomputed 30030 wheel: 5760 residues coprime to 2,3,5,7,11,13
static const unsigned char wheel_inc[5760]; // defined below

// Murphy's Sieve with 30030 wheel
long long run_sieve(long long limit) {
    long long words = (limit + 63) / 64;
    uint64_t *bits = calloc(words, sizeof(uint64_t));
    for (long long i = 0; i < words; i++) bits[i] = ~0ULL;
    
    // Clear 0 and 1
    bits[0] &= ~3ULL;
    
    // Wheel-walk: check each wheel residue for primality,
    // clear its multiples with stride = p (full stride, wheel handles small factor skipping)
    long long sqrt_limit = 1;
    while (sqrt_limit * sqrt_limit < limit) sqrt_limit++;
    
    int wi = 0;
    long long p = 1; // first wheel residue
    while (p * p < limit) {
        if (p >= 2 && ((bits[p / 64] >> (p % 64)) & 1)) {
            // Clear multiples of p with stride = p
            uint64_t start = p * p;
            uint64_t w = start / 64;
            uint64_t bit = 1ULL << (start % 64);
            
            for (uint64_t j = start + p; j < (uint64_t)limit; j += p) {
                uint64_t nw = j / 64;
                uint64_t nb = j % 64;
                if (nw != w) {
                    bits[w] &= ~bit;
                    w = nw;
                    bit = 1ULL << nb;
                } else {
                    bit |= 1ULL << nb;
                    if (nb == 63) { bits[w] &= ~bit; bit = 0; }
                }
            }
            if (bit) bits[w] &= ~bit;
        }
        p += wheel_inc[wi];
        if (++wi >= 5760) { wi = 0; }
    }
    
    // Count using POPCNT
    long long count = 0;
    for (long long i = 0; i < words; i++)
        count += __builtin_popcountll(bits[i]);
    
    free(bits);
    return count;
}

// Wheel increments table (5760 entries)
// Computed as gaps between consecutive numbers coprime to 30030
static void build_wheel(void) {
    // Static initialization - fill once
    static int built = 0;
    static unsigned char inc[5760];
    if (built) return;
    
    int idx = 0, prev = 1;
    for (int i = 3; i < 30030; i += 2) {
        if (i % 3 == 0 || i % 5 == 0 || i % 7 == 0 || i % 11 == 0 || i % 13 == 0)
            continue;
        inc[idx++] = (unsigned char)(i - prev);
        prev = i;
    }
    inc[5759] = 30030 - prev; // wrap to 1
    memcpy((void*)wheel_inc, inc, sizeof(inc));
    built = 1;
}

// Need __attribute__((constructor)) or explicit init
// Let me put it in run_sieve
long long run_sieve(long long limit) {
    static int wheel_built = 0;
    if (!wheel_built) { build_wheel(); wheel_built = 1; }
    // ... rest of function
}

// Actually I already wrote the function above. Let me use a constructor.
__attribute__((constructor)) static void init(void) { build_wheel(); }
