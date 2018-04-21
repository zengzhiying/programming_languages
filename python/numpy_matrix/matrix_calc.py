#!/usr/bin/env python
# coding=utf-8
import numpy as np
def matrix_test(x):
    a1 = np.array([
    [1, x],
    [2, 2.3]
])
    a2 = np.array([
    [x, 2],
    [1.2, 3]
])
    return np.dot(a1, a2)
a1 = np.array([[1, 0, 2], 
               [-1, 3, 1]])
a2 = np.array([
    [3, 1],
    [2, 1],
    [1, 0]
])
num = np.dot(a1, a2)
print num
print np.linalg.det(num)
print matrix_test(3.1)
print np.linalg.det(matrix_test(3.1))
print "--------------------------"
hx = np.array([
    [1, 2, 3, 4],
    [2, 2.3,3,5]
], dtype=float)
h1 = np.array([
    [1, 2.2,2.3,1.5],
    [2, 4, 6, 8],
    [1.3, 2.3, 3.2, 3.6],
    [7.2, 6.8, 8.9, 9.8]
], dtype=float)
hxx = np.array([
    [1, 3, 2,6],
    [2, 3, 7, 5]
], dtype=float)
cj = np.dot(hx, h1)
cjj = np.dot(cj, hxx.T)
print cjj
