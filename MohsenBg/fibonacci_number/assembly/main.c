#include <stdio.h>

// external fibonacci function from assembly
extern int fibonacci(int n);

int main() {
    int n = 20;
    int result = fibonacci(n);
    printf("fibonacci(%d) = %d\n", n, result);
    return 0;
}
 