#!/usr/bin/env python
# coding=utf-8
import ctypes
lib_dll = ctypes.CDLL("./libdemo.so")
print lib_dll.test(5)
