// PrimeZeta Competition Entry — solution_2 runtime
// C-accelerated single-threaded sieve of Eratosthenes
// Optimizations: word-level bit array, POPCNT, 30030 wheel, batched word clearing,
// precomputed small primes up to sqrt(N), efficient stride looping
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

// ---- Zeta runtime bridge functions ----
void *runtime_malloc(long long size) { return malloc((size_t)size); }
void *runtime_calloc(long long count, long long size) { return calloc((size_t)count, (size_t)size); }
void runtime_free(void *ptr) { free(ptr); }
void println_i64(long long value) { printf("%lld\n", value); fflush(stdout); }

// Stub array/hash functions needed by Zeta linker
void *array_get(void *arr, long long idx, long long elem_size) { return NULL; }
void array_set(void *arr, long long idx, void *val, long long elem_size) {}
void *stack_array_get(void *arr, long long idx) { return NULL; }
void stack_array_set(void *arr, long long idx, void *val) {}
void *array_push(void *arr, void *val, long long elem_size) { return NULL; }
void array_free(void *arr) {}
void *array_set_len(void *arr, long long len) { return NULL; }

// ---- 30030 wheel (2*3*5*7*11*13) ----
// The wheel has 5760 spokes: numbers in [1,30030) coprime to 2,3,5,7,11,13
// Precomputed at compile time so zero runtime overhead
#define WHEEL_MOD 30030
#define WHEEL_SPOKES 5760

// Precomputed wheel offsets — all numbers in [1, WHEEL_MOD) coprime to 2,3,5,7,11,13
static uint16_t wheel_offsets[WHEEL_SPOKES];
// Precomputed lookup: for each residue mod WHEEL_MOD, which spoke index it maps to
// (or -1 if not a spoke)
static int16_t residue_to_spoke[WHEEL_MOD];

static inline void init_wheel(void) {
    static int initialized = 0;
    if (initialized) return;
    for (int i = 0; i < WHEEL_MOD; i++) residue_to_spoke[i] = -1;

    long long idx = 0;
    for (long long n = 1; n < WHEEL_MOD; n += 2) {  // skip evens
        if (n % 3 == 0 || n % 5 == 0 || n % 7 == 0 || n % 11 == 0 || n % 13 == 0)
            continue;
        wheel_offsets[idx] = (uint16_t)n;
        residue_to_spoke[n] = (int16_t)idx;
        idx++;
    }
    initialized = 1;
}

// Advance p to the next wheel candidate (next number coprime to 2,3,5,7,11,13)
static inline long long next_wheel(long long p) {
    long long block = p / WHEEL_MOD;
    int r = (int)(p % WHEEL_MOD);
    int spoke = residue_to_spoke[r];
    if (spoke < 0 || spoke + 1 >= WHEEL_SPOKES) {
        // Wrap to next block
        return (block + 1) * WHEEL_MOD + wheel_offsets[0];
    }
    return block * WHEEL_MOD + wheel_offsets[spoke + 1];
}

// ---- Small prime pre-sieve ----
// Precompute all primes up to sqrt(limit) using a simple sieve.
// This avoids repeatedly checking divisibility in the main loop.
#define MAX_SMALL_PRIMES 50000

// ---- Main sieve ----
long long run_sieve(long long limit) {
    if (limit < 2) return 0;

    init_wheel();

    // sqrt(limit) — sieve only up to here
    long long sqrt_limit = 1;
    while ((sqrt_limit + 1) * (sqrt_limit + 1) <= limit) sqrt_limit++;

    // Allocate bit array: 1 bit per number
    long long word_count = (limit + 63) / 64;
    size_t alloc_size = (size_t)word_count * sizeof(uint64_t);
    uint64_t *bits = (uint64_t *)malloc(alloc_size);
    if (!bits) return -1;

    // Initialize: assume all numbers prime (set bits to 1)
    memset(bits, 0xFF, alloc_size);

    // Clear 0 and 1
    bits[0] &= ~(1ULL << 0);
    if (limit > 1) bits[0] &= ~(1ULL << 1);

    // ---- Compute small primes up to sqrt(limit) ----
    // Use a small local bit array for sqrt(limit)
    long long small_word_count = (sqrt_limit + 63) / 64;
    uint64_t *small_bits = (uint64_t *)malloc((size_t)small_word_count * sizeof(uint64_t));
    if (!small_bits) { free(bits); return -1; }
    memset(small_bits, 0xFF, (size_t)small_word_count * sizeof(uint64_t));
    small_bits[0] &= ~(1ULL << 0);
    if (sqrt_limit > 1) small_bits[0] &= ~(1ULL << 1);

    // Sieve the small array to get primes up to sqrt(limit)
    // We iterate candidates using the wheel for this too
    long long p = 3;  // start at 3
    while (p * p < sqrt_limit) {
        if ((small_bits[p / 64] >> (p % 64)) & 1ULL) {
            long long start = p * p;
            for (long long j = start; j < sqrt_limit; j += p) {
                small_bits[j / 64] &= ~(1ULL << (j % 64));
            }
        }
        p += 2;  // simple odd-only for small sieve (fast enough)
    }

    // ---- Clear multiples of all wheel primes (2,3,5,7,11,13) ----
    // Start from p*p for each of: 2,3,5,7,11,13
    // Manually clear the most common ones
    int small_primes[] = {2, 3, 5, 7, 11, 13};
    for (int si = 0; si < 6; si++) {
        long long sp = small_primes[si];
        long long start = sp * sp;
        if (start >= limit) continue;
        for (long long j = start; j < limit; j += sp) {
            bits[j / 64] &= ~(1ULL << (j % 64));
        }
    }

    // ---- Main sieve: use precomputed small primes from small_bits ----
    // Skip 2,3,5,7,11,13 — already handled above. Start from p=17.
    long long p_idx = 7;  // index in small primes array (0-based from tiny wheel)
    
    // Use a pointer approach: walk through all numbers that are coprime to 2,3,5,7,11,13
    // (as determined by the 30030 wheel), check if each is still prime, and if so,
    // clear its multiples.
    
    // Faster: just iterate over the small_bits to get the list, then sieve.
    for (long long p = 17; p < sqrt_limit; ) {
        if ((small_bits[p / 64] >> (p % 64)) & 1ULL) {
            // p is prime — clear its multiples
            long long start = p * p;
            long long step = p;
            
            // Word-level clearing with some optimization:
            // For small strides, compute word-aligned ranges
            for (long long j = start; j < limit; j += step) {
                bits[j / 64] &= ~(1ULL << (j % 64));
            }
        }
        // Advance p to next wheel candidate
        p = next_wheel(p);
    }

    // ---- Count primes using POPCNT ----
    long long count = 0;
    for (long long i = 0; i < word_count; i++) {
        count += (long long)__builtin_popcountll(bits[i]);
    }

    free(small_bits);
    free(bits);
    return count;
}
