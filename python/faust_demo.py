#!/usr/bin/env python3
# coding=utf-8
"""faust流计算示例程序"""
import json
from datetime import datetime

import pytz
import faust
from faust.web import Request, Response, View

print("init.")
user_basic = {
    'xm': '17261',
    'xiaoming': 'haha'
}
message_count = {
    'user': 0,
    'order': 0,
    'num': 0
}
app = faust.App(
    'demo-20190411',
    # broker: kafka://kafka1:9092;kafka2:9092
    # or ['kafka://kafka1:9092', 'kafka://kafka2:9092']
    broker='kafka://monchickey:9092',
    store='rocksdb://',
    topic_partitions=2,
    web_port=6066,
    web_bind="0.0.0.0")

class Order(faust.Record, serializer='json'):
    user_name: str
    amount: int

class Account(faust.Record):
    account_id: int
    account_name: str

user_topic = app.topic('userMessage', value_type=bytes, value_serializer='raw')
order_topic = app.topic('orderMessage', key_type=str, value_type=Order)
account_topic = app.topic('accountMessage', value_type=Account)

# rocksdb 统计
account_db = app.Table('account_groupby', default=int)

@app.agent(user_topic)
async def user_process(users):
    print("user process.")
    message_count['num'] += 1
    async for user in users:
        print(user)
        user_info = json.loads(user)
        message_count['user'] += 1

@app.agent(order_topic, concurrency=2)
async def order_process(orders):
    print("order process.")
    message_count['num'] += 1
    async for order_number, order in orders.items():
        print("order_number: {}, order: {}".format(order_number, order))
        if order.user_name in user_basic:
            # print(user_basic[order.user_name])
            pass
        message_count['order'] += 1

@app.agent(account_topic)
async def account_groupby(accounts: faust.Stream[Account]) -> None:
    print("account groupby.")
    # 注意这里的分组是基于kafka topic进行的, 会自动创建用于分区的topic
    # 务必保证创建topic的分区数和接收数据的topic分区数一致, 否则会报错
    async for account in accounts.group_by(Account.account_id):
        account_db[account.account_id] += 1
        print(account_db)

@app.timer(interval=3.0)
async def example_sender(app):
    """定时发送示例"""
    print("sending...")
    await order_process.send(
        key="order-01",
        value=Order(user_name='xiaoming', amount=3))

@app.page('/count/')
async def get_message_count(self, request):
    print("get count.")
    return self.json(message_count)

@app.page('/groupby/{account_id}')
async def get_account_group(web, request, account_id):
    return web.json({
        account_id: account_db[account_id],
    })

@app.page('/query')
class Query(View):
    count: int = 0
    async def get(self, request: Request) -> Response:
        param = 0
        if 'n' in request.query:
            param = request.query['n']
        return self.json({'count': self.count, 'param': param})
    # async def post(self, request: Request) -> Response:
    #     n: int = await request.post()
    #     self.count += int(n)
    #     return self.json({'count': self.count})

@app.crontab('0 10 * * *', timezone=pytz.timezone('Asia/Shanghai'), on_leader=False)
async def example_task():
    print("任务执行, time: {}".format(datetime.now().strftime('%Y-%m-%d %H:%M:%S')))

if __name__ == '__main__':
    app.main()
