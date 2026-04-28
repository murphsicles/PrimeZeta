#include <stdio.h>
#include <time.h>

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
    printf("murphsicles;%lld;%.6f;1;algorithm=base,faithful=yes,bits=1\n", passes, secs);
    fflush(stdout);
}
