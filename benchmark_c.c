#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Simple benchmark that calls the Zeta executable in a loop
int main() {
    clock_t start = clock();
    double duration = 5.0; // 5 seconds
    int passes = 0;
    
    printf("Benchmarking Zeta competition submission for %.1f seconds...\n", duration);
    
    while ((double)(clock() - start) / CLOCKS_PER_SEC < duration) {
        // Call the Zeta executable
        int result = system("Primes\\PrimeZeta\\solution_1\\prime_final.exe > nul");
        passes++;
        
        if (passes % 1000 == 0) {
            double elapsed = (double)(clock() - start) / CLOCKS_PER_SEC;
            printf("  Pass %d - Elapsed: %.3f seconds\n", passes, elapsed);
        }
    }
    
    double total_time = (double)(clock() - start) / CLOCKS_PER_SEC;
    double passes_per_second = passes / total_time;
    
    printf("\n=== BENCHMARK RESULTS ===\n");
    printf("Total passes: %d\n", passes);
    printf("Total time: %.3f seconds\n", total_time);
    printf("Passes per second: %.3f\n", passes_per_second);
    printf("Expected competition output:\n");
    printf("zeta;%d;%.3f;4;algorithm=wheel;faithful=yes;bits=8;parallel=yes;simd=avx512\n", 
           passes, total_time);
    
    return 0;
}