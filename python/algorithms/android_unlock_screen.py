# 计算安卓手机锁屏中所有的解锁方案一共有几个
from itertools import *
impossible = {'19': '5',
              '13': '2',
              '91': '5',
              '97': '8',
              '73': '5',
              '71': '4',
              '93': '6',
              '17': '4',
              '46': '5',
              '28': '5',
              '31': '2',
              '64': '5',
              '37': '5',
              '79': '8',
              '39': '6',
              '82': '5'}
def counts():
    iterlst = chain(*(permutations('123456789', i) for i in range(4, 10)))
    count = 0
    for i in iterlst:
        stri = ''.join(i)
        for k, v in impossible.items():
            if k in stri and v not in stri[:stri.find(k)]:
                break
        else:
            count += 1
    return count
print(counts())