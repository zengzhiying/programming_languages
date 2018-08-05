#!/usr/bin/env python
# coding=utf-8
from cffi import FFI

ffibuilder = FFI()

ffibuilder.set_source("_compute",
   r""" // passed to the real C compiler,
        // contains implementation of things declared in cdef()
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
    """,
    libraries=[])   # or a list of libraries to link with
    # (more arguments like setup.py's Extension class:
    # include_dirs=[..], extra_objects=[..], and so on)

ffibuilder.cdef("""
    // declarations that are shared between Python and C
    void compute(int *a, int *b, int size, int *r);
""")

if __name__ == "__main__":
    ffibuilder.compile(verbose=True)