// C wrapper for Zeta competition entry
// Calls Zeta function in infinite loop, prints result

#include <stdio.h>

// External Zeta function
extern long long zeta_main(void);

int main() {
    // Call once to compute prime count
    long long result = zeta_main();
    
    // Infinite loop printing result
    while (1) {
        printf("%lld\n", result);
    }
    
    return 0;
}