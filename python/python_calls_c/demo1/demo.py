#!/usr/bin/env python
# coding=utf-8
import ctypes
c_lib = ctypes.cdll.LoadLibrary("./libdemo.so")
print c_lib.add(3, 5)
print "end."
