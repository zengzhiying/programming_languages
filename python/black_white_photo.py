#!/usr/bin/env python3
# coding=utf-8
import cv2
import numpy as np

def is_black_white(img: np.ndarray) -> bool:
    if img.ndim == 2:
        return True
    if img.ndim != 3 or img.shape[-1] != 3:
        return False

    r, g, b = img[..., 0], img[..., 1], img[..., 2]

    r = r.astype(dtype=np.int16)
    g = g.astype(dtype=np.int16)
    b = b.astype(dtype=np.int16)

    offset = np.std(r - g) + np.std(r - b) + np.std(g - b)
    offset /= 3
    print(offset)
    if offset < 5:
        return True

    return False

if __name__ == '__main__':
    img1 = cv2.imread('color1.jpg')
    img2 = cv2.imread('color2.jpg')
    img3 = cv2.imread('color3.jpg')

    img4 = cv2.imread('gray1.jpg')
    img5 = cv2.imread('gray2.jpg')
    img6 = cv2.imread('gray3.jpg')

    img7 = cv2.imread('example1.jpg')
    img8 = cv2.imread('example2.jpg')
    img9 = cv2.imread('example3.jpg')

    print(is_black_white(img1))
    print(is_black_white(img2))
    print(is_black_white(img3))
    print(is_black_white(img4))
    print(is_black_white(img5))
    print(is_black_white(img6))

    print(is_black_white(img7))
    print(is_black_white(img8))
    print(is_black_white(img9))


