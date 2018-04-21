#!/usr/bin/env python
# -*- coding:utf-8 -*-
# 汉诺塔问题 python递归模型
'''
来源：大梵天创造世界的时候做了三根金刚石柱子，在一根柱子上从下往上按照大小顺序摞着64片黄金圆盘。
大梵天命令婆罗门把圆盘从下面开始按大小顺序重新摆放在另一根柱子上。
并且规定，在小圆盘上不能放大圆盘，在三根柱子之间一次只能移动一个圆盘。
'''
# n为起始盘子个数，a,b,c为三个柱子的名称
def move_hanoi(n, a, b, c, num = 0):
    if n == 1:
        print a, '-->', c
        num = num + 1
        return num
    else:
        num = move_hanoi(n - 1, a, c, b, num)
        num = move_hanoi(1, a, b, c, num)
        num = move_hanoi(n - 1,b, a, c, num)
        return num

# 3层汉诺塔
num = move_hanoi(3, '柱子A', '柱子B', '柱子C')
print "移动次数：", num

'''
总的移动次数为：K(n) = 2^(n+2)-5
计算结果（移动圆片的次数）大约是1.84467440*10^19。
'''