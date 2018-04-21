#!/usr/bin/python
# -*- coding:utf-8 -*-
from fabric.api import *

# 服务器登录参数
env.roledefs = {
    # 操作一致的放一组，一组执行同一个操作
    'servers1':['root@bigdata2:22',],
    # 第二组
    'servers2':['root@bigdata3:22',]
}

# 集群间设置ssh免密登录，这样就不用设置密码参数了

# 本机操作
def localtask():
    local('/bigdata/zookeeper/zookeeper-3.4.6/bin/zkServer.sh start')
# servers1服务器组操作
@roles('servers1')
def task1():
    run('/bigdata/zookeeper/zookeeper-3.4.6/bin/zkServer.sh start')

# servers服务器组操作
@roles('servers2')
def task2():
    run('/bigdata/zookeeper/zookeeper-3.4.6/bin/zkServer.sh start')

# 执行任务
def doworks():
    execute(localtask)
    execute(task1)
    execute(task2)
