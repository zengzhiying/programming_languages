#!/usr/bin/env python
# coding=utf-8
"""ffi调用带初始化过程的库.
无痛调用并获得原生性能.
"""
from _compute1 import ffi, lib

data1 = ffi.new("int[]", [1, 3, 8])
data2 = ffi.new("int[]", [3, 2, 7])
size = ffi.cast("int", 3)
weight = ffi.new("int[]", [0, 1, 0])
count = ffi.new("int *")
lib.init(weight, size)
# 重复初始化测试
# lib.init(weight, size)
lib.compute(data1, count)
print(count[0])
lib.compute(data2, count)
print(count[0])
lib.release()

weight = ffi.new("int[]", [1, 0, 1])
lib.init(weight, size)
lib.compute(data1, count)
print(count[0])
lib.compute(data2, count)
print(count[0])
lib.release()
# 重复释放测试
# lib.release()
