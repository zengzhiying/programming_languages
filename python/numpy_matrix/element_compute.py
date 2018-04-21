# coding=utf-8
import math
import numpy as np
class ElementCompute(object):
    
    # 获得乘积公因子元素
    def get_factor1(self, r, x):
        return 1.0/(2.0*(r ** 4/4.0 + x) ** 0.5)
    # 获得求和公因子元素1 正数
    def get_fector2(self, r, x):
        return r**2/2.0 + (r**4/4.0 + x) ** 0.5

    # 获得求和公因子元素2 负数
    def get_fector3(self, r, x):
        return -1 * (r**2/2.0) + (r**4/4.0 + x) ** 0.5

    '''获得下标为1系列的元素'''
    # 获得A1元素
    def get_A1(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = (math.cos(fector2 ** 0.5/2.0))/fector2
        f2 = (math.cosh(fector3 ** 0.5/2.0))/fector3
        return fector1*(f1 + f2)

    # 获得B1元素
    def get_B1(self, r, x):
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.sin(fector2 ** 0.5/2.0)/(fector2 ** 1.5)
        f2 = math.sinh(fector3 ** 0.5/2.0)/(fector3 ** 1.5)
        return self.get_factor1(r, x)*(f1 + f2)

    # 获得C1元素
    def get_C1(self, r, x):
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.cos(fector2 ** 0.5/2.0)
        f2 = math.cosh(fector3 ** 0.5/2.0)
        return self.get_factor1(r, x)*(f1 - f2)

    # 获得D1元素
    def get_D1(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.sin(fector2 ** 0.5/2.0)/(fector2 ** 0.5)
        f2 = math.sinh(fector3 ** 0.5/2.0)/(fector3 ** 0.5)
        return fector1*(f1 - f2)

    # 获得m1元素
    def get_m1(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = (fector2 ** 0.5)*math.sin(fector2 ** 0.5/2.0)
        f2 = (fector3 ** 0.5)*math.sinh(fector3 ** 0.5/2.0)
        return fector1*(f1 + f2)

    # 获得b1元素
    def get_b1(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = fector2*math.cos(fector2 ** 0.5/2.0)
        f2 = fector3*math.cosh(fector3 ** 0.5/2.0)
        return fector1*(f1 + f2)

    # 获得d1元素
    def get_d1(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = (fector2**1.5)*math.sin(fector2 ** 0.5/2.0)
        f2 = (fector3 ** 1.5)*math.sinh(fector3**0.5/2.0)
        return fector1*(f1 - f2)

    '''获得下标为2系列的元素'''
    # 获得A2元素
    def get_A2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.cos(fector3**0.5)/fector3
        f2 = math.cosh(fector2**0.5)/fector2
        return fector1*(f1 + f2)

    # 获得B2元素
    def get_B2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.sin(fector3**0.5)/fector3**1.5
        f2 = math.sinh(fector2**0.5)/fector2**1.5
        return fector1*(f1 + f2)

    # 获得C2元素
    def get_C2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.cos(fector3**0.5)
        f2 = math.cosh(fector2**0.5)
        return fector1*(f1 - f2)

    # 获得D2元素
    def get_D2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = math.sin(fector3**0.5)/fector3**0.5
        f2 = math.sinh(fector2**0.5)/fector2**0.5
        return fector1*(f1 - f2)

    # 获得m2元素
    def get_m2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = (fector3**0.5)*math.sin(fector3**0.5)
        f2 = (fector2**0.5)*math.sinh(fector2**0.5)
        return fector1*(f1 + f2)

    # 获得b2元素
    def get_b2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = fector3*math.cos(fector3**0.5)
        f2 = fector2*math.cosh(fector2**0.5)
        return fector1*(f1 + f2)

    # 获得d2元素
    def get_d2(self, r, x):
        fector1 = self.get_factor1(r, x)
        fector2 = self.get_fector2(r, x)
        fector3 = self.get_fector3(r, x)
        f1 = (fector3**1.5)*math.sin(fector3**0.5)
        f2 = (fector2**1.5)*math.sinh(fector2**0.5)
        return fector1*(f1 - f2)



    # 求矩阵乘积行列式的值
    def get_matrix_value(self, r, x, ei):
        hx = np.array([
            [self.get_A2(r, x)*x, self.get_B2(r, x)*x, self.get_C2(r, x)/ei, self.get_D2(r, x)/ei],
            [self.get_C2(r, x)*x*ei, self.get_D2(r, x)*x*ei, self.get_b2(r, x), self.get_m2(r, x)]
            ])
        h1 = np.array([
            [self.get_A1(r, x)*x, self.get_B1(r, x)*x, self.get_C1(r, x)/ei, self.get_D1(r, x)/ei],
            [(-1)*x*self.get_D1(r, x), self.get_A1(r, x)*x , (-1)*self.get_m1(r, x)/ei, self.get_C1(r, x)/ei],
            [ei*x*self.get_C1(r, x), ei*x*self.get_D1(r, x), self.get_b1(r, x), self.get_m1(r, x)],
            [(-1)*ei*x*self.get_m1(r, x), ei*x*self.get_D1(r, x), (-1)*self.get_d1(r, x), self.get_b1(r, x)]
            ])
        hxx = np.array([
            [x*self.get_B1(r, x), x*self.get_A1(r, x), ei*x*self.get_D1(r, x), ei*x*self.get_C1(r, x)],
            [self.get_D1(r, x)/ei, self.get_C1(r, x)/ei, self.get_m1(r, x), self.get_b1(r, x)]
            ])
        # 矩阵相乘
        return np.linalg.det(np.dot(np.dot(hx, h1), hxx.T))

