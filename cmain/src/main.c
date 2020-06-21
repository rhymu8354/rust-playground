#include <inttypes.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

extern int32_t fib(int32_t n);

int main(int argc, char* argv[]) {
    static const int32_t n = 7;
    printf("F(%" PRId32 ") = %" PRId32 "\n", n, fib(n));
    return EXIT_SUCCESS;
}
