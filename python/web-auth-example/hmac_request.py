#!/usr/bin/env python3
# coding=utf-8
import base64
import hmac

from hashlib import sha256
from datetime import datetime

import requests


username = "qiuyue123"
secret = b"805b540cbbdf9dc0"
url = "http://localhost:8000/hmac-test"

def get_hmac_sha256(key, msg=None):
    return hmac.new(key, msg, digestmod=sha256).digest()

if __name__ == '__main__':
    utc_time = datetime.utcnow().strftime('%a, %d %b %Y %H:%M:%S GMT')
    date_line = "date: {}".format(utc_time)
    signature = base64.b64encode(get_hmac_sha256(secret, date_line.encode('utf-8'))).decode('utf-8')
    req_headers = {
        'Date': utc_time,
        'Authorization': 'hmac username="{}", algorithm="hmac-sha256", headers="date", signature="{}"'.format(username, signature)
    }
    print(req_headers)
    r = requests.get(url, headers=req_headers)
    print(r.status_code)
    print(r.text)

