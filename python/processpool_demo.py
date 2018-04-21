#!/usr/bin/python
# coding=utf-8
import multiprocessing
import time
import random
""" 进程池demo 进程池使用时一定要是独立的方法和外部没有耦合 不能有全局共享变量也不能是类方法否则都将报错
    进程间通信推荐使用队列 """
def test(s):
    print "hello %s" % s
    iii = random.randint(0,5)
    print iii
    time.sleep(iii)
    return ""

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
