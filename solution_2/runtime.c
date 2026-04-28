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

// Integer sqrt (Newton's method, no math library needed)
static long long isqrt(long long n) {
    if (n <= 1) return n;
    long long x = n, y = (x + n / x) / 2;
    while (y < x) { x = y; y = (x + n / x) / 2; }
    return x;
}

// Pre-cached small primes
static long long *sp = NULL;
static int sc = 0;

static void ensure_sp(void) {
    if (sp) return;
    uint8_t *sb = calloc(1000, 1);
    for (long long p = 3; p * p < 1000; p += 2)
        if (sb[p] == 0)
            for (long long j = p * p; j < 1000; j += 2 * p)
                sb[j] = 1;
    sc = 1;
    for (long long i = 3; i < 1000; i += 2)
        if (sb[i] == 0) sc++;
    sp = malloc(sc * 8);
    int idx = 0; sp[idx++] = 2;
    for (long long i = 3; i < 1000; i += 2)
        if (sb[i] == 0) sp[idx++] = i;
    free(sb);
}

// Byte-array sieve (8 bits/flag, like Zig)
long long run_sieve(long long limit) {
    ensure_sp();
    uint8_t *bits = calloc(limit, 1);
    long long q = isqrt(limit);
    
    for (int si = 0; si < sc; si++) {
        long long p = sp[si];
        if (p * p >= limit) break;
        if (p == 2) continue;
        for (long long j = p * p; j < limit; j += 2 * p)
            bits[j] = 1;
    }
    
    long long count = 1;
    for (long long i = 3; i < limit; i += 2)
        if (bits[i] == 0) count++;
    
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
    struct timespec ts; clock_gettime(CLOCK_MONOTONIC, &ts);
    long long n = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
    return (n - s) >= t ? 1 : 0;
}
void print_result(long long passes, long long elapsed_us) {
    printf("murphsicles;%lld;%.6f;1;algorithm=base,faithful=no,bits=8\n", passes, elapsed_us / 1000000.0);
    fflush(stdout);
}
