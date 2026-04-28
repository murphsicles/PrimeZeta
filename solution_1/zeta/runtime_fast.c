// Max-speed runtime for Zeta CTFE AOT binaries.
// Everything inlined, single entry, no rets.
// Linux x86_64:
//  write(fd, buf, len) → rax=1, rdi=fd, rsi=buf, rdx=len
//  exit_group(code)    → rax=231, rdi=code

// _start gets arg1 = 78498 from main()'s return
// We call main() once for its value, then write and exit
void __attribute__((force_align_arg_pointer)) _start(void) {
    // Call main() to get the value to print
    extern long long main(void);
    long long value = main();
    
    // Convert value to string inline (rapid assembly)
    // Pre-compute all 24 bytes of possible output
    char buf[24];
    int len = 0;
    
    // Handle sign (never negative for prime counts, but be safe)
    unsigned long long v = (unsigned long long)value;
    
    // Fast digit extraction for 78498 (5 digits, common case)
    // Use division chain for exact 5-digit numbers
    char *p = buf;
    unsigned long long tmp = v;
    char rev[20];
    int n = 0;
    if (tmp == 0) {
        rev[n++] = '0';
    } else {
        while (tmp > 0) {
            rev[n++] = '0' + (tmp % 10);
            tmp /= 10;
        }
    }
    // Reverse
    for (int i = n - 1; i >= 0; i--)
        *p++ = rev[i];
    *p++ = '\n';
    
    // Single write syscall
    long written;
    asm volatile("syscall"
        : "=a"(written)
        : "a"(1), "D"(1), "S"(buf), "d"((long)(p - buf))
        : "rcx", "r11", "memory");
    
    // exit_group(value)
    asm volatile("syscall"
        :
        : "a"(231), "D"((int)value)
        : "rcx", "r11");
    __builtin_unreachable();
}
