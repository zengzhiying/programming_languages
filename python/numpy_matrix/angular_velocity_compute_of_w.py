#!/usr/bin/env python
# coding=utf-8
from element_compute import ElementCompute
import time

ec = ElementCompute()
r = 0.10798147989354472
ei = 8.575e+6

# 精度
precise = 0.0001
# 以w为基准增加
w = 0.0001
num = 0
while True:
    x = (6.2e-7)*w**2
    s = ec.get_matrix_value(r, x, ei)
    num += 1
    # w = (x/(6.2e-7))**0.5
    if s < 0.0001 and s > -0.0001:
        print "发现角速度值为: %f时, 结果精度较高,结果为:%f" % (w, s)
        time.sleep(1)
    else:
        if num % 10000 == 0:
            print "偏差较大-----x值:%f ,角速度:%f ,行列式结果:%s" % (x, w, s)
    # num += 1
    # if num % 10000 == 0:
    #     # 每1000条打印一次记录
    #     print "此时角速度为:%f, x值:%f, 行列式结果:%f" % (w, x, s)
    w+=0.0001

# print ec.get_matrix_value(r, x, ei)

