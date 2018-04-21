#!/usr/bin/python
# -*- coding:utf-8 -*-
from fabric.api import local, lcd, cd, run, env
#执行本地操作
def lsfab():
    # 进入目录
    with lcd('~/app'):
        # 本地执行命令
        local('ls')

env.hosts=['root@bigdata2:22',]
env.password = 'bigdata.123'

def remote_ls():
    print "remote ls shell"
    with cd('~'):
        run('ls -l')
