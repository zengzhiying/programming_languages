#!/usr/bin/env python
# coding=utf-8
"""使用matplotlib绘制3D曲面图
"""
import sys

import numpy
from matplotlib import pyplot
from mpl_toolkits.mplot3d import Axes3D

reload(sys)
sys.setdefaultencoding('utf-8')

if __name__ == '__main__':
    fig = pyplot.figure()
    axes = Axes3D(fig)

    x = numpy.arange(-5, 2, 0.01)
    y = numpy.arange(-5, 2, 0.01)
    X, Y = numpy.meshgrid(x, y)

    Z = numpy.power(X, 4) + 3 * X**3 + 2 * X**2 + 2.5 * X + 6 + Y**4 + 1.1 * Y**2 + 2 * (X*Y)**2 + 0.8 * X * Y**2

    pyplot.xlabel('x.')
    pyplot.ylabel('y.')

    # azim为水平顺时针旋转度数, elev为倾斜角度
    axes.view_init(elev=0, azim=45)
    axes.plot_surface(X, Y, Z, rstride=1, cstride=1, cmap=pyplot.cm.coolwarm)
    axes.set_zlabel('z')
    pyplot.show()
