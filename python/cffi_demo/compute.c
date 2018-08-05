#include <stdio.h>
#include <math.h>

void compute(int *a, int *b, int size, int *r)
{
    int i, sum = 0;
    for(i = 0; i < size; i++) {
        sum += pow(a[i] - b[i], 2);
    }
    *r = sum;
}
