#include "stdio.h"
#include "rusty_primes.h"

int main() {
    int output_buffer[1000];
    int cnt;
    cnt = primes(20, output_buffer, 1000);
    printf("cnt until 20 is %d\n", cnt);
    for (int i = 0; i < cnt; i += 1) {
        printf("%d ", output_buffer[i]);
    }
    printf("\n");
    cnt = primes(2000, output_buffer, 1000);
    printf("cnt until 2000 is %d\n", cnt);
    cnt = primes(200000, output_buffer, 1000);
    printf("cnt until 200000 is %d\n", cnt);
}
