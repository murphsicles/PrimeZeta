#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <stdint.h>

// These are called BY NAME from the Zeta-generated LLVM IR
// They CANNOT be inlined since they're in a separate compilation unit
long long array_get(long long a, long long i) { return ((long long*)a)[i]; }
void array_set(long long a, long long i, long long v) { ((long long*)a)[i] = v; }
long long stack_array_get(long long b, long long i) { return ((long long*)b)[i]; }
void stack_array_set(long long b, long long i, long long v) { ((long long*)b)[i] = v; }
long long array_push(long long a, long long v) { return a; }
void array_free(long long a) {}
void array_set_len(long long a, long long l) {}
void println_i64(long long v) { printf("%lld\n", v); fflush(stdout); }
void *runtime_malloc(long long s) { return malloc(s); }
void runtime_free(void *p) { free(p); }

long long get_time_us(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
}
long long time_is_up(long long start, long long target) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    long long now = ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
    return (now - start) >= target ? 1 : 0;
}
void print_result(long long passes, long long elapsed_us) {
    double secs = (double)elapsed_us / 1000000.0;
    printf("murphsicles;%lld;%.6f;1;algorithm=base,faithful=no,bits=1\n", passes, secs);
    fflush(stdout);
}
