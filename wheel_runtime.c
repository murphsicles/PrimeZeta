#define _GNU_SOURCE
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// Zeta bridge
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

// ====== 30030 WHEEL ======
// 30030 = 2*3*5*7*11*13, phi(30030) = 5760 wheel spokes
static int wheel[5760];     // wheel residues (coprime to 30030)
static int increments[5760]; // gaps between consecutive wheel residues
static int wheel_initialized = 0;

static void init_wheel(void) {
    if (wheel_initialized) return;
    int idx = 0;
    for (int i = 1; i < 30030; i += 2) { // only odd numbers
        if (i % 3 == 0 || i % 5 == 0 || i % 7 == 0 || i % 11 == 0 || i % 13 == 0)
            continue;
        wheel[idx] = i;
        if (idx > 0) increments[idx - 1] = i - wheel[idx - 1];
        idx++;
    }
    increments[5759] = 30030 - wheel[5759]; // wrap-around
    wheel_initialized = 1;
}

// Popcount for a word
static inline int popcnt(uint64_t x) {
    return __builtin_popcountll(x);
}

// Run Murphy's Sieve with 30030 wheel
long long run_sieve(long long limit) {
    init_wheel();
    long long words = (limit + 63) / 64;
    uint64_t *bits = calloc(words, sizeof(uint64_t));
    if (!bits) return 0;
    
    // Init all bits = 1
    for (long long i = 0; i < words; i++) bits[i] = ~0ULL;
    
    // Clear 0,1
    bits[0] &= ~3ULL;
    
    // Pre-sieve: clear multiples of 2,3,5,7,11,13 up to limit
    int small[] = {2,3,5,7,11,13};
    for (int si = 0; si < 6; si++) {
        int sp = small[si];
        long long start = sp * 2;
        if (start >= limit) continue;
        // Word-level batch: stride = sp
        long long w = start / 64;
        long long b = start % 64;
        uint64_t mask = 1ULL << b;
        for (long long j = start + sp; j < limit; j += sp) {
            long long nw = j / 64;
            long long nb = j % 64;
            if (nw != w) {
                bits[w] &= ~mask;
                w = nw;
                mask = 1ULL << nb;
            } else {
                mask |= 1ULL << nb;
                if (nb == 63) { bits[w] &= ~mask; mask = 0; w++; }
            }
        }
        if (mask) bits[w] &= ~mask;
    }
    
    // Main sieve: walk wheel residues
    long long sqrt_limit = 1;
    while (sqrt_limit * sqrt_limit < limit) sqrt_limit++;
    
    int wi = 0;
    long long p = wheel[0];
    while (p * p < limit) {
        if ((bits[p / 64] >> (p % 64)) & 1) {
            // Clear multiples of p: j = p*p, stride = p (since wheel already clears small ones)
            long long start = p * p;
            long long w = start / 64;
            long long b = start % 64;
            uint64_t mask = 1ULL << b;
            long long current = start + p;
            while (current < limit) {
                long long nw = current / 64;
                long long nb = current % 64;
                if (nw != w) {
                    bits[w] &= ~mask;
                    w = nw;
                    mask = 1ULL << nb;
                } else {
                    mask |= 1ULL << nb;
                    if (nb == 63) { bits[w] &= ~mask; mask = 0; w++; }
                }
                current += p;
            }
            if (mask) bits[w] &= ~mask;
        }
        // Advance to next wheel residue
        p += increments[wi];
        wi++;
        if (wi >= 5760) { wi = 0; p += wheel[0] - 30030; }
    }
    
    // Count using POPCNT
    long long count = 0;
    for (long long i = 0; i < words; i++) count += popcnt(bits[i]);
    
    free(bits);
    return count;
}

// ====== TIMING ======
long long get_time_us(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
}

long long time_is_up(long long start_us, long long target_us) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    long long now = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
    return (now - start_us) >= target_us ? 1 : 0;
}

void print_result(long long passes, long long elapsed_us) {
    double secs = (double)elapsed_us / 1000000.0;
    printf("murphsicles;%lld;%.6f;1;algorithm=wheel,faithful=no,bits=1\n", passes, secs);
    fflush(stdout);
}
