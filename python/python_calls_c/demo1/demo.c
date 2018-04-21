#include<stdio.h>

/**
 * gcc -o libdemo.so -shared -fPIC demo.c
 */

int add(int a, int b) {
    printf("a: %d   b: %d\n", a, b);
    int sum = a + b;
    return sum;
}
