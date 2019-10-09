#!/usr/bin/env python
# coding=utf-8
"""python进程池测试案例
进程间通信建议使用进程间队列, 但是注意进程间队列的性能比较低下, 不能用在要求高的场景
性能最佳的方式是所有进行独立并发执行, 没有通信
"""

import time
import random
import math
import multiprocessing
from multiprocessing import Pool
from multiprocessing import Queue

def test(s):
    print "hello %s" % s
    iii = random.randint(0,5)
    print iii
    time.sleep(iii)
    return ""

def test1(msg):
    print "msg: %s" % msg
    time.sleep(3)

def test2(msg):
    time.sleep(random.randint(1, 3))
    print msg
    return msg

def test3(datas, n):
    print "group number: %d size: %d" % (n, len(datas))
    data_sum = 0
    for data in datas:
        # d = random.randint(1, 256)
        data_sum += ((data**0.1)*abs(math.sin(data)) + math.cos(data**0.5)**2)
    return data_sum

if __name__ == '__main__':
    # 进程池并行处理
    s = 'kk'
    pool = multiprocessing.Pool(processes=10)
    for i in range(100):
       msg = str(i)
       pool.apply_async(test, (msg,))
    print 'close'
    pool.close()
    print 'join'
    pool.join()
    print "done"

    # 初始化进程池 初始化之后进程数量就确定了
    # 如果不指定进程池大小 默认根据计算机CPU核数自动分配
    print "初始化进程池."
    process_pool = Pool()
    # time.sleep(3)
    for i in range(4):
        msg = 'number -> %d' % i
        process_pool.apply_async(test1, args=(msg, ))
    print "主进程加入完毕."
    process_pool.close()    # 关闭进程池 不再接受新的进程加入
    process_pool.join()    # 阻塞当前主进程 等待多个子进程执行完毕
    print "test done."    # 此时多个进程结束后 只存在一个主进程
    time.sleep(3)
    print "test2 start."
    process_pool = Pool(8)
    process_result = []
    for i in range(36):
        msg = 'n: %d' % i
        rs = process_pool.apply_async(test2, args=(msg, ))
        process_result.append(rs)
    process_pool.close()
    process_pool.join()
    print "多进程返回结果如下:"
    for rs in process_result:
        print rs.get()

    print "test3 start."
    data_set = [x for x in range(2, 60000001, 2)]
    start_time = time.time()
    print test3(data_set, 1)
    end_time = time.time()
    print "计算耗时: %f" % (end_time - start_time)
    time.sleep(6)
    print "test3多进程计算."
    compute_result = []
    process_pool = Pool(processes=8)
    start_time = time.time()
    group_number = len(data_set)/16
    for i in range(16):
        rs = process_pool.apply_async(test3, args=(data_set[i*group_number:(i + 1)*group_number], i, ))
        compute_result.append(rs)
    process_pool.close()
    process_pool.join()
    # 合并计算
    print sum(map(lambda r:r.get(), compute_result))
    end_time = time.time()
    print "计算耗时: %f" % (end_time - start_time)


