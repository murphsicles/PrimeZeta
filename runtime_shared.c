#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include <string.h>

// Get time in microseconds
long long get_time_us(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000000LL + (long long)ts.tv_nsec / 1000LL;
}

// Print competition result line
// Format: <label>;<iterations>;<total_time>;<num_threads>;<tags>
void print_result(const char *label, long long passes, double elapsed, int threads, const char *tags) {
    printf("%s;%lld;%.6f;%d;%s\n", label, passes, elapsed, threads, tags);
    fflush(stdout);
}

// Zeta bridge functions
void *runtime_malloc(long long size) { return malloc(size); }
void runtime_free(void *ptr) { free(ptr); }

// Array runtime stubs
long long array_get(long long ary, long long idx) { return ((long long*)ary)[idx]; }
long long stack_array_get(long long base, long long idx) { return ((long long*)base)[idx]; }
void array_set(long long ary, long long idx, long long val) { ((long long*)ary)[idx] = val; }
void stack_array_set(long long base, long long idx, long long val) { ((long long*)base)[idx] = val; }
long long array_push(long long ary, long long val) { return ary; }
void array_free(long long ary) { free((void*)ary); }
void array_set_len(long long ary, long long len) {}

void println_i64(long long value) {
    printf("%lld\n", value);
    fflush(stdout);
}
