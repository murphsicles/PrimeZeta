// PrimeZeta competition runner - vfork-optimized for max passes/sec
// Replaces the bash while-true loop with a C vfork+exec harness.
// Compile: gcc -O2 -s -o runner runner.c
#define _GNU_SOURCE
#include <unistd.h>
#include <sys/wait.h>

int main(void) {
    char *const argv[] = { "./prime", NULL };
    char *const envp[] = { NULL };  // empty environ for max speed
    for (;;) {
        pid_t pid = vfork();
        if (pid == 0) {
            execve("./prime", argv, envp);
            _exit(1);
        }
        // parent blocks until child exits
        waitpid(pid, NULL, 0);
    }
}
