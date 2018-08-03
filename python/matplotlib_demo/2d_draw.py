#!/usr/bin/env python
# coding=utf-8
"""使用matplotlib模块绘制基本的平面图表
"""
import matplotlib.pyplot as plt

year = [1991, 1992, 1993, 1994, 1995]
number = [35, 46, 57, 68, 76]
# 绘制线形图
# plt.plot(year, number)
plt.fill_between(year, number, 0, color='green')
# 给坐标轴加标签 只能是英文
plt.xlabel('year')
plt.ylabel('number')
# 添加标题
plt.title("this is title")
# 添加刻度
plt.yticks([0, 20, 40, 60, 80, 100], ['0', '20', '40', '60', '80', '100'])
plt.show()

# 绘制散点图
plt.scatter(year, number)
plt.show()

values = [0, 0.6, 1.4, 1.6, 2.2, 2.5, 2.6, 3.2, 3.5, 3.9, 4.2, 6, 6.36]

# 绘制柱状图 bins是区间范围 也就是柱宽
plt.hist(values, bins=3)
plt.show()

