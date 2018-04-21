#!C:\Python27\python.exe
# -*- coding:utf-8 -*-
import numpy as np
import cv2

# 创建画布
img = np.zeros((512, 512, 3), np.uint8)

def draw_line(img):
    cv2.line(img, (0,0), (511, 511), (255, 0, 0), 5)
    return img

def draw_rectangle(img):
    cv2.rectangle(img, (384, 0), (510, 128), (0, 255, 0), 3)
    return img

def draw_round(img):
    cv2.circle(img, (447, 63), 63, (0, 0, 255), -1)
    return img
def draw_ellipse(img):
    cv2.ellipse(img, (256, 256), (100, 50), 0, 0, 180, 255, -1)
    return img

def draw_polygon():
    pts = np.array([[10, 5], [20, 30], [70, 20], [50, 10]], np.int32)
    return pts.reshape((-1, 1, 2))

def draw_text(img):
    font = cv2.FONT_HERSHEY_SIMPLEX
    cv2.putText(img, 'OpenCV', (10, 500), font, 4, (255,255,255), 2)
    return img


img = draw_line(img)
img = draw_rectangle(img)
draw_round(img)
draw_ellipse(img)
draw_polygon()
draw_text(img)

cv2.namedWindow("draw_graph")
cv2.imshow("draw_graph", img)
cv2.waitKey(0)
cv2.destroyWindow("draw_graph")