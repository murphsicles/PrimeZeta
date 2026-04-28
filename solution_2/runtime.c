#define _GNU_SOURCE
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

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

// Pattern: all odd bits = 1, all even bits = 0
// Bit position k = 1 iff k is odd
#define ODD_MASK 0xAAAAAAAAAAAAAAAAULL

// Optimized sieve with pre-cleared evens
long long run_sieve(long long limit) {
    long long words = (limit + 63) / 64;
    uint64_t *restrict bits = calloc(words, sizeof(uint64_t));
    
    // Init: all odd numbers marked prime, evens cleared
    // 0xAA... = bit 2i+1 = 1 (odds prime), bit 2i = 0 (evens not prime)
    for (long long i = 0; i < words; i++) bits[i] = ODD_MASK;
    
    // Clear bits 0 and 1 (not prime), SET bit 2 (prime 2)
    bits[0] = (ODD_MASK & ~3ULL) | (1ULL << 2);
    if (limit % 64) {
        long long last_word = words - 1;
        bits[last_word] &= ~(~0ULL << (limit % 64));
    }
    
    // Main sieve: odd numbers starting at 3
    for (long long p = 3; p * p < limit; p += 2) {
        if ((bits[p / 64] >> (p % 64)) & 1) {
            // Clear multiples of p with stride = 2p (skip evens)
            long long start = p * p;
            long long w = start / 64;
            long long b = start % 64;
            uint64_t mask = 1ULL << b;
            
            for (long long j = start + p * 2; j < limit; j += p * 2) {
                long long nw = j / 64;
                long long nb = j % 64;
                if (nw != w) {
                    bits[w] &= ~mask;
                    w = nw;
                    mask = 1ULL << nb;
                } else {
                    mask |= 1ULL << nb;
                    if (nb == 63) { bits[w] &= ~mask; mask = 0; }
                }
            }
            if (mask) bits[w] &= ~mask;
        }
    }
    
    // Count using POPCNT  
    long long count = 0;
    for (long long i = 0; i < words; i++)
        count += __builtin_popcountll(bits[i]);
    
    free(bits);
    return count;
}

// Timing
long long get_time_us(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
}
long long time_is_up(long long s, long long t) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ((ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL) - s) >= t ? 1 : 0;
}
void print_result(long long passes, long long elapsed_us) {
    double secs = (double)elapsed_us / 1000000.0;
    printf("murphsicles;%lld;%.6f;1;algorithm=other,faithful=no,bits=1\n", passes, secs);
    fflush(stdout);
}
