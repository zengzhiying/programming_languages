#!/usr/bin/python
# -*- coding:utf-8 -*-
# 计算函数在某一范围内的极值
import datetime
# 取最小值和最大值
min_x = 1
max_x = 35
min_y = 0.2501
max_y = 0.999
float_x = 1.0
float_y = 0.001
range_num1 = int((max_x-min_x)/float_x)
range_num2 = int((max_y-min_y)/float_y)
print range_num1,range_num2
# 求函数值的方法
def two_func(x, y):
    if(1.0055*x - 1 <= 0.000001):
        return 0
    else:
        return (((15 - 15*y)*x*0.0055*1.0055**x)/(1.0055**x - 1)) - (15-15*y)

def two_func1(x, y):
    return x**3 + y**3 - 3*x*y

#exit()
# 最值计算
def funcMax(min_x, max_x, min_y, max_y, float_x, float_y):
    Max = two_func(min_x, min_y)
    lx = min_x
    ly = min_y
    y = min_y
    while(min_x <= max_x):
        min_y = y
        while(min_y <= max_y):
            min_y = min_y + float_y
            tj = two_func(min_x, min_y)
            if(tj > Max):
                Max = tj
                lx = min_x
                ly = min_y
        min_x = min_x + float_x
    return [lx, ly, Max]
def funcMin(min_x, max_x, min_y, max_y, float_x, float_y):
    Min = two_func(min_x, min_y)
    lx = min_x
    ly = min_y
    y = min_y
    while(min_x <= max_x):
        min_y = y
        while(min_y <= max_y):
            min_y = min_y + float_y
            tj = two_func(min_x, min_y)
            if(tj < Min):
                Min = tj
                lx = min_x
                ly = min_y
            # 以下输出统计数据，可通过 >> result.out保存到文件
            #print min_x,min_y,tj
        min_x = min_x + float_x
    return [lx, ly, Min]
start = datetime.datetime.now()
result = funcMin(min_x, max_x, min_y, max_y, float_x, float_y)
end = datetime.datetime.now()
print "t: ",result[0], " x: ", result[1], " min: ", result[2]
print "计算时间：",(end-start).seconds, "s"
