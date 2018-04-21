#!/usr/bin/env python
# coding=utf-8
from apscheduler.schedulers.blocking import BlockingScheduler
#from apscheduler.scheduler import Scheduler
from datetime import datetime

def my_job1():
    print "runing job1!"
def my_job2():
    print "runing job2!"
def my_job3():
    print "runing job3!"

sched = BlockingScheduler()
#sched = Scheduler(daemonic = False)
# 每隔5s执行my_job1
# sched.add_job(my_job1, 'interval', seconds=5)

# 每整10分钟执行一次 比如:10:10,10:20这样
sched.add_job(my_job2, 'cron', hour="0-23", minute='*/10')
# 固定时间执行一次作业
sched.add_job(my_job3, 'date', run_date=datetime(2017, 01, 17, 10, 12, 20))
# 装饰器写法 每30s一次
@sched.scheduled_job("cron", second="*/30")
def my_job4():
    print "runing job4!"
try:
    sched.start()
except (KeyboardInterrupt, SystemExit):
    print "shutdown!"
    sched.shutdown()

# 更多参数参考:https://my.oschina.net/u/2306127/blog/662267
