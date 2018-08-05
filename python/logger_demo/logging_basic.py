#!/usr/bin/env python
# -*- coding:utf-8 -*-
"""logging日志模块基本的配置, 只用于测试, 不建议用于生产环境
生产环境请使用标准的配置方式参考: logger_demo.py
"""
import logging
from logging.handlers import RotatingFileHandler
# 配置
logging.basicConfig(level=logging.DEBUG,
                    format='%(asctime)s %(filename)s[line:%(lineno)d] %(levelname)s %(message)s',
                    datefmt='%Y-%m-%d %H:%M:%S',
                    filename='debug.log',
                    filemode='w')
# 使用debug打印的是等级高的信息，信息严格按照上面配置的打印，info只是打印普通日志信息，一般情况下就是使用debug和info打印
logging.debug("debug info")
logging.info("info info")
logging.warning("warning info")

# 日志级别大小关系为：CRITICAL > ERROR > WARNING > INFO > DEBUG > NOTSET 
# 默认日志级别为WARNING这样的话小于默认级别的打印信息都不显示，大于或等于默认级别的输出才会打印或写日志
# 所以可以通过basicConfig中配置默认日志级别，然后灵活使用logging中打印不同日志类型的方法来实现不同类型日志的打印，日志的开启与关闭等

# 除了上面配置方式 还常用到下面的配置方式 (配置日志滚动数量和单个文件的大小)
log_handler = RotatingFileHandler('/var/log/log_demo.log', maxBytes=64*1024*1024, backupCount=7)
log_formatter = logging.Formatter('%(asctime)s [%(levelname)s] %(message)s')
log_handler.setFormatter(log_formatter)
logger = logging.getLogger('')
logger.setLevel(logging.INFO)
logger.addHandler(log_handler)
logging.info("logging configure done.")
