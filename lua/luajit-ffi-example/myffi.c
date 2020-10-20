#include <stdio.h>
#include <stdlib.h>

int *m = NULL;
int idx = 0;
size_t MAX_SIZE = 0;

// gcc -g -o libmyffi.so -fpic -shared myffi.c -std=c99

void init_memory(size_t size) {
    if(m == NULL) {
        m = (int *)malloc(size * sizeof(int));
        MAX_SIZE = size;
    } else
        printf("Initialized\n");
}

void set_value(int v) {
    if(idx >= MAX_SIZE) {
        printf("index overflow!\n");
        return;
    }
    m[idx] = v;
    idx++;
}


void print_values() {
    if(m == NULL)
        return;
    for (int i = 0; i < MAX_SIZE; ++i)
    {
        printf("%d -> %d\n", i, m[i]);
    }
}

void free_memory() {
    if(m == NULL) {
        printf("Already released\n");
        return;
    }
    free(m);
    m = NULL;
    idx = 0;
    MAX_SIZE = 0;
    printf("released!\n");
}

int *malloc_convert(void *p) {
    return (int *) p;
}
