#!/usr/bin/env python3
# coding=utf-8
import hmac
import json
import base64
import hashlib

import requests

iss_key = '9yp8zIZRUB54GFhNhUu96UOgeG3BYT4s'
secret_base64 = 'LdALChAs0Pm5dmT78fAss559JGw26umv'
proxy_api = 'http://localhost:8000/jwt-test'

if __name__ == '__main__':
    header = {
        'typ': 'JWT',
        'alg': 'HS256'
    }
    payload = {
        'iss': iss_key
    }
    header_base64 = base64.b64encode(json.dumps(header).encode('utf-8')).decode('utf-8')
    payload_base64 = base64.b64encode(json.dumps(payload).encode('utf-8')).decode('utf-8')

    msg = f'{header_base64}.{payload_base64}'.replace('=', '')

    if len(secret_base64) % 4 != 0:
        fill = (4 - len(secret_base64) % 4) * '='
        secret_base64 += fill

    secret = base64.b64decode(secret_base64)

    signature = hmac.new(secret, msg.encode('utf-8'), digestmod=hashlib.sha256).digest()
    signature_base64 = base64.b64encode(signature).decode().replace('=', '')

    auth_content = f'{msg}.{signature_base64}'
    print(auth_content)

    req_headers = {
        'Authorization': 'Bearer {}'.format(auth_content)
    }

    resp = requests.get(proxy_api, headers=req_headers)
    print(resp.status_code)
    print(resp.text)


