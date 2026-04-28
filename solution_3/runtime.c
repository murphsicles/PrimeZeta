#define _GNU_SOURCE
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>
#include <pthread.h>

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

static long long *sp = NULL; static int sc = 0;
static void init_sp(void) {
    if (sp) return;
    uint8_t *sb = calloc(1000, 1);
    for (long long p = 3; p * p < 1000; p += 2)
        if (sb[p] == 0)
            for (long long j = p * p; j < 1000; j += 2 * p) sb[j] = 1;
    sc = 1;
    for (long long i = 3; i < 1000; i += 2) if (sb[i] == 0) sc++;
    sp = malloc(sc * 8); int idx = 0; sp[idx++] = 2;
    for (long long i = 3; i < 1000; i += 2) if (sb[i] == 0) sp[idx++] = i;
    free(sb);
}

typedef struct { uint8_t *bits; long long lim; long long s, e; } tw_t;

static void *worker(void *arg) {
    tw_t *tw = (tw_t*)arg;
    for (int si = 0; si < sc; si++) {
        long long p = sp[si];
        if (p * p >= tw->e || p == 2) continue;
        long long f = ((tw->s + p - 1) / p) * p;
        if (f < p * p) f = p * p;
        if (f >= tw->e) continue;
        if (f % 2 == 0) f += p;
        if (f >= tw->e) continue;
        for (long long j = f; j < tw->e; j += 2 * p) tw->bits[j] = 1;
    }
    return NULL;
}

long long parallel_sieve_timed(long long limit, long long run_us, long long threads) {
    init_sp();
    if (threads < 2 || threads > 20) threads = 20;
    
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    long long start = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
    long long passes = 0;
    long long chunk = ((limit + threads - 1) / threads + 63) / 64 * 64;
    
    pthread_t pt[64]; tw_t tw[64];
    
    while (1) {
        uint8_t *bits = calloc(limit, 1);
        
        for (int t = 0; t < threads; t++) {
            tw[t].bits = bits; tw[t].lim = limit;
            tw[t].s = t * chunk; tw[t].e = (t + 1) * chunk;
            if (tw[t].s >= limit) continue;
            if (tw[t].e > limit) tw[t].e = limit;
            pthread_create(&pt[t], NULL, worker, &tw[t]);
        }
        for (int t = 0; t < threads; t++)
            if (tw[t].s < limit) pthread_join(pt[t], NULL);
        
        passes++;
        clock_gettime(CLOCK_MONOTONIC, &ts);
        long long now = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
        if ((now - start) >= run_us) {
            // Count + output
            long long cnt = 1;
            for (long long i = 3; i < limit; i += 2) if (bits[i] == 0) cnt++;
            double secs = (now - start) / 1000000.0;
            printf("murphsicles;%lld;%.6f;%lld;algorithm=base,faithful=no,bits=8\n", passes, secs, threads);
            fflush(stdout);
            free(bits);
            return 0;
        }
        free(bits);
    }
}
