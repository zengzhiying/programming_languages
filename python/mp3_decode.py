# coding=utf-8
"""mp3 ID3V2解码示例代码
仅供测试, 正常建议使用第三方库, 推荐: mutagen和eyed3
"""
import os
import struct
import chardet

ID3_KEYS = {b"TALB", b"TPE1", b"TIT2"}


def decode_from_ID3V2(filename: str):
    """解析mp3 ID3信息 只支持ID3V2.x版本
    Args:
        filename: mp3文件路径
    Returns:
        dict, key如下:
        TALB: 专辑名
        TPE1: 作者
        TIT2: 歌曲标题名
        目前只有这3个key, 解码不出来key不存在
    """
    dec = {}
    if not os.path.isfile(filename):
        return dec
    f = open(filename, 'rb')
    tag = f.read(3).decode()
    if tag != 'ID3':
        return dec

    ver = f.read(2)
    print("version: 2.%d.%d" % (ver[0], ver[1]))

    flag = f.read(1)[0]
    flag >>= 6
    is_ext = False
    if flag & 1:
        # 扩展头
        is_ext = True

    id3_size = f.read(4)
    size = id3_size[0] << 21 | id3_size[1] << 14 | id3_size[2] << 7 | id3_size[3]

    if size == 0:
        return dec

    label = f.read(size)
    if is_ext:
        if label[4] >> 7:
            label = label[10:]
        else:
            label = label[6:]

    i = 0
    while i < len(label):
        frame_id = label[i:i + 4]
        i += 4
        frame_size = struct.unpack('>I', label[i:i + 4])[0]
        if frame_size == 0:
            print("框架大小为0, 错误! 原始数据:")
            print(label[i:])
            break

        i += 6
        if frame_id not in ID3_KEYS:
            i += frame_size
            continue

        try:
            frame_content = label[i:i + frame_size][1:].decode()
        except UnicodeDecodeError as e:
            print("utf-8 decode error: {}".format(e))
        else:
            dec[frame_id.decode()] = frame_content
            i += frame_size
            continue

        det = chardet.detect(label[i:i + frame_size][1:])
        if det['confidence'] > 0.9 and det['encoding']:
            try:
                frame_content = label[i:i + frame_size][1:].decode(det['encoding'])
            except UnicodeDecodeError as e:
                print(f"{det['encoding']} decode error: {e}")
            else:
                dec[frame_id.decode()] = frame_content

        i += frame_size

    # mp3 frame =============== example
    frame_sync = f.read(2)
    if frame_sync[0] == 255 and frame_sync[1] >> 4 == 15:
        mpeg_version = (frame_sync[1] & 15) >> 3
        if mpeg_version == 0:
            print(f"MPEG Version: 2")
        else:
            print(f"MPEG Version: 1")

        layer = (frame_sync[1] & 7) >> 1
        if layer == 0:
            print(f"Layer reserved")
        elif layer == 1:
            print(f"Layer 3")
        elif layer == 2:
            print(f"Layer 2")
        else:
            print(f"Layer 1")

        crc_pro = frame_sync[1] & 1
        if crc_pro:
            print("Not CRC protected")
        else:
            print("Protected by CRC.")

        header = f.read(1)[0]
        bit_rate = header >> 4
        # 这里需要查表 举个例子
        if mpeg_version == 1 and layer == 1:
            if bit_rate == 0b1001:
                print("bit rate: 128K")
            elif bit_rate == 0b1010:
                print("bit rate: 160K")

        # 采样频率
        frequency = (header & 7) >> 2
        # 需要查表
        if mpeg_version == 1:
            if frequency == 0:
                print("采样频率: 44100Hz")
            elif frequency == 1:
                print("采样频率: 48000Hz")
            elif frequency == 2:
                print("采样频率: 32000Hz")
            else:
                print("采样频率: reserv.")

        padding = (header & 3) >> 1
        if padding:
            print("有填充")
        else:
            print("无填充")

        # 数据帧大小: 144 * bit rate / 采样频率 + (有填充位 + 1否则不加) 详细参考标准文档

    f.close()

    return dec


if __name__ == '__main__':
    filename = "xiatian.mp3"
    print(decode_from_ID3V2(filename))

    
