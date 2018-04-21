#!/usr/bin/python
# coding=utf-8
import hashlib
''' 模仿java中根据字符串生成uuid的方法 但是由于初始md5字节数组不知道怎么创建 
    所以这个类不能使用 留作记录 '''
class JavaNameUUID(object):

    def __init__(self):
        self.most_sig_bits = 0L
        self.least_sig_bits = 0L

    # 创建md5
    def create_md5(self, src):
        m = hashlib.md5()
        m.update(src)
        return m.hexdigest()

    # 模仿java中UUID私有构造方法 传入list类型的data
    def _create_uuid(self, data):
        msb = 0L
        lsb = 0L
        for i in range(0, 8):
            msb = (msb << 8) | (self.str_to_int(data[i]) & 0xff)
        for i in range(8, 16):
            lsb = (lsb << 8) | (self.str_to_int(data[i]) & 0xff)
        self.most_sig_bits = msb
        self.least_sig_bits = lsb
        # print self.most_sig_bits
        # print self.least_sig_bits

    # 模仿java中的digits方法
    def _digits(self, val, digits):
        hi = 1L << (digits * 4)
        return self.int_to_hex(hi | (val & (hi - 1)))[1:]

    # 模仿java中的toString()
    def _to_string(self):
        print self._digits(self.most_sig_bits >> 32, 8)
        return (self._digits(self.most_sig_bits >> 32, 8) + \
            self._digits(self.most_sig_bits >> 16, 4) + \
            self._digits(self.most_sig_bits, 4) + \
            self._digits(self.least_sig_bits >> 48, 4) + \
            self._digits(self.least_sig_bits, 12))

    def create_nameuuid(self, data):
        md5_bytes = list(self.create_md5(data))
        # import binascii
        # md5_bytes = list(binascii.a2b_hex('ffffffffffffffe22a'))
        
        md5_bytes[6] = self.int_to_str(self.str_to_int(md5_bytes[6]) & 0x0f)
        md5_bytes[6] = self.int_to_str(self.str_to_int(md5_bytes[6]) | 0x30)
        md5_bytes[8] = self.int_to_str(self.str_to_int(md5_bytes[8]) & 0x3f)
        md5_bytes[8] = self.int_to_str(self.str_to_int(md5_bytes[8]) | 0x80)
        print md5_bytes
        self._create_uuid(md5_bytes)
        return self._to_string()

    def int_to_hex(self, integer_number):
        # int 一般为4个字节 最大值为2^31 - 1也就是0xfffffff 整数为214748367 超过这个数之后将会变为长整型 相应的16进制后面也会带个L
        # 此方法不安全因为不同机器上int长度不同
        # if integer_number <= 2147483647:
        #     return hex(integer_number)[2:]
        # else:
        #     hex_str = hex(integer_number)[2:]
        #     return hex_str[0:len(hex_str) - 1]
        hex_content = hex(integer_number)[2:]
        if 'L' in hex_content:
            return hex_content[:-1]
        return hex_content

    # 字符或字符串转成整数
    def str_to_int(self, str_data):
        return int(str_data.encode('hex'), 16)

    # 0 ~ 256 整数转换为字节
    def int_to_str(self, integer_number):
        return chr(integer_number)

if __name__ == '__main__':
    java_uuid = JavaNameUUID()
    print java_uuid.create_nameuuid("jinan_576")
    # print java_uuid.int_to_hex(3339L)
    s = ['s', 'a', 'b']
    s[2] = java_uuid.int_to_str(java_uuid.str_to_int(s[2]) & 0x0f)
    print s
