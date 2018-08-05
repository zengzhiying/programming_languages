#!/usr/bin/env python
# coding=utf-8
"""python拦截kill信号的代码, 最常用的信号有下面两个:
SIGTERM - 15, kill命令默认信号
SIGINT - 2, 可以拦截Ctrl-C终止命令

SIGKILL - 9, 不可被捕获和阻塞处理.
"""
import sys
import time
import signal

def terminate_handler(signum, frame):
    current_time = time.time()
    print(current_time)
    time.sleep(1)
    print("signum: {}".format(signum))
    sys.exit(0)

print("start.")
signal.signal(signal.SIGTERM, terminate_handler)
signal.signal(signal.SIGINT, terminate_handler)
print("signal.")
for i in range(100):
    time.sleep(1)
