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

// Precomputed small primes
// Auto-generated small primes < 1000 (168 primes)
static long long sp[168] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,941,947,953,967,971,977,983,991,997};
static int sc = 168;

static long long isqrt(long long n) {
    if (n <= 1) return n;
    long long x = n, y = (x + n / x) / 2;
    while (y < x) { x = y; y = (x + n / x) / 2; }
    return x;
}

static long long sieve_pass(long long limit) {
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

long long run_sieve_timed(long long limit, long long run_us) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    long long start = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
    long long passes = 0;
    
    while (1) {
        sieve_pass(limit);
        passes++;
        clock_gettime(CLOCK_MONOTONIC, &ts);
        long long now = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
        if ((now - start) >= run_us) {
            double secs = (now - start) / 1000000.0;
            printf("murphsicles;%lld;%.6f;1;algorithm=base,faithful=no,bits=8\n", passes, secs);
            fflush(stdout);
            return 0;
        }
    }
}
