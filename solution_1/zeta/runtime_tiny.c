// Ultra-minimal: println_i64 directly, no _start wrapper.
// main() is the ELF entry point (set with -e main).
// inline the write and exit directly in main.
void println_i64(long long value);
void __attribute__((force_align_arg_pointer)) _start(void) {
    extern long long main(void);
    // inline: call main, then exit_group
    long long result;
    asm volatile("call *%1" : "=a"(result) : "r"(main) : "rdi");
    // exit_group
    asm volatile("syscall" : : "a"(231), "D"((int)result) : "rcx", "r11");
    __builtin_unreachable();
}
