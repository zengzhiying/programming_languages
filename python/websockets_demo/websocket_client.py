#!/usr/bin/env python3
# coding=utf-8
import asyncio
import websockets
import time

async def hello():
    async with websockets.connect('ws://0.0.0.0:8765') as websocket:
        i = 1
        while True:
            message = "hello! %d" % i
            await websocket.send(message)
            print("发送: {}".format(message))
            # 获取返回
            greeting = await websocket.recv()
            print("返回: {}".format(greeting))
            i += 1
            time.sleep(0.06)

while True:
    try:
        asyncio.get_event_loop().run_until_complete(hello())
    except websockets.exceptions.ConnectionClosed as e:
        print(e)
        print("重新运行.")
        time.sleep(3)
    except Exception as e:
        print(e)
        print("重新连接.")
        time.sleep(3)
