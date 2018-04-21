#!/usr/bin/env python
# coding=utf-8
"""python设置当前进程名称
比如当前进程默认情况下进程名和启动命令一致, 为: python set_process_name.py
这里手动设置为: process_name_demo
设置后通过: ps -ef或ps aux就可以看到指定的进程名而不再是默认进程名
setproctitle模块: https://pypi.python.org/pypi/setproctitle
"""
import time

from setproctitle import setproctitle
from setproctitle import getproctitle

if __name__ == '__main__':
    print("当前进程名称: {}".format(getproctitle()))
    # 设置进程名
    proc_name = "process_name_demo"
    setproctitle(proc_name)
    print("设置后的进程名称: {}".format(getproctitle()))
    time.sleep(30)
    print("bye.")