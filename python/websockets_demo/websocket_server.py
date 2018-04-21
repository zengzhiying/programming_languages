#!/usr/bin/env python3

import asyncio
import websockets

# 直接收一条消息处理 每次需要重新连接
async def hello(websocket, path):
    message = await websocket.recv()
    print("收到消息: {}".format(message))

    greeting = "Hello {}!".format(message)
    await websocket.send(greeting)
    print("返回响应: {}".format(greeting))
# 一直循环接收消息处理
async def handler(websocket, path):
    while True:
        message = await websocket.recv()
        print("msg: %s" % message)
        greeting = "Hello {}!".format(message)
        await websocket.send(greeting)

start_server = websockets.serve(handler, '0.0.0.0', 8765)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
