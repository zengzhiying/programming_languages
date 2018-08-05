#!/usr/bin/env python3
# coding=utf-8
"""python3 日志模块 logging使用demo
测试同样适用于python 2.7版本
任何可用于生产的项目或应用系统都应该合理添加日志功能
软件系统必备:
    文档:
        需求文档
        开发文档
        架构设计文档
        良好的对外接口设计和接口文档
        安装和使用文档
    主要源代码
    辅助脚本 (安装脚本, 启动脚本, 维护脚本等.)
    配置文件
    日志系统
    单元测试和用例
    README.md
    等...
"""
import time
import logging
import logging.config
from logging.handlers import RotatingFileHandler
from logging.handlers import TimedRotatingFileHandler

import yaml

if __name__ == '__main__':
    # 日志配置方式1
    # logger_example 为日志模块名称, 默认情况下会创建;
    # 指定模块的输出只会输出该模块的日志, 并且子模块会继承配置(永远不需要跨模块传递), 
    # 即假设名称为'a', 则'a.b', 'a.b.c'都会继承配置, 直接get然后使用即可.
    # 对于需要直接执行的主脚本, 一般写成: logging.getLogger()或者logging.getLogger(''), 
    # 这样会返回层次结构根记录器, 即所有调用的第三方模块和库日志都可以正常按照根记录器的配置输出.
    logger = logging.getLogger('logger_example')
    logger.setLevel(logging.DEBUG)

    handler = RotatingFileHandler('/var/log/logger_example.log',
                                  mode='w',
                                  maxBytes=128*1024*1024,
                                  backupCount=3,
                                  encoding='utf-8')
    handler.setLevel(logging.DEBUG)

    # datefmt参数默认为带毫秒的格式, 一般情况下建议默认即可
    formatter = logging.Formatter(fmt='%(asctime)s - %(name)s [%(levelname)s] %(message)s',
                                  datefmt='%Y-%m-%d %H:%M:%S')

    handler.setFormatter(formatter)

    logger.addHandler(handler)

    # test
    logger.info("test")
    logger.debug('test')
    logger.warning('test')
    # logging 的函数只打印root log也就是根日志, 一般不使用
    # logging.info("test.")

    # 按照时间回滚日志
    logger1 = logging.getLogger('logger_example_time')
    logger1.setLevel(logging.DEBUG)
    handler1 = TimedRotatingFileHandler('/var/log/logger_example_time.log',
                                        # 日志单位: 默认H:小时, S:秒, M:分钟, D:天, 另外还有星期等参考文档.
                                        when='s',  # 这里为了演示改为秒, 参数不分大小写.
                                        # 间隔, 默认: 1
                                        interval=1,
                                        backupCount=0,
                                        encoding='utf-8')
    formatter1 = logging.Formatter(fmt='%(asctime)s - %(name)s [%(levelname)s] %(message)s')
    handler1.setFormatter(formatter1)
    logger1.addHandler(handler1)

    # test logger1
    logger1.info("test1")
    time.sleep(2)
    logger1.debug('test1')
    time.sleep(2)
    logger1.warning('test1')
    time.sleep(2)
    logger1.error('test1')

    # 日志配置方式2
    with open('logger.yaml', 'r') as f:
        logger_conf = yaml.load(f)

    logging.config.dictConfig(logger_conf)

    logger = logging.getLogger('loggerExample')

    logger.debug('test2')
    logger.info('test2')
    logger.error('test2')
