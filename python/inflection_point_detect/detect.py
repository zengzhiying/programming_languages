#!/usr/bin/python3
# coding=utf-8
"""使用opencv内置的图像中拐点检测
只能检测背景单一, 轮廓清晰并且相对规则的图形.
"""
import cv2
import numpy as np

image_name = 'test.jpg'
output_image_name = 'test1.jpg'

if __name__ == '__main__':
    image = cv2.imread(image_name)
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    corners = cv2.goodFeaturesToTrack(gray, 25, 0.01, 10)
    corners = np.int0(corners)
    for i in corners:
        x, y = i.ravel()
        cv2.circle(image, (x, y), 7, 255, -1)
    cv2.imwrite(output_image_name, image)
