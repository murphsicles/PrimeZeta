#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <pthread.h>

// Zeta runtime stubs
void *runtime_malloc(long long size) { return malloc(size); }
void runtime_free(void *ptr) { free(ptr); }
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

// ---- Prime Sieve ----

// Thread work: each thread sieves & counts a disjoint range [start_word, end_word)
typedef struct {
    uint64_t *bits;             // shared bit array (word-aligned)
    long long  limit;           // total sieve limit (exclusive)
    long long  start_word;      // first word index for this thread
    long long  end_word;        // one-past-last word index
    long long  result;          // prime count from this thread's range
    const long long *small;     // small primes list (read-only, shared)
    long long  nsmall;          // number of small primes
} thread_work_t;

static void *thread_worker(void *arg) {
    thread_work_t *tw = (thread_work_t *)arg;
    long long count = 0;

    // For each small prime, mark its multiples in our word range
    for (long long si = 0; si < tw->nsmall; si++) {
        long long p = tw->small[si];
        // Find first multiple of p within our word range
        // Convert start_word to a bit index
        long long bit_start = tw->start_word * 64;
        long long bit_end   = tw->end_word * 64;
        if (bit_end > tw->limit) bit_end = tw->limit;

        long long first = ((bit_start + p - 1) / p) * p;
        if (first < p * p) first = p * p;
        if (first >= bit_end) continue;

        // Mark multiples within our range
        // Fast 64-bit-aligned inner loop
        for (long long j = first; j < bit_end; j += p) {
            tw->bits[j / 64] &= ~(1ULL << (j % 64));
        }
    }

    // Count primes: POPCNT over every word in our range
    for (long long w = tw->start_word; w < tw->end_word; w++) {
        count += __builtin_popcountll(tw->bits[w]);
    }

    tw->result = count;
    return NULL;
}

long long parallel_sieve(long long limit, long long threads) {
    if (limit < 2) return 0;

    // Clamp threads to reasonable number
    if (threads < 1) threads = 1;
    if (threads > 64) threads = 64;

    long long word_count = (limit + 63) / 64;
    uint64_t *bits = calloc(word_count, sizeof(uint64_t));
    if (!bits) return -1;

    // Init: all bits = 1, then clear 0 and 1
    for (long long i = 0; i < word_count; i++) bits[i] = ~0ULL;
    bits[0] &= ~(1ULL << 0);
    if (limit > 1) bits[0] &= ~(1ULL << 1);

    // ---- Compute small primes up to sqrt(limit) ----
    long long sqrt_limit = 1;
    while ((sqrt_limit + 1) * (sqrt_limit + 1) <= limit) sqrt_limit++;

    uint64_t *small_bits = calloc((sqrt_limit + 63) / 64, sizeof(uint64_t));
    for (long long i = 0; i < (sqrt_limit + 63) / 64; i++) small_bits[i] = ~0ULL;
    small_bits[0] &= ~(1ULL << 0);
    if (sqrt_limit > 1) small_bits[0] &= ~(1ULL << 1);

    for (long long p = 2; p * p < sqrt_limit; p++) {
        if ((small_bits[p / 64] >> (p % 64)) & 1) {
            for (long long j = p * p; j < sqrt_limit; j += p) {
                small_bits[j / 64] &= ~(1ULL << (j % 64));
            }
        }
    }

    // Count small primes
    long long nsmall = 0;
    for (long long i = 2; i < sqrt_limit; i++) {
        if ((small_bits[i / 64] >> (i % 64)) & 1) nsmall++;
    }
    long long *small = malloc(nsmall * sizeof(long long));
    long long idx = 0;
    for (long long i = 2; i < sqrt_limit; i++) {
        if ((small_bits[i / 64] >> (i % 64)) & 1) {
            small[idx++] = i;
        }
    }
    free(small_bits);

    // ---- Split range into threads ----
    // Align splits to word boundaries to avoid data races on shared words
    long long words_per_thread = word_count / threads;
    long long extra_words = word_count % threads;

    pthread_t *threads_arr = malloc(threads * sizeof(pthread_t));
    thread_work_t *work_arr = malloc(threads * sizeof(thread_work_t));

    long long cur_word = 0;
    for (long long t = 0; t < threads; t++) {
        long long nwords = words_per_thread + (t < extra_words ? 1 : 0);
        work_arr[t].bits = bits;
        work_arr[t].limit = limit;
        work_arr[t].start_word = cur_word;
        work_arr[t].end_word = cur_word + nwords;
        work_arr[t].result = 0;
        work_arr[t].small = small;
        work_arr[t].nsmall = nsmall;
        cur_word += nwords;
    }

    // ---- Launch threads ----
    for (long long t = 0; t < threads; t++) {
        pthread_create(&threads_arr[t], NULL, thread_worker, &work_arr[t]);
    }

    // ---- Join and sum ----
    long long total = 0;
    for (long long t = 0; t < threads; t++) {
        pthread_join(threads_arr[t], NULL);
        total += work_arr[t].result;
    }

    // Cleanup
    free(bits);
    free(small);
    free(threads_arr);
    free(work_arr);

    return total;
}
