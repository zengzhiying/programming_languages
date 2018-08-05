#!/usr/bin/env python
# coding=utf-8
from cffi import FFI

ffibuilder = FFI()

ffibuilder.set_source("_compute1",
   r""" // passed to the real C compiler,
        // contains implementation of things declared in cdef()
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
    """,
    libraries=[])   # or a list of libraries to link with
    # (more arguments like setup.py's Extension class:
    # include_dirs=[..], extra_objects=[..], and so on)
    # 如果编译好其他的c/c++动态链接库, 比如librtmp.so, 则参数可以写成: libraries=['rtmp'], build之后即可正常调用.

ffibuilder.cdef("""
    // declarations that are shared between Python and C
    int size;
    int *weights;
    void compute(int *data, int *sum);
    void init(int *w, int l);
    void release();
""")

if __name__ == "__main__":
    ffibuilder.compile(verbose=True)