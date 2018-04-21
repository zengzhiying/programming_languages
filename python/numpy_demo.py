#!/usr/bin/env python
# coding=utf-8
import numpy as np
# 注意numpy数组元素类型应该全部一致 否则numpy计算会出问题
heights = [1.73, 1.68, 1.78, 1.80, 1.83]
weights = [65.4, 63.2, 66.2, 68.3, 58.6]

np_heights = np.array(heights)
np_weights = np.array(weights)
print np_heights
print np_weights
bmi = np_weights/(np_heights ** 2)
print bmi
print np_heights + np_weights
print bmi > 21
print bmi[bmi > 21]
print type(np_heights)

np_2d = np.array([
        [1.72, 1.67, 2.32],
        [1.65, 2.12, 1.56]
    ])
print np_2d
print np_2d[0][1]
print np_2d[0:,1:3]

# 身高体重统计
np_city = np.array([
    [1.74, 65.3],
    [1.68, 55.6],
    [1.79, 60.3]
    ])
# 统计身高平均值
print np.mean(np_city[:,0])
# 身高中位数
print np.median(np_city[:,0])
# 求和
print np.sum(np_city[:,0])
# 样本标准差
print np.std(np_city[:,0])
# 生成符合正态分布的数据集 loc是概率平均值 scale是概率分布标准差(越大越矮胖,越小越瘦高) size指定数据集个数 默认是1个
hs = np.round(np.random.normal(loc=1.80, scale=0.20, size=100), 2)
#for h in hs:
#    print h
ws = np.round(np.random.normal(loc=60.0, scale=0.20, size=100), 2)
# 列连接成二维
hs_ws = np.column_stack((hs, ws))
print hs_ws
print np.mean(hs_ws[:,0])

