#!/usr/bin/env python3
# coding=utf-8
import time
import os
import asyncio

import tornado.ioloop
import tornado.web
from tornado.concurrent import run_on_executor
from tornado.concurrent import Future
from concurrent.futures import ThreadPoolExecutor
from tornado import gen

class MainHandler(tornado.web.RequestHandler):
    def get(self):
        print("get.")
        self.write("Hello, world")
class TestHandler(tornado.web.RequestHandler):
    def initialize(self, p):
        self.param = p
    def get(self, req_id):
        self.write("This is request id: {}, param: {}".format(req_id, self.param))
        print(self.get_arguments('name'))
        print(self.get_argument('name', default='1'))
        print(self.get_query_arguments('name'))
        # empty
        # print(self.get_body_arguments('name'))
    def post(self, req_id):
        self.set_header("Content-Type", "text/html")
        print(self.get_body_arguments('name'))
        print(self.get_body_argument('name', default='1'))
        print(self.get_arguments('name'))
        # empty
        #print(self.get_query_arguments('name'))
        self.write(req_id)

class ThreadPoolTestHandler(tornado.web.RequestHandler):
    _executor_pool = ThreadPoolExecutor(30)
    @tornado.web.asynchronous
    @gen.coroutine
    def get(self):
        #print(self.request.headers)
        print("start. {}".format(os.getpid()))
        rs = yield self.do_action()
        print("end")
        self.write("end. {}".format(rs))
        #self.finish()
    @run_on_executor(executor='_executor_pool')
    def do_action(self):
        time.sleep(10)
        return 1

from tornado.httpclient import AsyncHTTPClient
class AsyncTestHandler(tornado.web.RequestHandler):
    # new python >= 3.5
    async def get(self):
        http_client = AsyncHTTPClient()
        print("request.")
        response = await http_client.fetch('http://www.baidu.com')
        self.write(response.body)

    # old python
    # @gen.coroutine
    # def get(self):
    #     print("request.")
    #     http_client = AsyncHTTPClient()
    #     response = yield http_client.fetch('http://www.163.com')
    #     # raise gen.Return(response.body)
    #     self.write(response.body)

class UploadFileHandler(tornado.web.RequestHandler):
    """接收表单文件上传"""
    _executor_pool = ThreadPoolExecutor(30)
    def get(self):
        self.write("")
        
    @run_on_executor(executor='_executor_pool')
    def post(self):
        print("request method post.")

        # 示例默认只取第一张
        image_file = self.request.files['image'][0]['body']
        name = self.get_body_argument('name')

        # 保存接收的图片
        with open('upload.jpg', 'wb') as f:
            f.write(image_file)

        print('name: {}'.format(name))

        rep = {'status': 0, "message": "success"}
        self.write(json.dumps(rep))


def make_app():
    return tornado.web.Application([
        (r"/", MainHandler),
        tornado.web.url(r"/req/([0-9]+)", TestHandler, dict(p='haha'), name='kaka'),
        (r"/threads", ThreadPoolTestHandler),
        (r"/async", AsyncTestHandler),
        (r"/form_upload", UploadFileHandler)
    ])

if __name__ == "__main__":
    # 1. ioloop
    #app = make_app()
    #app.listen(8888)
    #tornado.ioloop.IOLoop.current().start()
    # 2. httpserver
    app = make_app()
    server = tornado.httpserver.HTTPServer(app)
    server.bind(8888, address='192.168.112.128')
    server.start(4)
    tornado.ioloop.IOLoop.current().start()
    
