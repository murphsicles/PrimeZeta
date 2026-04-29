#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// ---------------------------------------------------
// Zeta bridge functions (needed for linking)
// ---------------------------------------------------
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

// ---------------------------------------------------
// Optimized sieve runtime (same code as original 4,200-pass version)
// ---------------------------------------------------

// Fill memory with byte value (AVX2 or fallback)
void avx512_byte_fill(long long ptr, long long val, long long count) {
    memset((void*)ptr, (int)val, (size_t)count);
}

// Count set bits using POPCNT
long long avx512_count_bits(long long ptr, long long words) {
    uint64_t *p = (uint64_t*)ptr;
    long long count = 0;
    for (long long i = 0; i < words; i++) {
        count += __builtin_popcountll(p[i]);
    }
    return count;
}

// Test if bit is set (non-volatile)
long long test_bit(long long ptr, long long index) {
    uint64_t *words = (uint64_t*)ptr;
    return (words[index / 64] >> (index % 64)) & 1;
}

// Sieve step: clear multiples of p in the bit array, word-level batched
// stride = 2*p (skip even multiples, base algorithm compliant)
void sieve_step(long long ptr, long long limit, long long p) {
    uint64_t *bits = (uint64_t*)ptr;
    long long start = p * p;
    
    // Batched clearing: for each word touched, build a mask of all bits
    // that need clearing in that word, then apply once with AND
    long long w = start / 64;
    long long b = start % 64;
    long long end_word = (limit - 1) / 64;
    long long stride = 2 * p;  // skip even multiples
    
    // Process word by word within range
    uint64_t bit = 1ULL << b;
    bits[w] &= ~bit;
    long long current = start + stride;
    
    while (current < limit) {
        long long new_word = current / 64;
        long long new_bit = current % 64;
        if (new_word != w) {
            // Moved to next word, start fresh
            w = new_word;
            bit = 1ULL << new_bit;
            bits[w] &= ~bit;
        } else {
            // Same word, batch into existing mask
            bit |= (1ULL << new_bit);
            bits[w] &= ~bit;
            // Reset bit for next batch
            bit = 0;
        }
        current += stride;
    }
}

// Main sieve entry point: run classic sieve up to N
long long run_sieve(long long limit) {
    long long word_count = (limit + 63) / 64;
    uint64_t *bits = (uint64_t*)malloc(word_count * sizeof(uint64_t));
    if (!bits) return 0;
    
    // Initialize: all bits = 1 (assume all numbers are prime)
    for (long long i = 0; i < word_count; i++) bits[i] = ~0ULL;
    
    // Clear 0 and 1
    bits[0] &= ~(1ULL << 0);
    if (limit > 1) bits[0] &= ~(1ULL << 1);
    
    // Clear all even numbers (base algorithm only checks odd candidates)
    // Use word-level batch: every even bit in every word
    for (long long w = 0; w < word_count; w++) {
        uint64_t even_mask = 0;
        for (int b = 0; b < 64; b += 2) {
            uint64_t idx = w * 64 + b;
            if (idx >= 4 && idx < (unsigned long long)limit) {
                even_mask |= (1ULL << b);
            }
        }
        bits[w] &= ~even_mask;
    }
    
    // Main sieve: check all odd numbers starting at 3
    for (long long p = 3; p * p < limit; p += 2) {
        if ((bits[p / 64] >> (p % 64)) & 1) {
            sieve_step((long long)bits, limit, p);
        }
    }
    
    // Count primes using POPCNT
    long long result = avx512_count_bits((long long)bits, word_count);
    free(bits);
    return result;
}

// ---------------------------------------------------
// Timing and result output (competition format)
// ---------------------------------------------------
long long get_time_us(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000000LL + (long long)ts.tv_nsec / 1000LL;
}

long long time_is_up(long long start_us, long long target_us) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    long long now = (long long)ts.tv_sec * 1000000LL + (long long)ts.tv_nsec / 1000LL;
    return (now - start_us) >= target_us ? 1 : 0;
}

void print_result(long long passes, long long elapsed_us) {
    double secs = (double)elapsed_us / 1000000.0;
    printf("murphsicles;%lld;%.6f;1;algorithm=other,faithful=no,bits=1\n", passes, secs);
    fflush(stdout);
}
