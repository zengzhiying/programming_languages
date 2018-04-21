#!C:\Python27\python.exe
# -*- coding:utf-8 -*-
import socket
import sys
remote_ip = "127.0.0.1"
port = 10188
message = "Hello Server"
try:
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((remote_ip, port))
    s.sendall(message)
except socket.error, msg:
    print msg[1]
    sys.exit()
else:
    print "send message ok..."

# 接收响应
reply = s.recv(4096)
print "----------server response-----------"
print reply

# 关闭socket
s.close()