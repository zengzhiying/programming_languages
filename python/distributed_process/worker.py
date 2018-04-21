#!/usr/bin/python
# coding=utf-8
"""分布式进程worker
一般运行多个并行处理manager队列
"""
import logging
import os
import sys
import time
import multiprocessing
import threading
import Queue
from multiprocessing.managers import BaseManager
reload(sys)
sys.setdefaultencoding('utf-8')

class QueueManager(BaseManager):
    pass

# 注册manager队列
QueueManager.register('get_task_queue')
QueueManager.register('get_result_queue')
# 连接manager
manager = QueueManager(address=('192.168.0.136', 7001), authkey='123456')
manager.connect()
# 获取队列
task = manager.get_task_queue()
result = manager.get_result_queue()

while True:
    try:
        if task.empty():
            print("task queue 为空.")
            time.sleep(0.1)
            continue
        message = task.get()
        print("received message: {}".format(message))
        r = 3 * message + 6
        result.put(r)
        print('result message: {}'.format(r))
    except Exception, e:
        print("读取队列异常！ {}".format(e))
        time.sleep(3)
        print("重新连接manager.")
        try:
            manager.connect()
            task = manager.get_task_queue()
            result = manager.get_result_queue()
        except Exception, e:
            print("重新连接失败！ {}".format(e))

