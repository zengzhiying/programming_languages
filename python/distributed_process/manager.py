#!/usr/bin/python
# coding=utf-8
"""分布式进程manager
用于向任务队列分发任务或消息
"""
import sys
import time
import Queue
from multiprocessing.managers import BaseManager
reload(sys)
sys.setdefaultencoding('utf-8')

class QueueManager(BaseManager):
    pass

# 队列初始化 根据实际需要
# 任务队列 manager -> worker
task_queue = Queue.Queue()
# 结果队列 worker -> manager 如不需要结果队列可以去掉
result_queue = Queue.Queue()

# 注册队列
QueueManager.register('get_task_queue',callable=lambda: task_queue)
QueueManager.register('get_result_queue', callable=lambda: result_queue)

# 实例化manager并启动 将队列绑定值网络端口
manager = QueueManager(address=('192.168.0.136', 7001), authkey='123456')
manager.start()

# 从manager获取队列
task = manager.get_task_queue()
result = manager.get_result_queue()

# 示例发送数据
for i in range(1000):
    task.put(i)
    print('send message: %d' % i)
    if not result.empty():
        r = result.get()
        print("result message: {}".format(r))
    time.sleep(0.1)
