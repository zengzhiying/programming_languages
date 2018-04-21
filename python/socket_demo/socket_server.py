#!C:\Python27\python.exe
# -*- coding:utf-8 -*-
# 单线程 即一次只能处理一个请求 多余请求会被阻塞
import socket
import sys
import time
host = '127.0.0.1'
port = 10188
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# 绑定端口
try:
    s.bind((host, port))
except socket.error, msg:
    print msg

# 设置连接数 同时处理的连接个数为10
s.listen(10)

while True:
    # 监听客户端请求 并输出连接状态
    conn, addr = s.accept()
    print "Connected with: %s : %s" % (addr[0], str(addr[1]))

    # 获取客户端数据
    data = conn.recv(1024)
    reply = "ok...%s" % data
    if not data:
        break
    print reply
    # 将获取到客户端的数据推到客户端
    time.sleep(6)
    conn.sendall(reply)

conn.close()
s.close()