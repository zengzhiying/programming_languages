#include <stdio.h>
#include <math.h>
#include <malloc.h>

int size;
int *weights = NULL;

void init(int *w, int l);
void compute(int *data, int *sum);
void release();

void compute(int *data, int *sum)
{
    int i, r = 0;
    for(i = 0; i < size; i++)
        r += weights[i] * data[i];
    *sum = r;
}

void init(int *w, int l)
{
    int i;
    size = l;
    if(weights != NULL)
        release();
    weights = (int *)malloc(size * sizeof(int));
    for(i = 0; i < size; i++)
        weights[i] = w[i];
    printf("inited.\n");
}

void release()
{
    if(weights != NULL) {
        free(weights);
        weights = NULL;
    }
    printf("released.\n");
}
