#!/usr/bin/python3
# coding=utf-8
"""python pyftpdlib模块使用, 快速搭建一个简单的ftp服务器
模块地址: https://pypi.org/project/pyftpdlib/
pip安装: pip3 install pyftpdlib
安装之后启动一个最简单的ftp服务: python3 -m pyftpdlib
此时根目录为当前目录(无论当前哪个目录都无法进入上级目录), 默认监听地址为: 0.0.0.0, 端口号: 2121
默认目录只读, 匿名用户登录即可
具体参数帮助执行: python3 -m pyftpdlib --help
-i 指定绑定的地址
-p 指定监听端口, 默认2121
-w 添加这个参数表示开启写权限
-d 指定根目录, 默认为当前目录
-u 指定用户名登录, 默认为匿名用户
-P 指定登录密码
完整的命令比如: python3 -m pyftpdlib -i 192.168.182.133 -p 2123 -w -d /home/ftp_test -u ftp1 -P 123456
另外也可以通过代码实现更多的功能, 简单示例代码如下
"""
from pyftpdlib.authorizers import DummyAuthorizer
from pyftpdlib.handlers import FTPHandler
from pyftpdlib.servers import FTPServer

authorizer = DummyAuthorizer()
# 设置用户和权限
# perm参数: e - 更改目录(但是一定不能进入设置目录的上层目录)
# l - 列出文件, r - 检索文件
# a - 将数据追加到现有文件, d - 删除文件或目录
# f - 重命名文件或目录, m - 创建目录
# w - 将文件存储到服务器, M - 更改文件模式和权限
# T - 更改文件修改时间  上面这些参数不加即不允许
authorizer.add_user("user", "12345", "/home/ftp_test", perm="elradfmwMT")
# 添加允许匿名并指定根目录
# authorizer.add_anonymous("/home/nobody")

handler = FTPHandler
handler.authorizer = authorizer

server = FTPServer(("0.0.0.0", 21), handler)
server.serve_forever()
