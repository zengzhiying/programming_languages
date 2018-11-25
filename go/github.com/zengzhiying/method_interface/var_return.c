#include <stdio.h>
#include <stdlib.h>

int main(int argc, char const *argv[])
{
    int *get_var();
    int *get_malloc_var();
    int *p = get_var();
    *p = 30;
    printf("%d\n", *p);
    int *p1 = get_malloc_var();
    printf("%d\n", *p1);
    free(p1);
    return 0;
}

int *get_var()
{
    int i = 20;
    return &i;
}

int *get_malloc_var()
{
    int *i = (int *)malloc(sizeof(int));
    *i = 30;
    return i;
}
