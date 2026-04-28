// No-libc syscall minimal runtime for Zeta CTFE AOT binaries.
// Provides only: _start (ELF entry), println_i64 (write syscall).
// Link with: gcc -static -nostartfiles -nodefaultlibs -o prime prime.o runtime_syscall.o
//
// Linux x86_64 syscalls:
//   write(fd, buf, count) → rax=1, rdi=fd, rsi=buf, rdx=count
//   exit_group(status)     → rax=231, rdi=status

static void write_stdout(const char* buf, unsigned long len) {
    long ret;
    asm volatile ("syscall"
        : "=a"(ret)
        : "a"(1), "D"(1), "S"(buf), "d"(len)
        : "rcx", "r11", "memory");
}

void println_i64(long long value) {
    char buf[24];
    int len = 0;
    unsigned long long v;
    
    if (value < 0) {
        buf[len++] = '-';
        v = -(unsigned long long)value;
    } else {
        v = (unsigned long long)value;
    }
    
    char digits[20];
    int dlen = 0;
    if (v == 0) {
        digits[dlen++] = '0';
    } else {
        while (v > 0) {
            digits[dlen++] = '0' + (v % 10);
            v /= 10;
        }
    }
    for (int i = dlen - 1; i >= 0; i--)
        buf[len++] = digits[i];
    buf[len++] = '\n';
    
    write_stdout(buf, len);
}

void _start(void) {
    extern long long main(void);
    long long result = main();
    
    // exit_group(rax=231, rdi=result)
    asm volatile ("syscall"
        :
        : "a"(231), "D"((long long)((int)result))
        : "rcx", "r11");
    __builtin_unreachable();
}
