#define _GNU_SOURCE
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>
#include <pthread.h>

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

// Zeta's sieve function (defined by Zeta compiler, linked from prime.o)
extern long long sieve(long long limit);

// Thread: each runs the Zeta sieve independently
typedef struct { long long result; } tw_t;

static void *worker(void *arg) {
    tw_t *tw = (tw_t*)arg;
    tw->result = sieve(1000000);
    return NULL;
}

long long parallel_sieve_timed(long long limit, long long run_us, long long threads) {
    if (threads < 2 || threads > 20) threads = 20;
    
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    long long start = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
    long long passes = 0;
    
    pthread_t pt[64];
    tw_t tw[64];
    
    while (1) {
        for (int t = 0; t < threads; t++)
            pthread_create(&pt[t], NULL, worker, &tw[t]);
        for (int t = 0; t < threads; t++)
            pthread_join(pt[t], NULL);
        
        // Verify all threads got the right answer
        for (int t = 0; t < threads; t++)
            if (tw[t].result != 78498) return 1;
        
        passes++;
        
        clock_gettime(CLOCK_MONOTONIC, &ts);
        long long now = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
        if ((now - start) >= run_us) {
            double secs = (now - start) / 1000000.0;
            printf("murphsicles;%lld;%.6f;%lld;algorithm=base,faithful=no,bits=1\n", 
                   passes * threads, secs, threads);
            fflush(stdout);
            return 0;
        }
    }
}
