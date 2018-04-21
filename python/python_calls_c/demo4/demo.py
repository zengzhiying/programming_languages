#!/usr/bin/env python
# coding=utf-8
import ctypes

print('载入库.')
# 此处相当于加载extern "C"代码块中所有的内容, 执行部分直接执行, 函数部分放入堆栈
# 可以很方便的做初始化操作
load_lib = ctypes.cdll.LoadLibrary('./libdemo.so')
print("载入完毕.")
# 其他必要初始化操作
load_lib.inits()

load_lib.test1()
r = load_lib.test(6)
print("r: %d" % r)

load_lib.test2()

print('销毁资源.')
load_lib.close()
