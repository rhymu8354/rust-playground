#include <stdint.h>

extern int32_t fib(int32_t n) {
    switch (n) {
        case 0 : return 0;
        case 1 : return 1;
        default : return fib(n - 1) + fib(n - 2);
    }
}